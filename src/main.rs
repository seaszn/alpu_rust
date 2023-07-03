#[macro_use]
extern crate lazy_static;
extern crate base64;

mod handlers;
pub mod env;
pub mod exchanges;
pub mod networks;
pub mod types;
pub mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    utils::logger::clear_console();

    println!("\nRunning on {}\n", env::RUNTIME_NETWORK.name);

    println!("Wallet: {}", env::RUNTIME_CACHE.client.address());
    println!("Executor: {}", env::RUNTIME_CACHE.bundle_executor.address());
    println!("Query: {}\n", env::RUNTIME_CACHE.uniswap_query.address());

    // RuntimeCache::load_markets();
    // Get the markets

    // Calculate the route templates

    handlers::init(env::RUNTIME_CONFIG.chain_id).await;
}
