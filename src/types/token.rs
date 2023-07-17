use ethers::prelude::*;
use serde::Deserialize;

use crate::networks::Network;

#[derive(Debug, Deserialize, Clone, Copy, Eq)]
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: u32,
}

impl Token {
    pub fn from_address(
        address: &NameOrAddress,
        runtime_network: &'static Network,
    ) -> Option<&'static Token> {
        for token in &runtime_network.tokens {
            if token.contract_address.0 == address.as_address().unwrap().0 {
                return Some(&token);
            }
        }

        return None;
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.contract_address.0.eq(&other.contract_address.0);
    }

    fn ne(&self, other: &Self) -> bool {
        return self.contract_address.0.ne(&other.contract_address.0);
    }
}
