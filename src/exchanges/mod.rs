use std::{sync::Arc, vec};

use crate::{
    env::types::{RuntimeClient, UniswapQueryContract},
    exchanges::types::Protocol,
    networks::Network,
    types::{market::Market, TransactionLog},
};

use self::types::{Exchange, Swap};

mod stable_swap;
pub mod types;
mod uniswap_v2;

pub async fn get_exchange_markets(
    exchange: &Exchange,
    network: Arc<Network>,
    client: RuntimeClient,
    uniswap_query: UniswapQueryContract,
) -> Vec<Arc<Market>> {
    println!("Loading markets from factory {}", exchange.factory_address);

    if exchange.protocol == Protocol::UniswapV2 {
        return uniswap_v2::get_markets(exchange, network, client, uniswap_query).await;
    } else if exchange.protocol == Protocol::StableSwap {
        // return stable_swap::get_markets(exchange);
    }

    return vec![];
}

pub fn parse_exchange_logs(logs: Vec<TransactionLog>) -> Vec<Swap> {
    let mut result: Vec<Swap> = vec![];

    for log in logs{
        if log.protocol == Protocol::UniswapV2 {
            result.append(uniswap_v2::parse_logs(&[log.raw]).as_mut());
        } else if log.protocol == Protocol::StableSwap {
        }
    }

    return result;
}
