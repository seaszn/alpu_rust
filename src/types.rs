use std::sync::Weak;

use ethers::abi::Address;
use serde::Deserialize;
use ethers::prelude::*;



#[derive(Debug, Deserialize)]
#[derive(Clone, Copy)]
    
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

#[derive(Clone)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [Weak<Token>; 2],
    pub fee: i32,
    pub stable: bool,
}

// #[derive(Clone, Copy)]
// pub struct MarketReserves {
//     pub reserve0: U256,
//     pub reserve1: U256
// }
