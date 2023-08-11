use std::{io::Error, vec};

use crate::{
    env::{RuntimeCache, RuntimeConfig, EXECUTE_TX_BUNDLE_FUNCTION},
    exchanges::types::Protocol,
    networks::Network,
    types::{
        market::Market, BalanceChange, MarketState, OrganizedList, SwapLog, Token, TransactionLog,
    },
};
use ethers::{
    prelude::AbiError,
    types::{Bytes, H160, U256},
};
use tokio::task::JoinSet;

pub use self::stable_swap::StableSwapMarketState;
use self::types::Exchange;
pub use self::uniswap_v2::UniswapV2MarketState;

mod stable_swap;
pub mod types;
pub mod uniswap_v2;

pub use stable_swap::get_k;

#[inline(always)]
pub async fn get_exchange_markets(
    network: &'static Network,
    runtime_cache: &RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> Result<Vec<Market>, Error> {
    let mut result: Vec<Market> = vec![];

    for exchange in &network.exchanges {
        match exchange.protocol {
            Protocol::UniswapV2 => {
                if let Ok(mut response) =
                    uniswap_v2::get_markets(exchange, network, runtime_cache, runtime_config).await
                {
                    println!(
                        "Loaded {} markets from {}...",
                        response.len(),
                        exchange.factory_address
                    );
                    result.append(&mut response);
                };
            }
            Protocol::StableSwap => {
                if let Ok(mut response) =
                    stable_swap::get_markets(exchange, network, runtime_cache, runtime_config).await
                {
                    println!(
                        "Loaded {} markets from {}...",
                        response.len(),
                        exchange.factory_address
                    );
                    result.append(&mut response);
                };
            }
        }
    }

    return Ok(result);
}

pub fn init_exchange_handlers() {
    let _ = &EXECUTE_TX_BUNDLE_FUNCTION.name;
    uniswap_v2::init_handler();
    stable_swap::init_handler();
}

#[inline(always)]
pub fn parse_balance_changes(
    logs: &Vec<TransactionLog>,
    runtime_cache: &'static RuntimeCache,
) -> Vec<BalanceChange> {
    let mut result: Vec<BalanceChange> = vec![];

    // Uniswap V2
    result.append(&mut uniswap_v2::parse_balance_changes(
        logs.iter()
            .filter(|x| x.protocol == Protocol::UniswapV2)
            .collect(),
        runtime_cache,
    ));

    // stable swap
    result.append(&mut stable_swap::parse_balance_changes(
        logs.iter()
            .filter(|x| x.protocol == Protocol::StableSwap)
            .collect(),
        runtime_cache,
    ));

    return result;
}

#[inline(always)]
pub async fn get_market_reserves(
    markets: &'static OrganizedList<Market>,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> OrganizedList<MarketState> {
    let mut join_set: JoinSet<OrganizedList<MarketState>> = JoinSet::new();
    let mut result: OrganizedList<MarketState> = OrganizedList::new();

    for protocol in Protocol::iterator() {
        join_set.spawn(async move {
            let markets = markets.filter(|x| x.value.protocol == *protocol);
            match protocol {
                Protocol::UniswapV2 => {
                    return uniswap_v2::get_market_reserves(markets, runtime_cache, runtime_config)
                        .await;
                }
                Protocol::StableSwap => {
                    return stable_swap::get_market_reserves(
                        markets,
                        runtime_cache,
                        runtime_config,
                    )
                    .await;
                }
            }
        });
    }

    while let Some(Ok(mut call_result)) = join_set.join_next().await {
        result.append_unsorted(&mut call_result);
    }

    result.sort();
    return result;
}

#[inline(always)]
pub fn calculate_amount_out(
    market_state: &MarketState,
    input_amount: &U256,
    market: &Market,
    token_in: &'static Token,
) -> U256 {
    return match market_state {
        MarketState::UniswapV2(reserves) => uniswap_v2::calculate_amount_out(
            market,
            &sort_reserves(reserves, market, token_in),
            &input_amount,
        ),
        MarketState::StableSwap(reserves) => {
            stable_swap::calculate_amount_out(
                market,
                reserves,
                &input_amount,
                token_in
            )
        }
    };
}

#[inline(always)]
pub fn sort_reserves(
    reserves: &(U256, U256),
    market: &Market,
    token_in: &'static Token,
) -> (U256, U256) {
    if token_in.eq(market.tokens[1]) {
        return (reserves.1, reserves.0);
    }

    return *reserves;
}

#[inline(always)]
pub fn populate_swap(swap_log: &SwapLog, recipient: &H160) -> Result<Bytes, AbiError> {
    match swap_log.market.value.protocol {
        Protocol::StableSwap => {
            return stable_swap::populate_swap(&swap_log, recipient);
        }
        Protocol::UniswapV2 => {
            return uniswap_v2::populate_swap(&swap_log, recipient);
        }
    }
}

#[inline(always)]
pub fn calculate_circ_liquidity_step(
    market: &Market,
    reserves: (U256, U256),
    previous_reserves: &(U256, U256),
    token_in: &'static Token,
) -> (U256, U256) {
    return match market.protocol {
        Protocol::UniswapV2 => uniswap_v2::calc_circ_liq_step(previous_reserves, reserves, &market),
        Protocol::StableSwap => {
            if market.stable == true {
                stable_swap::calc_circ_liq_step(previous_reserves, reserves, &market, token_in)
                // unisw/ap_v2::calc_circ_liq_step(previous_reserves, reserves, &market)
            } else {
                uniswap_v2::calc_circ_liq_step(previous_reserves, reserves, &market)
            }
        }
    };
}
