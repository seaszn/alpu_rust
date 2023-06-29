use std::fs;

pub fn deserialize_token_file(path: String) -> Vec<crate::networks::Token> {
    let file_contents = fs::read_to_string(path);
    let result = serde_json::from_str(&file_contents.unwrap()).expect("JSON was not well-formatted");

    return result;
}

pub fn deserialize_exchange_file(path: String) -> Vec<crate::exchanges::Exchange> {
    let file_contents = fs::read_to_string(path);
    let result = serde_json::from_str(&file_contents.unwrap()).expect("JSON was not well-formatted");

    return result;
}
