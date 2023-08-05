use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::LocalWallet,
};

use super::{
    config::RuntimeConfig,
    types::{BundleExecutorContract, RuntimeClient, UniswapQueryContract},
};

use crate::{
    exchanges::get_exchange_markets,
    networks::Network,
    types::{market::Market, OrganizedList, Route},
};
use futures::executor::block_on;
use std::{io::Error, sync::Arc};

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

#[derive(Clone)]
pub struct RuntimeCache {
    pub client: Arc<RuntimeClient>,
    pub uniswap_query: UniswapQueryContract,
    pub bundle_executor: BundleExecutorContract,
    pub markets: OrganizedList<Market>,
    pub routes: Vec<Route>,
}

impl RuntimeCache {
    pub fn new(
        config: &'static RuntimeConfig,
        network: &'static Network,
    ) -> Result<RuntimeCache, Error> {
        let provider: Provider<Ws> =
            block_on(Provider::<Ws>::connect(config.rpc_endpoint.as_str())).unwrap();
        let wallet = config
            .private_key
            .parse::<LocalWallet>()
            .expect("PRIVATE_KEY is not a valid private key");

        let client: Arc<RuntimeClient> = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

        let uniswap_query: UniswapQueryContract =
            UniswapQuery::new(network.uniswap_query_address, client.clone());

        let bundle_executor: BundleExecutorContract =
            BundleExecutor::new(config.executor_address, client.clone());

        return block_on(async {
            match client.client_version().await {
                Ok(version) => {
                    println!("Connected to client ({})\n", version);

                    let mut result: RuntimeCache = RuntimeCache {
                        client,
                        markets: OrganizedList::new(),
                        uniswap_query,
                        bundle_executor,
                        routes: vec![],
                    };

                    println!("Caching runtime...\n");
                    result.init_markets(network, config).await;
                    result.markets.sort();

                    return Ok(result);
                }
                Err(ss) => {
                    return Err(Error::new(std::io::ErrorKind::ConnectionRefused, ss));
                }
            }
        });
    }

    async fn init_markets(&mut self, network: &'static Network, config: &'static RuntimeConfig) {
        _ = self.client.get_block_number().await;

        match get_exchange_markets(network, self, config).await {
            Ok(result) => {
                for market in result {
                    self.markets.add_value(market.clone());
                }
            }
            Err(_) => {}
        };
    }
}
