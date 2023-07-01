use std::sync::Weak;

use ethers::types::Address;

use super::Token;

#[derive(Clone)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [Weak<Token>; 2],
    pub fee: i32,
    pub stable: bool,
}
