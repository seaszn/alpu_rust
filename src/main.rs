use std::io::Error;

use env::{RuntimeCache, RuntimeConfig};
use networks::Network;
use price_oracle::PriceOracle;
use types::Route;

use crate::{exchanges::init_exchange_handlers, handlers::NetworkHandler};

#[macro_use]
extern crate lazy_static;
extern crate async_trait;
extern crate base64;

pub mod env;
pub mod exchanges;
mod handlers;
pub mod log_tracer;
pub mod networks;
pub mod price_oracle;
pub mod types;
pub mod utils;

lazy_static! {
    static ref RUNTIME_CONFIG: RuntimeConfig = env::RuntimeConfig::from_dot_env_file();
    static ref RUNTIME_NETWORK: Network = Network::from_chain_id(&RUNTIME_CONFIG.chain_id);
    static ref RUNTIME_CACHE: Result<RuntimeCache, Error> =
        RuntimeCache::new(&RUNTIME_CONFIG, &RUNTIME_NETWORK);
    static ref RUNTIME_ROUTES: Vec<Route> =
        Route::generate_from_runtime(&RUNTIME_NETWORK, &RUNTIME_CONFIG, &RUNTIME_CACHE);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Connecting to {} network...\n", &*RUNTIME_NETWORK.name);

    utils::logger::clear_console();

    match &*RUNTIME_CACHE {
        Ok(runtime_cache) => {
            init_exchange_handlers();

            println!("Query: {}", runtime_cache.uniswap_query.address());
            println!("Wallet: {}", runtime_cache.client.address());
            println!("Executor: {}\n", runtime_cache.bundle_executor.address());

            println!("Cached {} tokens..", RUNTIME_NETWORK.tokens.len());
            println!("Cached {} markets..", runtime_cache.markets.len());
            println!("Cached {} routes..\n", RUNTIME_ROUTES.len());

            // println!("Waiting for validation, this might take a while");

            if let Some(mut network_handler) =
                NetworkHandler::from_network(&RUNTIME_NETWORK, &RUNTIME_CONFIG, runtime_cache)
            {
                network_handler.init().await;
            }
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}
