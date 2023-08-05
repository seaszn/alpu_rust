use ethers::prelude::*;
use serde::Deserialize;
use std::slice::Iter;

mod protocol_handler;

#[derive(Clone, Debug, Copy, Deserialize, PartialEq, Eq, Hash)]
pub enum Protocol {
    UniswapV2,
    StableSwap,
}
const PROTOCOL_ITER: [Protocol; 2] = [Protocol::UniswapV2, Protocol::StableSwap ];

impl Protocol {
    pub fn iterator() -> Iter<'static, Protocol>{
        PROTOCOL_ITER.iter()
    }
}

#[derive(Debug, Deserialize)]
pub struct Exchange {
    pub factory_address: Address,
    pub min_liquidity: i32,
    pub protocol: Protocol,
    pub base_fee: i32,
    pub stable_fee: Option<i32>,
}
