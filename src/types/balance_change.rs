use ethers::types::U256;

use crate::types::{OrgValue, market::Market};

#[derive(Debug, Clone)]
pub struct BalanceChange {
    pub market: &'static OrgValue<Market>,
    pub amount_0_in: U256,
    pub amount_1_in: U256,
    pub amount_0_out: U256,
    pub amount_1_out: U256,
}