pub mod types;
pub mod utils;
pub mod networks;
pub mod exchanges;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();

    // let mut markets: Vec<market::Market> = vec![];
    // let mut _routes: Vec<route::MarketRoute> = vec![];

    // market::Market::test();
}
