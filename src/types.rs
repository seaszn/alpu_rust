use ethers::abi::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub contract_address: Address,
    pub ref_symbol: Option<String>,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}
pub struct Market{
    pub tokens: [Address; 2],
    pub contract_address: Address,
    pub fee: i32,
    pub stable_fee: Option<i32>,
    pub stable: bool
}