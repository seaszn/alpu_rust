mod arbitrum;
use ethers::prelude::*;

use crate::{exchanges::Exchange, types::Token, utils};

pub struct Network {
    pub chain_id: i32,
    pub name: String,
    pub tokens: Vec<Token>,
    pub exchanges: Vec<Exchange>,
    pub flashloan_pool_address_provider: Address,
    pub uniswap_query_address: Address,
}

pub fn from_chain_id(chain_id: i32) -> Network {
    let mut network: Network = get_network_instance(chain_id);

    network.tokens =
        utils::json::deserialize_token_file(format!("src/networks/{}/_tokens.json", &network.name));

    network.exchanges = utils::json::deserialize_exchange_file(format!(
        "src/networks/{}/_exchanges.json",
        &network.name
    ));

    return network;
}

fn get_network_instance(chain_id: i32) -> Network {
    if chain_id == arbitrum::get_chain_id() {
        return arbitrum::get_instance();
    }

    panic!("d");
}
