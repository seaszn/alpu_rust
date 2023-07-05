use ethers::{types::*, abi::{ethereum_types::FromDecStrErr, AbiEncode}};
use serde_json::Value;

pub fn parse_address(value: &Value) -> H160 {
    return H160::from_slice(&parse_buffer(value));
}

pub fn parse_topic_buffer(value: &Value) -> Option<H256> {
    let s: Result<U256, FromDecStrErr> =
        U256::from_dec_str(value.as_str().unwrap());

    if s.is_ok() {
        return Some(H256::from_slice(s.unwrap().encode().as_slice()));
    }

    return None;
}

pub fn parse_buffer(value: &Value) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];
    for (key, value) in value.as_object().unwrap() {
        let index: usize = key.parse().unwrap();
        let bytecode: u8 = value.as_u64().unwrap().to_string().parse().unwrap();

        if index >= buffer.len() {
            buffer.push(bytecode);
        } else {
            buffer.insert(index, bytecode)
        }
    }

    return buffer;
}