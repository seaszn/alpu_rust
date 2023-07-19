use ethers::{
    abi::AbiEncode,
    types::{Bytes, H160, H256, U256},
};
use serde_json::Value;

#[inline(always)]
pub fn parse_topic_buffer(value: &Value) -> Option<H256> {
    if value.is_string() {
        if let Ok(parse_result) = U256::from_dec_str(value.as_str().unwrap()) {
            return Some(H256::from_slice(parse_result.encode().as_slice()));
        }
    }

    return None;
}

#[inline(always)]
pub fn parse_address(value: &Value) -> H160 {
    let bytes = Bytes::from(parse_buffer(value));
    return H160::from_slice(&bytes);
}

#[inline(always)]
pub fn parse_buffer(data: &Value) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    if let Some(buffer_map) = data.as_object() {
        for i in 0..buffer_map.len() {
            result.push(buffer_map[&i.to_string()].to_string().parse().unwrap());
        }
    }

    return result;
}
