mod arbitrum;
use ethers::prelude::*;

use crate::exchanges::Exchange;

pub struct Network {
     chain_id: u32,
    pub name: String,
    pub exchanges: Vec<Exchange>,
    pub flashloan_pool_address_provider: Address,
    pub uniswap_query_address: Address,
}

impl Network {
    pub fn chain_id(self) -> u32{
        return self.chain_id;
    }
}

pub fn get_network_instance(chain_id: &u32) -> Network {
    if *chain_id == arbitrum::chain_id() {
        return arbitrum::get_instance();
    }

    panic!("chain_id UNKOWN");
}
