use ethers::abi::RawLog;
use ethers::types::{Address, Bytes, NameOrAddress, U256, U64, H160, TransactionRequest};

use crate::exchanges::types::Protocol;

#[derive(Debug)]
pub struct Transaction {
    pub hash: String,
    pub to: Option<NameOrAddress>,
    pub from: Option<Address>,
    pub value: Option<U256>,
    pub data: Option<Bytes>,
    pub gas: Option<U256>,
    pub gas_price: Option<U256>,
    pub nonce: Option<U256>,
    pub chain_id: Option<U64>,
}

impl Transaction {
    pub fn to_request(&self) -> TransactionRequest{
        return TransactionRequest {
            from: self.from,
            to: self.to.clone(),
            value: self.value,
            data: self.data.clone(),
            gas: self.gas,
            gas_price: self.gas_price,
            nonce: self.nonce,
            chain_id: self.chain_id,
        };
    }
}

#[derive(Clone)]
pub struct TransactionLog {
    pub address: H160,
    pub protocol: Protocol,
    pub raw: RawLog,
}
