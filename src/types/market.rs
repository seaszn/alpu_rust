use std::sync::Arc;

use ethers::types::{Address, H160};

use crate::{env, exchanges::types::Protocol};

use super::Token;

#[derive(Clone, Debug)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [Arc<Token>; 2],
    pub fee: i32,
    pub stable: bool,
    pub protocol: Protocol,
}

pub fn from_address(address: &H160) -> Option<Arc<Market>> {
    for market in &env::RUNTIME_CACHE.markets {
        if market.contract_address.0 == address.0 {
            return Some(market.clone());
        }
    }

    return None;
}
