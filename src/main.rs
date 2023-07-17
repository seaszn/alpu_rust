use std::io::Error;

use env::{RuntimeCache, RuntimeConfig};
use networks::Network;
use types::Route;

#[macro_use]
extern crate lazy_static;
extern crate async_trait;
extern crate base64;

pub mod env;
pub mod exchanges;
mod handlers;
pub mod log_tracer;
pub mod networks;
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
            println!("Query: {}", runtime_cache.uniswap_query.address());
            println!("Wallet: {}", runtime_cache.client.address());
            println!("Executor: {}\n", runtime_cache.bundle_executor.address());

            println!("Cached {} tokens..", RUNTIME_NETWORK.tokens.len());
            println!("Cached {} markets..", runtime_cache.markets.len());
            println!("Cached {} routes..\n", RUNTIME_ROUTES.len());

            //         // runtime_cache
            //         //     .calculate_routes(&runtime_network, &RUNTIME_CONFIG)
            //         //     .await;
            //         // println!("Found {} routes..\n", runtime_cache.routes.len());

            if let Some(handler) = handlers::Handler::new(RUNTIME_NETWORK.chain_id).await {
                handler.init(&*RUNTIME_CONFIG, runtime_cache).await;
            }
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }

    // handlers::init(env::RUNTIME_CONFIG.chain_id).await;
}
