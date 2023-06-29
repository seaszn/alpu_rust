use std::fs;

use crate::{env, utils};

use super::types::{Network, Token};

const CHAIN_ID: i32 = 42161;

const NAME: &str = "Arbritrum One";
const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";

pub fn get_chain_id() -> i32 {
    return CHAIN_ID;
}

pub fn get_instance() -> Network {
    let tokens = utils::json::deserialize_token_file("src/networks/arbitrum/tokens.json");
    // let file = fs::read_to_string("src/networks/arbitrum/tokens.json");

    println!("{}", tokens[0].contract_address);
    // let tokens  = serde_json::from_str(&file.unwrap()).expect("JSON was not well-formatted");

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_string(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.to_string(),
        // tokens,
    };
}
