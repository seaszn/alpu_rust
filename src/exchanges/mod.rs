use ethers::abi::Address;
use serde::Deserialize;
use crate::types::Market;

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

pub fn get_exchange_markets<'g>(exchange: Exchange) -> Vec<&'g Market<'g>> {
    if exchange.protocol == Protocol::UniswapV2 {
        return uniswap_v2::get_markets::<'g>(exchange);
    }
    else if exchange.protocol == Protocol::StableSwap{
        return stable_swap::get_markets(exchange);
    }

    return vec![]
}
