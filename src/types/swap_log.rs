use ethers::types::U256;

use super::{market::Market, OrgValue};

pub struct SwapLog {
    pub market: &'static OrgValue<Market>,
    pub amount_0_out: U256,
    pub amount_1_out: U256,
}
