use ethers::prelude::*;

use crate::{env::RuntimeCache, exchanges::{types::Protocol, calculate_amount_out}};

use super::{OrgValue, Token, Reserves};

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

    #[inline(always)]
    pub fn from_address(
        address: &H160,
        runtime_cache: &'static RuntimeCache,
    ) -> Option<&'static OrgValue<Market>> {
        for market in &runtime_cache.markets.to_vec() {
            if market.value.contract_address.0 == address.0 {
                return Some(market);
            }
        }

        return None;
    }

    #[inline(always)]
    pub fn get_fee_data(&self) -> (&U256, &U256) {
        return (
            &self.fee_mul,
            &BASE_FEE_MUL,
        );
    }

    #[inline(always)]
    pub fn amount_out(&self, reserves: &Reserves, input_amount: &U256) -> U256{
        return calculate_amount_out(reserves, input_amount, self);
    }
}
