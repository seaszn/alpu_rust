use std::{collections::HashMap, io::Error, sync::Arc, vec};

use ethers::{
    abi::Address, prelude::k256::elliptic_curve::bigint::modular::runtime_mod, types::H160,
};
use rayon::{
    collections::hash_map,
    prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
};

use crate::{
    env::{
        types::{RuntimeClient, UniswapQueryContract},
        RuntimeCache,
    },
    exchanges::types::Protocol,
    handlers::types::swap::BalanceChange,
    networks::Network,
    types::{market::Market, ReserveTable, TransactionLog},
};

use self::types::Exchange;

mod stable_swap;
pub mod types;
mod uniswap_v2;

pub async fn get_exchange_markets(
    network: &Network,
    runtime_cache: &RuntimeCache,
) -> Result<Vec<Arc<Market>>, Error> {
    let mut result: Vec<Arc<Market>> = vec![];

    for exchange in &network.exchanges {
        if exchange.protocol == Protocol::UniswapV2 {
            if let Ok(mut response) =
                uniswap_v2::get_markets(exchange, network.clone(), runtime_cache.clone()).await
            {
                result.append(&mut response);
            };
        } else if exchange.protocol == Protocol::StableSwap {
            // return stable_swap::get_markets(exchange);
        }
    };

    return Ok(result);
}

pub fn parse_balance_changes(logs: Vec<TransactionLog>) -> Vec<BalanceChange> {
    let mut result: Vec<BalanceChange> = vec![];

    // Uniswap V2
    result.append(&mut uniswap_v2::parse_balance_changes(
        &logs
            .clone()
            .into_par_iter()
            .filter(|x| x.protocol == Protocol::UniswapV2)
            .collect(),
    ));

    return result;
}

pub async fn get_market_reserves(markets: &Vec<Arc<Market>>, runtime_cache: &RuntimeCache) {
    let addressess: Vec<(Address, Protocol)> = markets
        .par_iter()
        .map(|x| (x.contract_address, x.protocol))
        .collect();

    // Uniswap V2
    let markets: ReserveTable = uniswap_v2::get_market_reserves(
        &addressess
            .clone()
            .into_par_iter()
            .filter(|x| x.1 == Protocol::UniswapV2 || x.1 == Protocol::StableSwap)
            .collect::<Vec<(H160, Protocol)>>()
            .par_iter()
            .map(|x| x.0)
            .collect(),
        runtime_cache,
    )
    .await;
}
