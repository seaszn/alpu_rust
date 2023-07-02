#[macro_use]
extern crate lazy_static;
extern crate base64;

mod handlers;
pub mod env;
pub mod exchanges;
pub mod networks;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();

    handlers::init(env::RUNTIME_CONFIG.chain_id).await;
    println!("{}", env::RUNTIME_CONFIG.chain_id);
    // println!("{}", env::RUNTIME_NETWORK.name);

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
