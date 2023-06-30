use ethers::providers::Middleware;

pub mod environment;
pub mod exchanges;
pub mod networks;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();

    let env: environment::Environment = environment::init().await;

    // println!("{:#?}", &env.cache.tokens);
    // for exchange in env.network.exchanges {
    //     let markets = exchanges::get_exchange_markets(&exchange, &env.cache).await;
    // }

    print!(
        "{}",
        env.cache
            .client
            .get_block_number()
            .await
            .expect("Failed to get block number")
    )
    // let mut markets: Vec<market::Market> = vec![];
    // let mut _routes: Vec<route::MarketRoute> = vec![];

    // market::Market::test();
}
