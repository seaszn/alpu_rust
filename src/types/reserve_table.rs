use std::collections::HashMap;

use ethers::{types::H160, types::U256};

pub type Reserves = (U256, U256);
pub type ReserveTable = HashMap<H160, Reserves>;
