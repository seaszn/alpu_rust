use ethers::prelude::*;
use serde::Deserialize;

mod protocol_handler;

// pub use protocol_handler::ProtocolHandler;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
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
