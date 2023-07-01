use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
};
use std::sync::Arc;

use super::config::RuntimeConfig;
use crate::networks::Network;

// use crate::{networks::Network, types::Token, utils::json::deserialize_token_file};

// use super::config::Config;

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

pub struct RuntimeCache {
    pub client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub uniswap_query: Arc<UniswapQuery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    pub bundle_executor: Arc<BundleExecutor<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
}

pub fn init(config: &RuntimeConfig, network: &Network) -> RuntimeCache {
    let provider: Provider<Http> =
        Provider::<Http>::try_from(config.rpc_endpoint.clone()).expect("msg");

    let wallet = config
        .private_key
        .parse::<LocalWallet>()
        .expect("PRIVATE_KEY is not a valid private key");

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
   
    let uniswap_query = Arc::new(UniswapQuery::new(
        network.uniswap_query_address,
        client.clone(),
    ));

    let bundle_executor = Arc::new(BundleExecutor::new(config.executor_address, client.clone()));

    return  RuntimeCache{
        client,
        uniswap_query,
        bundle_executor
    };
}