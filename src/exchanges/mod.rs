use std::sync::Arc;

use crate::{
    env::types::{RuntimeClient, UniswapQueryContract},
    networks::Network,
    types::Market,
};
use ethers::prelude::*;
use serde::Deserialize;

mod stable_swap;
mod uniswap_v2;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Protocol {
    UniswapV2,
    StableSwap,
}

#[derive(Debug, Deserialize)]
pub struct Exchange {
    pub factory_address: Address,
    pub min_liquidity: i32,
    pub protocol: Protocol,
    pub base_fee: i32,
    pub stable_fee: Option<i32>,
}

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
