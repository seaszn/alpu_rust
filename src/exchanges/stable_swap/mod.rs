use tokio::task::JoinSet;
pub use types::StableSwapMarketState;

use crate::{
    types::{MarketState, OrgValue, OrganizedList},
    utils::parse::dec_to_u256,
};
use ethers::{
    abi::{AbiParser, Function},
    prelude::*,
    utils::{parse_units, WEI_IN_ETHER},
};
use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::io::{Error, ErrorKind};

use super::WEI_IN_ETHER_U512;
use ethers::types::U256;

use self::types::{StableSwapFactory, StableSwapPair, SwapCall, SwapFilter};

use super::{
    uniswap_v2::{self},
    Exchange,
};
use crate::{
    env::{RuntimeCache, RuntimeConfig},
    networks::Network,
    types::{market::Market, BalanceChange, SwapLog, Token, TransactionLog},
};

mod types;
lazy_static! {
    static ref SWAP_METHOD: Function = AbiParser::default()
        .parse_function("swap(uint256,uint256,address,bytes)")
        .unwrap();
}

#[inline(always)]
pub async fn get_market_reserves(
    markets: Vec<&'static OrgValue<Market>>,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &RuntimeConfig,
) -> OrganizedList<MarketState> {
    let mut join_set: JoinSet<Vec<(usize, MarketState)>> = JoinSet::new();

    for market_chunk in &markets.into_iter().chunks(runtime_config.small_chunk_size) {
        let market_values = market_chunk.collect_vec();
        let addressess: Vec<H160> = market_values
            .iter()
            .map(|x| x.value.contract_address)
            .collect();

        join_set.spawn(async move {
            match runtime_cache
                .uniswap_query
                .get_stable_swap_states(addressess.clone())
                .await
            {
                Ok(response) => {
                    let mut result: Vec<(usize, MarketState)> = Vec::new();

                    for i in 0..market_values.len() {
                        let raw_reserves: [U256; 3] = response[i];
                        result.push((
                            market_values[i].id,
                            MarketState::StableSwap((
                                raw_reserves[0].into(),
                                raw_reserves[1].into(),
                            )),
                        ));
                    }

                    return result;
                }
                Err(_) => {
                    return vec![];
                }
            }
        });
    }

    let mut res: OrganizedList<MarketState> = OrganizedList::new();
    while let Some(Ok(result)) = join_set.join_next().await {
        for i in 0..result.len() {
            res.add_pair(OrgValue {
                id: result[i].0,
                value: result[i].1,
            });
        }
    }

    res.sort();
    return res;
}

#[inline(always)]
pub fn init_handler() {
    let _ = { &SWAP_METHOD.name };
}

#[inline(always)]
pub fn populate_swap(swap: &SwapLog, to: &H160) -> Result<Bytes, AbiError> {
    return ethers::contract::encode_function_data::<types::SwapCall>(
        &SWAP_METHOD,
        SwapCall {
            amount_0_out: swap.amount_0_out.as_u128().into(),
            amount_1_out: swap.amount_1_out.as_u128().into(),
            to: *to,
            data: Bytes::new(),
        },
    );
}

