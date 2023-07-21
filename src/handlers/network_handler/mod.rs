use std::time::Instant;

use ethers::{
    prelude::AbiError,
    types::{Address, Bytes, TransactionRequest, U256},
    utils::format_units,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    env::{RuntimeCache, RuntimeConfig, EXECUTE_TX_BUNDLE_FUNCTION},
    exchanges::populate_swap,
    networks::Network,
    price_oracle::PriceOracle,
    types::{BundleExecutionCall, OrganizedList, PriceTable, Reserves, RouteResult},
    RUNTIME_CACHE, RUNTIME_CONFIG, RUNTIME_ROUTES,
};

use super::{market_data_feed::get_network_data_feed, MarketDataFeed};

lazy_static! {
    static ref BASE_TRANSACTION: TransactionRequest = TransactionRequest {
        from: Some(RUNTIME_CACHE.as_ref().unwrap().client.address()),
        to: Some(ethers::types::NameOrAddress::Address(
            RUNTIME_CONFIG.executor_address,
        )),
        gas: None,
        gas_price: None,
        value: None,
        data: None,
        chain_id: None,
        nonce: None,
    };
}

pub struct NetworkHandler {
    /* private */
    runtime_config: &'static RuntimeConfig,
    runtime_cache: &'static RuntimeCache,
    price_oracle: &'static PriceOracle,
    data_feed: &'static (dyn MarketDataFeed + Send + Sync),
    base_transaction: TransactionRequest,
}

impl NetworkHandler {
    #[inline(always)]
    pub fn from_network(
        network: &'static Network,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
        price_oracle: &'static PriceOracle,
    ) -> Option<NetworkHandler> {
        if let Some(data_feed) = get_network_data_feed(network.chain_id) {
            return Some(NetworkHandler {
                runtime_config,
                runtime_cache,
                price_oracle,
                data_feed,
                base_transaction: BASE_TRANSACTION.clone(),
            });
        }

        return None;
    }

    #[inline(always)]
    pub async fn init(&self) {
        let (sender, mut receiver): (Sender<(OrganizedList<Reserves>, Vec<usize>)>, Receiver<_>) =
            channel(32);

        let data_feed = self.data_feed;
        let config_reference = self.runtime_config;
        let cache_reference = self.runtime_cache;

        // initiate the data feed
        _ = tokio::spawn(async move {
            _ = data_feed
                .init(sender, config_reference, cache_reference)
                .await;
        });

        let mut first_message = true;
        while let Some((reserve_table, market_ids)) = receiver.recv().await {
            // Skip the first message, the node's activity has not been whispered yet, so first message is often significantly delayed
            if !first_message {
                if reserve_table.len() > 0 {
                    self.handle_market_update(&reserve_table, &market_ids).await;
                }
            } else {
                println!("Validation received...\n");
                println!("Listening to market changes...\n");
                first_message = false;
            }
        }
    }

    #[inline(always)]
    async fn handle_market_update(
        &self,
        reserve_table: &OrganizedList<(U256, U256)>,
        market_ids: &Vec<usize>,
    ) {
        let inst = Instant::now();
        let price_table: &PriceTable = self.price_oracle.get_ref_price_table();

        let route_results: Vec<RouteResult> = RUNTIME_ROUTES
            .par_iter()
            .filter_map(|x| {
                return x.calculate_result(&reserve_table, price_table, &market_ids);
            })
            .collect();

        if route_results.len() > 0 {
            let mut best_route_result = &route_results[0];

            for i in 1..route_results.len() {
                let current_route_result = &route_results[i];
                if current_route_result.ref_profit_loss > best_route_result.ref_profit_loss {
                    best_route_result = current_route_result;
                }
            }

            if let Ok(transaction_data) =
                self.build_bundled_transaction(best_route_result, self.runtime_config)
            {
                let _transaction: TransactionRequest = TransactionRequest {
                    data: Some(transaction_data),
                    ..self.base_transaction.clone()
                };

                println!(
                    "calculated {} / {} routes in {:?} ({} WETH)",
                    route_results.len(),
                    RUNTIME_ROUTES.len(),
                    inst.elapsed(),
                    format_units(best_route_result.ref_profit_loss, 18).unwrap()
                );

                println!("{:#?}", _transaction);
            }
        }
    }

    #[inline(always)]
    fn build_bundled_transaction(
        &self,
        best_route_result: &RouteResult,
        runtime_config: &'static RuntimeConfig,
    ) -> Result<Bytes, AbiError> {
        let transactions = &best_route_result.transactions;

        let volume = best_route_result.start_balance;
        // let flash_loan_fee = price_oracle.get_flash_loan_fee() * volume / U256::from(10000);

        let mut targets: Vec<Address> = vec![];
        let mut payloads: Vec<Bytes> = vec![];

        for i in 0..transactions.len() {
            let transaction = &transactions[i];

            if i < transactions.len() - 1 {
                targets.push(transaction.value.market.value.contract_address);

                if let Ok(swap_data) = populate_swap(
                    &transaction.value,
                    &best_route_result.transactions[i + 1]
                        .value
                        .market
                        .value
                        .contract_address,
                ) {
                    payloads.push(swap_data);
                }
            } else {
                if let Ok(swap_data) =
                    populate_swap(&transaction.value, &runtime_config.executor_address)
                {
                    payloads.push(swap_data);
                }
            }
        }

        return ethers::contract::encode_function_data::<BundleExecutionCall>(
            &EXECUTE_TX_BUNDLE_FUNCTION,
            BundleExecutionCall {
                token: best_route_result.base_token.contract_address,
                amount_to_first_market: volume,
                targets,
                payloads,
            },
        );
    }
}
