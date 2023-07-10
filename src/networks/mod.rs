mod arbitrum;
use crate::utils;
use std::sync::Arc;

use crate::{exchanges::types::Exchange, types::Token};
use ethers::prelude::*;

pub use self::arbitrum::CHAIN_ID as arbitrum;

pub struct Network {
    pub chain_id: u32,
    pub name: String,
    pub exchanges: Vec<Exchange>,
    pub tokens: Vec<Arc<Token>>,
    pub lower_token_addressess: Vec<String>,
    pub flashloan_pool_address_provider: Address,
    pub uniswap_query_address: Address,
}

impl Network{
    pub fn from_chain_id(chain_id: &u32) -> Network{
        if *chain_id == arbitrum::CHAIN_ID {
            return arbitrum::get_instance();
        }
        else {
            panic!("NETWORK NOT FOUND");
        }
    }
}

fn load_exchanges_from_file(network_name: &str) -> Vec<Exchange> {
    return utils::json::deserialize_exchange_file(format!(
        "src/networks/{}/_exchanges.json",
        network_name
    ));
}

fn load_tokens_from_file(network_name: &str) -> Vec<Arc<Token>> {
    return utils::json::deserialize_token_file(format!(
        "src/networks/{}/_tokens.json",
        network_name
    ))
    .iter()
    .map(|t: &Token| Arc::new(*t))
    .collect();
}
