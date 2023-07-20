use ethers::types::U256;

use super::{OrgValue, market::Market, Token};

pub struct SwapLog{
    pub market: &'static OrgValue<Market>,
    pub token_in: &'static Token,
    pub token_out: &'static Token,
    pub amount_in: U256,
    pub amount_out: U256,
}