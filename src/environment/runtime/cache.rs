use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
};
use std::sync::Arc;

use crate::{networks::Network, types::Market};

use super::config::Config;

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

pub struct Cache<'g> {
    pub client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub uniswap_query: UniswapQuery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub bundle_executor: BundleExecutor<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub markets: Vec<&'g Market<'g>>,
}

pub fn get_runtime_cache<'c>(config: &Config, network: &Network) -> Cache<'c> {
    let provider: Provider<Http> =
        Provider::<Http>::try_from(config.rpc_endpoint.clone()).expect("msg");

    let wallet = config
        .private_key
        .parse::<LocalWallet>()
        .expect("PRIVATE_KEY is not a valid private key");

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    let uniswap_query = UniswapQuery::new(network.uniswap_query_address, client.clone());
    let bundle_executor = BundleExecutor::new(config.executor_address, client.clone());

    let markets: Vec<&'c Market> = vec![];

    return Cache {
        client,
        uniswap_query,
        bundle_executor,
        markets
    };
}
