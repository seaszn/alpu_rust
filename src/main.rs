use std::sync::Arc;

use env::RuntimeCache;
use networks::Network;

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
    let runtime_config: env::RuntimeConfig = env::RuntimeConfig::from_dot_env_file();
    let runtime_network: Arc<Network> = Arc::new(Network::from_chain_id(&runtime_config.chain_id));

    println!("Connecting to {} network...\n", runtime_network.name);

    utils::logger::clear_console();

    match RuntimeCache::new(&runtime_config, &runtime_network) {
        Ok(mut runtime_cache) => {
            println!("Query: {}", runtime_cache.uniswap_query.address());
            println!("Wallet: {}", runtime_cache.client.address());
            println!("Executor: {}\n", runtime_cache.bundle_executor.address());
            
            println!("Found {} tokens..", runtime_network.tokens.len());

            runtime_cache.init_markets(&runtime_network).await;
            println!("Found {} markets..\n", runtime_cache.markets.len());
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }

    // handlers::init(env::RUNTIME_CONFIG.chain_id).await;
}
