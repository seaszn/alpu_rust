use ethers::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq)]
pub struct Token {
    pub contract_address: H160,
    pub flash_loan_enabled: bool,
    pub decimals: u32,
    pub ref_symbol: Option<String>
}


impl PartialEq for Token {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        return self.contract_address == other.contract_address;
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        return self.contract_address != other.contract_address;
    }
}
