use std::sync::Arc;

use ethers::prelude::*;
use serde::Deserialize;

use crate::env;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

impl Token {
    pub fn from_address(address: &NameOrAddress) -> Option<Arc<Token>> {
        for market in &env::RUNTIME_NETWORK.tokens {
            if market.contract_address.0 == address.as_address().unwrap().0 {
                return Some(market.clone());
            }
        }

        return None;
    }
}
