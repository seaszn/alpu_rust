use crate::networks::{self, Network};
use dotenv::dotenv;
use ethers::prelude::*;

use self::contracts::{get_bundle_executor, get_uniswap_query};
mod config;
mod contracts;

pub struct Environment {
    pub provider: Provider<Http>,
    pub network: Network,
    pub uniswap_query: contracts::UniswapQuery<Provider<Http>>,
    pub bundle_executor: contracts::BundleExecutor<Provider<Http>>,
    pub config: config::Configuration,
}

pub async fn init_environment() -> Environment {
    dotenv().ok();

    let config: config::Configuration = config::get_environment_config().clone();
    let rpc_endpoint: String = config.rpc_endpoint.clone();

    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_endpoint.to_owned()).unwrap();

    let network: networks::Network = networks::get_network(config.chain_id);
    // let wallet = config.private_key.parse::<LocalWallet>();

    let uniswap_query: contracts::UniswapQuery<Provider<Http>> = get_uniswap_query(
        Provider::<Http>::try_from(rpc_endpoint.to_owned()).unwrap(),
        network.uniswap_query_address,
    );

    let bundle_executor: contracts::BundleExecutor<Provider<Http>> = get_bundle_executor(
        Provider::<Http>::try_from(rpc_endpoint.to_owned()).unwrap(),
        config.bundle_executor_address,
    );

    let result: Environment = Environment {
        provider: provider,
        network,
        config: config,
        bundle_executor,
        uniswap_query,
    };

    return result;
}
