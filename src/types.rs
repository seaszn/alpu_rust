use ethers::abi::Address;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(Clone, Copy)]
pub struct Token {
    pub contract_address: Address,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

#[derive(Clone, Copy)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [Address; 2],
    pub fee: i32,
    pub stable: bool,
}

// #[derive(Clone, Copy)]
// pub struct MarketReserves {
//     pub reserve0: U256,
//     pub reserve1: U256
// }
