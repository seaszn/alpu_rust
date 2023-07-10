use ethers::abi::RawLog;
use ethers::types::transaction::eip1559::*;
use ethers::types::{Address, Bytes, NameOrAddress, TransactionRequest, H160, H256, U256, U64};
use ethers::utils::keccak256;
use ethers::utils::rlp::*;

use crate::env;
use crate::exchanges::types::Protocol;

lazy_static! {
    static ref TOP_WALLET_ADDRESS: Address = "0xf977814e90da44bfa03b6295a0616a897441acec"
        .to_string()
        .parse()
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: H256,
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
    pub fn to_request(&self) -> TransactionRequest {
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

    pub fn from_data(data: &[u8]) -> Option<Transaction> {
        let tx_hash = H256::from_slice(&keccak256(data));

        if let Ok(legacy_transaction) = decode::<TransactionRequest>(data) {
            return Some(Transaction {
                hash: tx_hash,
                to: legacy_transaction.to,
                value: legacy_transaction.value,
                data: legacy_transaction.data,
                from: Some(*TOP_WALLET_ADDRESS),
                gas: legacy_transaction.gas,
                gas_price: legacy_transaction.gas_price,
                nonce: None,
                chain_id: None,
            });
        } else if let Ok(eip1559_transaction) =
            decode::<Eip1559TransactionRequest>(data.split_first().unwrap().1)
        {
            return Some(Transaction {
                hash: tx_hash,
                to: eip1559_transaction.to,
                value: eip1559_transaction.value,
                data: eip1559_transaction.data,
                from: Some(*TOP_WALLET_ADDRESS),
                gas: eip1559_transaction.gas,
                gas_price: eip1559_transaction.max_fee_per_gas,
                nonce: None,
                chain_id: None,
            });
        }

        return None;
    }
}

#[derive(Clone)]
pub struct TransactionLog {
    pub address: H160,
    pub protocol: Protocol,
    pub raw: RawLog,
}
