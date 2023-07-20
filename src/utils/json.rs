use std::fs;

use crate::exchanges::types::Exchange;

pub fn deserialize_token_file(path: String) -> Vec<crate::types::Token> {
    let file_contents = fs::read_to_string(path).expect("Failed to read token file");
    let result: Vec<crate::types::Token> =
        serde_json::from_str(&file_contents).expect("JSON was not well-formatted");

    return result;
}

pub fn deserialize_exchange_file(path: String) -> Vec<Exchange> {
    let file_contents = fs::read_to_string(path).expect("Failed to read exchange file");
    let result = serde_json::from_str(&file_contents).expect("JSON was not well-formatted");

    return result;
}
