use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::H160,
};

use super::{
    config::RuntimeConfig,
    types::{BundleExecutorContract, RuntimeClient, UniswapQueryContract},
};
use crate::{
    exchanges::{get_exchange_markets, get_market_reserves},
    networks::Network,
    types::market::Market,
};
use futures::executor::block_on;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{io::Error, sync::Arc, time::Instant, ops::DerefMut};

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

#[derive(Clone)]
pub struct RuntimeCache {
    pub client: RuntimeClient,
    pub uniswap_query: UniswapQueryContract,
    pub bundle_executor: BundleExecutorContract,
    pub markets: Vec<Arc<Market>>,
}

impl RuntimeCache {
    pub fn new(config: &RuntimeConfig, network: &Network) -> Result<RuntimeCache, Error> {
        let provider: Provider<Http> =
            Provider::<Http>::try_from(config.rpc_endpoint.as_str()).expect("msg");

        let wallet = config
            .private_key
            .parse::<LocalWallet>()
            .expect("PRIVATE_KEY is not a valid private key");

        let client: RuntimeClient =
            Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

        let uniswap_query: UniswapQueryContract = Arc::new(UniswapQuery::new(
            network.uniswap_query_address,
            client.clone(),
        ));

        let bundle_executor: BundleExecutorContract =
            Arc::new(BundleExecutor::new(config.executor_address, client.clone()));

        match block_on(client.client_version()) {
            Ok(version) => {
                println!("Connected to client ({})\n", version);

                return Ok(RuntimeCache {
                    client,
                    markets: vec![],
                    uniswap_query,
                    bundle_executor,
                });
            }
            Err(ss) => {
                return Err(Error::new(std::io::ErrorKind::ConnectionRefused, ss));
            }
        }
    }

    pub async fn init_markets(&mut self, network: &Network) {
        let mut unfiltered_markets: Vec<Arc<Market>> = vec![];

        match get_exchange_markets(network, self).await {
            Ok(mut result) => {
                unfiltered_markets.append(&mut result);

                let inst = Instant::now();
                let f = get_market_reserves(&unfiltered_markets, &self).await;
                println!("time took: {:?}", inst.elapsed())
            }
            Err(_) => {}
        };
    }
    pub async fn calculate_routes(&self) {}
}

