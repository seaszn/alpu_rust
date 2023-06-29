mod arbitrum;
use serde::Deserialize;

use crate::{utils, exchanges::Exchange};

#[derive(Debug, Deserialize)]
pub struct Token {
    pub symbol: String,
    pub contract_address: String,
    pub ref_symbol: Option<String>,
    pub flash_loan_enabled: bool,
    pub decimals: i32,
}

pub struct Network {
    pub chain_id: i32,
    pub name: String,
    pub flashloan_pool_address_provider: String,
    pub tokens: Vec<Token>,
    pub exchanges: Vec<Exchange>,
}

pub fn get_network(chain_id: i32) -> Network {
    let mut network: Network = get_network_instance(chain_id);

    network.tokens = utils::json::deserialize_token_file(format!("src/networks/{}/_tokens.json", &network.name ));
    network.exchanges = utils::json::deserialize_exchange_file(format!("src/networks/{}/_exchanges.json", &network.name ));

    return  network;
}

fn get_network_instance(chain_id: i32) -> Network {
    println!("getting network");

    // Arbitrum One
    if chain_id == arbitrum::get_chain_id() {
        return arbitrum::get_instance();
    }

    panic!("d");
}
