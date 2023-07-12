use ethers::types::U256;

use super::Dictionary;

pub type Reserves = (U256, U256);
pub type ReserveTable = Dictionary<[u8; 20], Reserves>;
