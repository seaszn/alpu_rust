use ethers::abi::RawLog;
use ethers::types::H160;

use crate::exchanges::types::Protocol;

#[derive(Clone)]
pub struct TransactionLog {
    pub address: H160,
    pub protocol: Protocol,
    pub raw: RawLog,
}
