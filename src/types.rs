use ethers::{abi::Address, types::U256};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(Clone, Copy)]
pub struct Token {
    pub contract_address: Address,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

#[derive(Clone, Copy)]
pub struct Market<'g> {
    pub contract_address: Address,
    pub tokens: [&'g Token; 2],
    pub fee: i32,
    pub stable: bool,
    pub reserves: MarketReserves
}

#[derive(Clone, Copy)]
pub struct MarketReserves {
    pub reserve0: U256,
    pub reserve1: U256
}
