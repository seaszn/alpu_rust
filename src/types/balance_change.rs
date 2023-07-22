use ethers::types::{U64, Transaction};

use crate::types::{OrgValue, market::Market};

#[derive(Debug, Clone)]
pub struct BalanceChange {
    pub market: &'static OrgValue<Market>,
    pub amount_0_in: u128,
    pub amount_1_in: u128,
    pub amount_0_out: u128,
    pub amount_1_out: u128,
}