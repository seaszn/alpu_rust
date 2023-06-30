use std::ptr::addr_of;

use ethers::abi::Tokenizable;
use ethers::prelude::abigen;
use ethers::prelude::H160;
use ethers::types::{Address, U256};

use super::Exchange;
use crate::environment::runtime::{self, *};
use crate::types::Market;

abigen!(UniswapV2Factory, "src/exchanges/uniswap_v2/_factory.json");

pub async fn get_markets(exchange: &Exchange, runtime_cache: &Cache) -> Vec<Market> {
    let factory_contract =
        UniswapV2Factory::new(exchange.factory_address, runtime_cache.client.clone());

    let start: U256 = U256::zero();
    let market_count: U256 = factory_contract.all_pairs_length().await.expect("");

    let f: Vec<[Address; 3]> = runtime_cache
        .uniswap_query
        .get_uniswap_v2_markets(exchange.factory_address, start, U256::from(100))
        .await
        .expect("");

    println!("{:#?}", f[0][0]);
    // let f = U256::from("2");

    // println!("{:?}", f);
    //     .get_uniswap_v2_markets(exchange.factory_address, U256::from(0), market_count - 1)
    //     .await.expect("");

    let markets: Vec<Market> = vec![];

    // runtime_cache.uniswap_query.get_uniswap_v2_markets(exchange.factory_address, U256::from(0), 100);

    return markets;
}
