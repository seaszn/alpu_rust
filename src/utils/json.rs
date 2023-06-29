use std::fs;
use crate::networks::types;

pub fn deserialize_token_file(path: &str) -> Vec<types::Token> {
    let file_contents = fs::read_to_string(path);
    let result = serde_json::from_str(&file_contents.unwrap()).expect("JSON was not well-formatted");

    return result;
}
