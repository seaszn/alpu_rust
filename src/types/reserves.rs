use ethers::prelude::*;

pub type Reserves = (U256, U256);

pub trait ReverseReserves{
    fn reverse(&self) -> (U256, U256);
}

impl ReverseReserves for Reserves{
    #[inline(always)]
    fn reverse(&self) -> (U256, U256) {
        return (self.1, self.0);
    }
}