mod arbitrum;
use crate::utils;
use std::sync::Arc;

use ethers::prelude::*;
use crate::{exchanges::Exchange, types::Token};
pub struct Network {
    pub chain_id: u32,
    pub name: String,
    pub exchanges: Vec<Exchange>,
    pub tokens: Vec<Arc<Token>>,
    pub flashloan_pool_address_provider: Address,
    pub uniswap_query_address: Address,
}

pub fn get_instance(chain_id: &u32) -> Network {
    if *chain_id == arbitrum::CHAIN_ID {
        return arbitrum::get_instance();
    }

    panic!("chain_id UNKOWN");
}

fn load_exchanges_from_file(network_name: &str) -> Vec<crate::exchanges::Exchange> {
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
