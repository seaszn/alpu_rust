use ethers::types::{H256, Transaction};

pub struct TransactionDecodeResult {
    pub hash: H256,
    pub transaction: Transaction,
}