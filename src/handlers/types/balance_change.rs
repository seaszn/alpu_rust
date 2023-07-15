use ethers::types::*;

#[derive(Debug, Clone, Copy)]
pub struct BalanceChange {
    pub address: H160,
    pub amount_0_in: u128,
    pub amount_1_in: u128,
    pub amount_0_out: u128,
    pub amount_1_out: u128,
}