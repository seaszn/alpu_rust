use ethers::prelude::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Copy)]
pub struct Swap {
    pub sender: H160,
    pub amount_0_in: U256,
    pub amount_1_in: U256,
    pub amount_0_out: U256,
    pub amount_1_out: U256,
    pub to: H160,
}