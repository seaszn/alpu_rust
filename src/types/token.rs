use std::{sync::Arc, sync::Weak};

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
        runtime_network: &Arc<Network>,
    ) -> Option<Arc<Token>> {
        for market in &runtime_network.tokens {
            if market.contract_address.0 == address.as_address().unwrap().0 {
                return Some(market.clone());
            }
        }

        return None;
    }

    pub fn eq_unsafe(&self, other: &Weak<Self>) -> bool {
        return self.eq(&unsafe { *other.as_ptr() });
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return self.contract_address.eq(&other.contract_address);
    }

    fn ne(&self, other: &Self) -> bool {
        return self.contract_address.ne(&other.contract_address);
    }
}
