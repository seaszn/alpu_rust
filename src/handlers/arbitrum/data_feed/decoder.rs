use crate::types::Transaction;
use ethers::{
    types::{transaction::eip1559, U64, TransactionRequest},
    utils::{
        hex, keccak256,
        rlp::{self, DecoderError},
    }, abi::Address,
};

const RAW_FROM_ADDRESS: &str = "0xf977814e90da44bfa03b6295a0616a897441acec";

lazy_static! {
    static ref FROM_ADDRESS: Address = RAW_FROM_ADDRESS.to_string().parse().unwrap();
}

pub fn decode_transaction(data: &[u8]) -> Result<Transaction, DecoderError> {
    let tx_hash = hex::encode(keccak256(data));
    let from: Option<Address> =
        Some(Address::from(*FROM_ADDRESS));
    // let from: Result<Address, _> = "".to_string().parse();

    let legacy_transaction: Result<TransactionRequest, DecoderError> = rlp::decode(data);
    if legacy_transaction.is_ok() {
        if legacy_transaction.is_ok() {
            let tx = legacy_transaction.unwrap();
            return Ok(Transaction {
                hash: tx_hash,
                to: tx.to,
                value: tx.value,
                data: tx.data,
                from,
                gas: tx.gas,
                gas_price: tx.gas_price,
                nonce: tx.nonce,
                chain_id: Some(U64::from(tx.chain_id.unwrap()))
            });
        }
    }

    let eip1559_transaction: Result<eip1559::Eip1559TransactionRequest, DecoderError> =
        rlp::decode(data.split_first().unwrap().1);

    if eip1559_transaction.is_ok() {
        let tx = eip1559_transaction.unwrap();

        return Ok(Transaction {
            hash: tx_hash,
            to: tx.to,
            value: tx.value,
            data: tx.data,
            from,
            gas: tx.gas,
            gas_price: tx.max_fee_per_gas,
            nonce: tx.nonce,
            chain_id: tx.chain_id
        });
    } else {
        Err(eip1559_transaction.err().unwrap())
    }
}
