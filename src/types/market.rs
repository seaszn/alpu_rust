use std::sync::Arc;

use ethers::prelude::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{env::RuntimeCache, exchanges::types::Protocol};

use super::{OrgValue, Token};

lazy_static! {
    static ref BASE_FEE_MUL: U256 = U256::from(10000u128);
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [&'static Token; 2],
    pub fee: i32,
    pub stable: bool,
    pub protocol: Protocol,
    fee_mul: U256,
}

impl Market {
    pub fn new(
        contract_address: Address,
        tokens: [&'static Token; 2],
        fee: i32,
        stable: bool,
        protocol: Protocol,
    ) -> Market {
        return Market {
            contract_address,
            tokens,
            fee,
            stable,
            protocol,
            fee_mul: U256::from(10000u128 - fee as u128),
        };
    }

    pub fn from_address(
        address: &H160,
        runtime_cache: &'static RuntimeCache,
    ) -> Option<&'static OrgValue<Market>> {
        for market in &runtime_cache.markets.to_vec() {
            if market.value.contract_address.0 == address.0 {
                return Some(market);
            }
        }

        // todo!("from_address in markets");

        return None;
    }

    pub fn get_market_addressess(markets: &Vec<Arc<Market>>) -> Vec<H160> {
        return markets.par_iter().map(|x| x.contract_address).collect();
    }

    #[inline(always)]
    pub fn get_fee_data(&self) -> (&U256, &U256) {
        return (
            &self.fee_mul,
            &BASE_FEE_MUL,
        );
    }
}
