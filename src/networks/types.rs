use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub symbol: String,
    pub contract_address: String,
    pub ref_symbol: Option<String>,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

pub struct Network {
    pub chain_id: i32,
    pub name: String,
    pub flashloan_pool_address_provider: String,
    // pub tokens: Vec<Token>
}

