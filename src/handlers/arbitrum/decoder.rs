use crate::types::Transaction;
use ethers::{
    types::{transaction::eip1559, TransactionRequest},
    utils::{
        hex, keccak256,
        rlp::{self, DecoderError},
    },
};

use super::FROM_ADDRESS;

pub fn decode_transaction(data: &[u8]) -> Result<Transaction, DecoderError> {
    let _tx_hash = hex::encode(keccak256(data));
    let from: Option<ethers::types::NameOrAddress> =
        Some(ethers::types::NameOrAddress::Address(*FROM_ADDRESS));
    // let from: Result<Address, _> = "".to_string().parse();

    let legacy_transaction: Result<TransactionRequest, DecoderError> = rlp::decode(data);
    if legacy_transaction.is_ok() {
        if legacy_transaction.is_ok() {
            let tx = legacy_transaction.unwrap();

            return Ok(Transaction {
                to: tx.to,
                value: tx.value,
                data: tx.data,
                from,
            });
        }
    }

    let eip1559_transaction: Result<eip1559::Eip1559TransactionRequest, DecoderError> =
        rlp::decode(data.split_first().unwrap().1);

    if eip1559_transaction.is_ok() {
        let tx = eip1559_transaction.unwrap();

        return Ok(Transaction {
            to: tx.to,
            value: tx.value,
            data: tx.data,
            from,
        });
    } else {
        Err(eip1559_transaction.err().unwrap())
    }
}
