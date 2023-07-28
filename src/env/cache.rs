use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider, Ws},
    signers::LocalWallet,
    types::{H160, U256},
};

use super::{
    config::RuntimeConfig,
    types::{BundleExecutorContract, RuntimeClient, UniswapQueryContract},
};

use crate::{
    exchanges::get_exchange_markets,
    networks::Network,
    types::{market::Market, OrganizedList, Reserves, Route},
    utils::parse::*,
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
        let provider: Provider<Ws> = block_on(Provider::<Ws>::connect(config.rpc_endpoint.as_str())).unwrap();
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

                    // result.calculate_routes(network, config);
                    // result.calculate_routes(network, config);
                    // result.calculate_routes(network, config).await;

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
                let market_addressess: Vec<H160> =
                    result.iter().map(|x| x.contract_address).collect();
                if let Ok(response) = self
                    .uniswap_query
                    .get_reserves_by_pairs(market_addressess.clone())
                    .await
                {
                    for i in 0..response.len() {
                        let reserves: Reserves =
                            (U256::from(response[i][0]), U256::from(response[i][1]));
                        let market = &result[i];

                        let min_reserve_0 =
                            dec_to_u256(&config.min_market_reserves, market.tokens[0].decimals);
                        let min_reserve_1 =
                            dec_to_u256(&config.min_market_reserves, market.tokens[1].decimals);

                        if reserves.0.ge(&min_reserve_0) && reserves.1.ge(&min_reserve_1) {
                            self.markets.add_value(market.clone());
                        }
                    }
                }
            }
            Err(_) => {}
        };

        // self.markets.sort_unstable_by(|x| x.)
    }

    /*
    pub fn calculate_routes(
        &mut self,
        network: &'static Network,
        config: &'static RuntimeConfig,
    ) {
        self.routes = network
            .tokens
            .iter()
            .filter(|token| token.flash_loan_enabled)
            .into_iter()
            .flat_map(|base_token| {
                return Route::generate_from_base_token(
                    &self.markets,
                    base_token,
                    config.route_restraints,
                );
            })
            .collect();

        // self.routes = base_tokens
        //     .par_iter()
        //     .flat_map(|token| {
        //         return Route::generate_from_base_token(
        //             self.markets.clone(),
        //             token.clone(),
        //             config.route_restraints,
        //         );
        //     })
        //     .collect::<Vec<Route>>();
    }
     */
}
