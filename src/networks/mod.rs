mod arbitrum;
use ethers::prelude::*;

use crate::exchanges::Exchange;

pub struct Network {
    pub chain_id: i32,
    pub name: String,
    pub exchanges: Vec<Exchange>,
    pub flashloan_pool_address_provider: Address,
    pub uniswap_query_address: Address,
}

pub fn get_network_instance(chain_id: &i32) -> Network {
    if *chain_id == arbitrum::get_chain_id() {
        return arbitrum::get_instance();
    }

    panic!("chain_id UNKOWN");
}
