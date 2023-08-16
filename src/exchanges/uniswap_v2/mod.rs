use crate::types::MarketState;
use ethers::{
    abi::{AbiParser, Function},
    prelude::*,
    utils::parse_units,
};
use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::io::Error;
use tokio::task::JoinSet;

use self::types::{
    uniswap_v2_pair::{self, SwapCall, UniswapV2Pair},
    UniswapV2Factory, UniswapV2FactoryContract,
};
use ethers::types::U256;

use super::Exchange;
use crate::{
    env::{RuntimeCache, RuntimeConfig},
    networks::Network,
    types::{
        market::Market, BalanceChange, OrgValue, OrganizedList, SwapLog, TransactionLog,
    },
};

mod types;
pub use types::UniswapV2MarketState;

lazy_static! {
    static ref SWAP_METHOD: Function = AbiParser::default()
        .parse_function("swap(uint256,uint256,address,bytes)")
        .unwrap();
}

#[inline(always)]
pub fn init_handler() {
    let _ = { &SWAP_METHOD.name };
}

#[inline(always)]
pub fn populate_swap(swap: &SwapLog, to: &H160) -> Result<Bytes, AbiError> {
    return ethers::contract::encode_function_data::<uniswap_v2_pair::SwapCall>(
        &SWAP_METHOD,
        SwapCall {
            amount_0_out: swap.amount_0_out,
            amount_1_out: swap.amount_1_out,
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
    let factory_contract: UniswapV2FactoryContract =
        UniswapV2Factory::new(exchange.factory_address, runtime_cache.client.clone());
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
                .get_uniswap_v2_markets(exchange.factory_address, start, stop)
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

                        if token_0.is_some() && token_1.is_some() {
                            let pair_contract =
                                UniswapV2Pair::new(element[2], runtime_cache.client.clone());

                            if let Ok((reserve_0, reserve_1, _)) =
                                pair_contract.get_reserves().await
                            {
                                let token_0_instance = token_0.unwrap();
                                let token_1_instance = token_1.unwrap();

                                let min_reserve_0: U256 = parse_units(
                                    &runtime_config.min_market_reserves,
                                    token_0_instance.decimals,
                                )
                                .unwrap()
                                .into();

                                let min_reserve_1: U256 = parse_units(
                                    &runtime_config.min_market_reserves,
                                    token_1_instance.decimals,
                                )
                                .unwrap()
                                .into();

                                if min_reserve_0.lt(&U256::from(reserve_0))
                                    && min_reserve_1.lt(&U256::from(reserve_1))
                                {
                                    result.push(Market::new(
                                        element[2],
                                        [token_0.unwrap(), token_1.unwrap()],
                                        exchange.base_fee,
                                        false,
                                        exchange.protocol,
                                    ));
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // println!("failed v2 {:#?}", err);
                    // return Err(Error::new(ErrorKind::ConnectionRefused, err));
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
                    ethers::contract::decode_logs::<uniswap_v2_pair::SwapFilter>(&[transaction_log
                        .raw
                        .clone()])
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
                .get_uniswap_v2_states(addressess.clone())
                .await
            {
                Ok(response) => {
                    let mut result: Vec<(usize, MarketState)> = Vec::new();

                    for i in 0..market_values.len() {
                        let raw_reserves: [u128; 3] = response[i];
                        result.push((
                            market_values[i].id,
                            MarketState::UniswapV2((
                                raw_reserves[0].into(),
                                raw_reserves[1].into(),
                            )),
                        ))
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
        // println!("{}", result.len());
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
pub fn calculate_amount_out(
    market: &Market,
    (reserve_in, reserve_out): &(U256, U256),
    input_amount: &U256,
) -> Option<U256> {
    // println!("uniswap");

    let (fee_multiplier, multiplier) = market.get_fee_data();
    let amount_in_with_fee = input_amount * fee_multiplier / multiplier;

    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in + amount_in_with_fee;

    return Some(numerator / denominator);
}

#[inline(always)]
pub fn calc_circ_liq_step(
    previous: &(U256, U256),
    reserves: (U256, U256),
    market: &Market,
) -> (U256, U256) {
    let (fee_multiplier, mul) = market.get_fee_data();

    let amount_in_with_fee = previous.1 * fee_multiplier / mul;
    let denominator = amount_in_with_fee + reserves.0;

    let l_0 = (previous.0 * reserves.0) / denominator;
    let l_1 = (amount_in_with_fee * reserves.1) / denominator;

    return (l_0, l_1);
}

// let delta = amount_in_with_fee + reserves.0;
// let l_0 = (res0 * reserves.0) / delta;
// let l_1 = ((amount_in_with_fee * reserves.1) / mul) / delta;
