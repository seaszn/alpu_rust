use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
};
use rayon::prelude::*;

use super::{
    config::RuntimeConfig,
    types::{BundleExecutorContract, RuntimeClient, UniswapQueryContract},
};
use crate::{
    exchanges::{get_exchange_markets, get_market_reserves},
    networks::Network,
    types::{market::Market, ReserveTable, Route, Token},
    utils::parse::*,
};
use futures::executor::block_on;
use std::{io::Error, sync::Arc};

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

#[derive(Clone)]
pub struct RuntimeCache {
    pub client: RuntimeClient,
    pub uniswap_query: UniswapQueryContract,
    pub bundle_executor: BundleExecutorContract,
    pub markets: Vec<Arc<Market>>,
    pub routes: Vec<Route>,
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
                    routes: vec![],
                });
            }
            Err(ss) => {
                return Err(Error::new(std::io::ErrorKind::ConnectionRefused, ss));
            }
        }
    }

    pub async fn init_markets(&mut self, network: &Network, config: &RuntimeConfig) {
        _ = self.client.get_block_number().await;

        match get_exchange_markets(network, self, config).await {
            Ok(result) => {
                let reserves: ReserveTable = get_market_reserves(&result, &self, config).await;
                for market in result {
                    if let Some(reserves) = reserves.get_value(&market.contract_address) {
                        let min_reserve_0: u128 =
                            dec_to_u128(&config.min_market_reserves, market.tokens[0].decimals);
                        let min_reserve_1: u128 =
                            dec_to_u128(&config.min_market_reserves, market.tokens[1].decimals);

                        if reserves.0.ge(&min_reserve_0) && reserves.1.ge(&min_reserve_1) {
                            self.markets.push(market);
                        }
                    }
                }
            }
            Err(_) => {}
        };
    }

    pub async fn calculate_routes(&mut self, network: &Network, config: &RuntimeConfig) {
        let base_tokens: Vec<Arc<Token>> = {
            let mut res = network.tokens.to_vec();
            res.retain(|x| x.flash_loan_enabled);

            res
        };

        self.routes = base_tokens
            .par_iter()
            .flat_map(|token| {
                return Route::generate_from_base_token(
                    self.markets.clone(),
                    token.clone(),
                    config.route_restraints,
                );
            })
            .collect::<Vec<Route>>();
    }
}