#[inline(always)]
pub async fn get_markets(
    exchange: &'static Exchange,
    network: &'static Network,
    runtime_cache: &RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> Result<Vec<Market>, Error> {
    let factory_contract: self::types::StableSwapFactoryContract =
        StableSwapFactory::new(exchange.factory_address, runtime_cache.client.clone());
    let mut result: Vec<Market> = vec![];

    if let Ok(market_count) = factory_contract.all_pairs_length().await {
        let total_market_count: u128 = market_count.as_u128();

        for chunk in &(0..total_market_count)
            .into_iter()
            .chunks(runtime_config.large_chunk_size)
        {
            let (start, stop) = {
                let chunk = chunk.collect_vec();
                (U256::from(chunk[0]), U256::from(*chunk.last().unwrap()))
            };

            match runtime_cache
                .uniswap_query
                .get_stable_swap_markets(exchange.factory_address, start, stop)
                .await
            {
                Ok(response) => {
                    for element in response {
                        let token_0 = network
                            .tokens
                            .iter()
                            .find(|s| s.contract_address.0 == element[0].0);
                        let token_1 = network
                            .tokens
                            .iter()
                            .find(|s| s.contract_address.0 == element[1].0);

                        let stable = !H160::zero().eq(&element[2]);

                        if token_0.is_some() && token_1.is_some() {
                            let pair_contract =
                                StableSwapPair::new(element[3], runtime_cache.client.clone());

                            if let Ok((reserve_0, reserve_1, _)) =
                                pair_contract.get_reserves().await
                            {
                                let token_0_instance = token_0.unwrap();
                                let token_1_instance = token_1.unwrap();

                                let min_reserve_0 = dec_to_u256(
                                    &runtime_config.min_market_reserves,
                                    token_0_instance.decimals,
                                );

                                let min_reserve_1 = dec_to_u256(
                                    &runtime_config.min_market_reserves,
                                    token_1_instance.decimals,
                                );

                                if min_reserve_0.lt(&U256::from(reserve_0))
                                    && min_reserve_1.lt(&U256::from(reserve_1))
                                {
                                    let fee = {
                                        if stable {
                                            exchange.stable_fee.unwrap()
                                        } else {
                                            exchange.base_fee
                                        }
                                    };
                                    result.push(Market::new(
                                        element[3],
                                        [token_0_instance, token_1_instance],
                                        fee,
                                        stable,
                                        exchange.protocol,
                                    ));
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("{:#?}", err);
                    return Err(Error::new(ErrorKind::ConnectionRefused, err));
                }
            }
        }
    }

    return Ok(result);
}

#[inline(always)]
pub fn parse_balance_changes(
    logs: Vec<&TransactionLog>,
    runtime_cache: &'static RuntimeCache,
) -> Vec<BalanceChange> {
    if logs.len() > 1 {
        let mut stacked_balance_changes: Vec<Vec<BalanceChange>> = vec![];
        logs.into_par_iter()
            .map(move |transaction_log| -> Vec<BalanceChange> {
                if let Ok(filters) =
                    ethers::contract::decode_logs::<SwapFilter>(&[transaction_log.raw.clone()])
                {
                    let mut swap_events: Vec<BalanceChange> = vec![];

                    for swap in filters {
                        swap_events.push(BalanceChange {
                            market: Market::from_address(&transaction_log.address, runtime_cache)
                                .unwrap(),
                            amount_0_in: swap.amount_0_in,
                            amount_1_in: swap.amount_1_in,
                            amount_0_out: swap.amount_0_out,
                            amount_1_out: swap.amount_1_out,
                        });
                    }

                    return swap_events;
                }

                return vec![];
            })
            .collect_into_vec(&mut stacked_balance_changes);

        let mut result: Vec<BalanceChange> = vec![];
        for mut change_stack in stacked_balance_changes {
            if change_stack.len() > 0 {
                result.append(&mut change_stack);
            }
        }

        return result;
    }

    return vec![];
}

#[inline(always)]
pub fn calculate_amount_out(
    market: &Market,
    reserves: &StableSwapMarketState,
    input_amount: &U256,
    token_in: &'static Token,
) -> U256 {
    if market.stable {
        if token_in.eq(market.tokens[0]) {
            let (fee_mul, mul) = market.get_fee_data();
            let token_in_pow: U256 = parse_units(1, market.tokens[0].decimals).unwrap().into();
            let token_out_pow: U256 = parse_units(1, market.tokens[1].decimals).unwrap().into();

            let amount_in_with_fee = (input_amount * fee_mul) / mul;
            let amount_in_formatted = amount_in_with_fee * WEI_IN_ETHER / token_in_pow;

            let reserve_0 = reserves.0 * WEI_IN_ETHER / token_in_pow;
            let reserve_1 = reserves.1 * WEI_IN_ETHER / token_out_pow;

            let a = (reserve_0 * reserve_1) / WEI_IN_ETHER;
            let b = (reserve_0 * reserve_1) / WEI_IN_ETHER + (reserve_1 * reserve_1) / WEI_IN_ETHER;
            let xy = a * b / WEI_IN_ETHER;

            let y = reserve_1 - get_y(amount_in_formatted + reserve_0, xy, reserve_1);
            return y * token_out_pow / WEI_IN_ETHER;
        } else {
            return calculate_amount_out(market, &(reserves.1, reserves.0), input_amount, token_in);
        }
    } else {
        return uniswap_v2::calculate_amount_out(market, reserves, input_amount, token_in);
    }
}

#[inline(always)]
fn get_y(x0: U256, xy: U256, mut y: U256) -> U256 {
    for _ in 0..255 {
        let prev_y = y;
        let liquidity = get_liquidity(x0, y);

        if liquidity < xy {
            y = y - ((xy - liquidity) * WEI_IN_ETHER / get_denominator(x0, y))
        } else {
            y = y - ((liquidity - xy) * WEI_IN_ETHER / get_denominator(x0, y))
        }

        if y > prev_y {
            if (y - prev_y).le(&U256::one()) {
                return y;
            }
        } else {
            if (prev_y - y).le(&U256::one()) {
                return y;
            }
        }
    }

    return y;
}

#[inline(always)]
fn get_liquidity(x0: U256, y: U256) -> U256 {
    return x0 * (y * y / WEI_IN_ETHER * y / WEI_IN_ETHER) / WEI_IN_ETHER
        + (x0 * x0 / WEI_IN_ETHER * x0 / WEI_IN_ETHER) * y / WEI_IN_ETHER;
}

#[inline(always)]
pub fn get_liquidity_u512(x0: &U512, y: &U512) -> U512 {
    return x0 * (y * y / WEI_IN_ETHER * y / WEI_IN_ETHER) / WEI_IN_ETHER
        + (x0 * x0 / WEI_IN_ETHER * x0 / WEI_IN_ETHER) * y / WEI_IN_ETHER;
}

#[inline(always)]
fn get_denominator(x0: U256, y: U256) -> U256 {
    return x0 * 3 * (y * y / WEI_IN_ETHER) / WEI_IN_ETHER
        + x0 * x0 / WEI_IN_ETHER * x0 / WEI_IN_ETHER;
}

#[inline(always)]
fn get_denominator_u512(x0: &U512, y: &U512) -> U512 {
    return x0 * 3 * (y * y / WEI_IN_ETHER) / WEI_IN_ETHER
        + x0 * x0 / WEI_IN_ETHER * x0 / WEI_IN_ETHER;
}

#[inline(always)]
pub fn calc_circ_liq_step(
    previous: &(U512, U512),
    reserves: (&U512, &U512),
    market: &Market,
    token_in: &'static Token,
) -> (U512, U512) {
    let (fee_multiplier, mul): (U512, U512) = {
        let temp = market.get_fee_data();
        (temp.0.into(), temp.1.into())
    };

    let sorted_tokens: [&Token; 2] = {
        if token_in.eq(market.tokens[0]) {
            market.tokens
        } else {
            [market.tokens[1], market.tokens[0]]
        }
    };

    let token_in_pow: U256 = parse_units(1, sorted_tokens[0].decimals).unwrap().into();
    let token_out_pow: U256 = parse_units(1, sorted_tokens[1].decimals).unwrap().into();

    let amount_in_with_fee = previous.1 * fee_multiplier / mul;
    let amount_in_formatted = amount_in_with_fee * WEI_IN_ETHER_U512 / token_in_pow;

    let reserve_0 = reserves.0 * WEI_IN_ETHER_U512 / token_in_pow;
    let reserve_1 = reserves.1 * WEI_IN_ETHER_U512 / token_out_pow;

    // let denominator = amount_in_with_fee + reserves.0;
    let denominator = get_denominator_u512(&(&amount_in_formatted + reserve_0), &reserve_1);
    // let l_0 = (previous.0 * reserves.0) / denominator; // (xy / d)
    let l_0 = get_liquidity_u512(&previous.0, &reserve_0) / denominator; //. mul(token_in_pow.into()) / *POW_18_U512;
    let l_1 = get_liquidity_u512(&amount_in_formatted, &reserve_1) / denominator;

    return (
        l_0 * U512::from(token_in_pow) / WEI_IN_ETHER_U512,
        l_1 * U512::from(token_out_pow) / WEI_IN_ETHER_U512,
    );
}
