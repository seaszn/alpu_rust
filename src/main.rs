use ethers::providers::Middleware;

pub mod types;
pub mod utils;
pub mod networks;
pub mod exchanges;
pub mod environment;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();

    let env: environment::Environment = environment::init();


    print!("{}", env.cache.client.get_block_number().await.expect("Failed to get block number"))
    // let mut markets: Vec<market::Market> = vec![];
    // let mut _routes: Vec<route::MarketRoute> = vec![];

    // market::Market::test();
}
