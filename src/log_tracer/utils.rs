use ethers::{
    abi::AbiEncode,
    types::{Bytes, H160, H256, U256},
};
use serde_json::Value;

pub fn parse_topic_buffer(value: &Value) -> Option<H256> {
    if value.is_string() {
        if let Ok(parse_result) = U256::from_dec_str(value.as_str().unwrap()) {
            return Some(H256::from_slice(parse_result.encode().as_slice()));
        }
    }

    return None;
}

pub fn parse_address(value: Value) -> H160 {
    let bytes = Bytes::from(parse_number_array(value));
    return H160::from_slice(&bytes);
}

pub fn parse_number_array(data: Value) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    if let Some(arr) = data.as_array() {
        for a in arr {
            let value: u8 = a.to_string().parse().unwrap();
            result.push(value);
        }
    }

    return result;
}
