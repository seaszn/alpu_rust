#[macro_use]
extern crate lazy_static;
extern crate base64;

pub mod env;
pub mod exchanges;
mod handlers;
pub mod log_tracer;
pub mod networks;
pub mod types;
pub mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    utils::logger::clear_console();

    // println!("{:?}", hex::decode("00000000000000000000000000000000000000000000001625680334c788f7320000000000000000000000000000000000000000000094d23a6e9a67286a1302").unwrap());

    println!("\nRunning on {}\n", env::RUNTIME_NETWORK.name);

    println!("\nWallet: {}", env::RUNTIME_CACHE.client.address());
    println!("Executor: {}", env::RUNTIME_CACHE.bundle_executor.address());
    println!("Query: {}", env::RUNTIME_CACHE.uniswap_query.address());

    println!("\nLoaded {} markets\n", env::RUNTIME_CACHE.markets.len());

    // println!("Found {} markets...", env::run);
    // RuntimeCache::load_markets();
    // Get the markets

    // Calculate the route templates

    handlers::init(env::RUNTIME_CONFIG.chain_id).await;
}
