use std::sync::Arc;

use ethers::types::Address;

use super::Token;

#[derive(Clone)]
pub struct Market {
    pub contract_address: Address,
    pub tokens: [Arc<Token>; 2],
    pub fee: i32,
    pub stable: bool,
}
