#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use ethers::providers::Middleware;

pub mod environment;
pub mod exchanges;
pub mod networks;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();

    println!("{}", environment::RUNTIME_CONFIG.chain_id);
    println!("{}", environment::NETWORK.name);

    // print!(
    //     "{}",
    //     env.cache
    //         .client
    //         .get_block_number()
    //         .await
    //         .expect("Failed to get block number")
    // )
    // let mut markets: Vec<market::Market> = vec![];
    // let mut _routes: Vec<route::MarketRoute> = vec![];

    // market::Market::test();
}
