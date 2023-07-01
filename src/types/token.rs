use ethers::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(Clone, Copy)]
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}