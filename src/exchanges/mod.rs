use ethers::abi::Address;
use serde::Deserialize;
use crate::types::Market;
use ethers::prelude::*;
use ethers::prelude::k256::ecdsa::SigningKey;


use crate::environment::runtime::*;

mod uniswap_v2;
mod stable_swap;

#[derive(Debug, Deserialize, PartialEq)]
#[derive(Clone)]
pub enum Protocol {
    UniswapV2,
    StableSwap,
}

#[derive(Debug, Deserialize)]
#[derive(Clone)]
pub struct Exchange {
    pub factory_address: Address,
    pub min_liquidity: i32,
    pub protocol: Protocol,
    pub base_fee: i32,
    pub stable_fee: Option<i32>,
}

pub async fn get_exchange_markets(exchange: &Exchange, runtime_cache: &Cache) -> Vec<Market> {
    if exchange.protocol == Protocol::UniswapV2 {
        return uniswap_v2::get_markets(exchange, runtime_cache).await;
    }
    else if exchange.protocol == Protocol::StableSwap{
        // return stable_swap::get_markets(exchange);
    }

    return vec![]
}
