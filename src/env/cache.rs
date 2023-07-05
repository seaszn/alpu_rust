use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use futures::executor::block_on;
use std::sync::Arc;

use super::{
    config::RuntimeConfig,
    types::{BundleExecutorContract, RuntimeClient, UniswapQueryContract},
};
use crate::{exchanges::get_exchange_markets, networks::Network, types::market::Market};

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

pub struct RuntimeCache {
    pub client: RuntimeClient,
    pub uniswap_query: UniswapQueryContract,
    pub bundle_executor: BundleExecutorContract,
    pub markets: Vec<Arc<Market>>,
    pub market_addressess: Vec<[u8; 20]>
}

pub fn init(config: &RuntimeConfig, network: Arc<Network>) -> RuntimeCache {
    let provider: Provider<Http> =
        Provider::<Http>::try_from(config.rpc_endpoint.as_str()).expect("msg");

    let wallet = config
        .private_key
        .parse::<LocalWallet>()
        .expect("PRIVATE_KEY is not a valid private key");

    let client: RuntimeClient = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    let uniswap_query: UniswapQueryContract = Arc::new(UniswapQuery::new(
        network.uniswap_query_address,
        client.clone(),
    ));

    let bundle_executor: BundleExecutorContract =
        Arc::new(BundleExecutor::new(config.executor_address, client.clone()));

    let markets = block_on(get_markets(
        network.clone(),
        client.clone(),
        uniswap_query.clone(),
    ));

    let market_addressess: Vec<[u8;20 ]> = markets.clone().iter().map(|x| x.contract_address.0).collect();

    return RuntimeCache {
        client,
        markets,
        market_addressess,
        uniswap_query,
        bundle_executor,
    };
}

async fn get_markets(
    network: Arc<Network>,
    client: RuntimeClient,
    uniswap_query: UniswapQueryContract,
) -> Vec<Arc<Market>> {
    let mut markets: Vec<Arc<Market>> = vec![];

    for exchange in &network.exchanges {
        markets.append(
            (get_exchange_markets(
                exchange,
                network.clone(),
                client.clone(),
                uniswap_query.clone(),
            )
            .await)
                .as_mut(),
        );
    }

    return markets;
}
