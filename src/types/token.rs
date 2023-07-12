use std::sync::Arc;

use ethers::prelude::*;
use serde::Deserialize;

use crate::networks::Network;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

impl Token {
    pub fn from_address(
        address: &NameOrAddress,
        runtime_network: &Arc<Network>,
    ) -> Option<Arc<Token>> {
        for market in &runtime_network.tokens {
            if market.contract_address.0 == address.as_address().unwrap().0 {
                return Some(market.clone());
            }
        }

        return None;
    }
}
