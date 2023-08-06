use std::{thread, time::Instant};

use ethers::{
    prelude::AbiError,
    types::{Address, Bytes, TransactionRequest, U64},
    utils::format_units,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    env::{RuntimeCache, RuntimeConfig, EXECUTE_TX_BUNDLE_FUNCTION},
    exchanges::{init_exchange_handlers, populate_swap},
    networks::Network,
    price_oracle::PriceOracle,
    types::{BalanceChange, BundleExecutionCall, RouteResult},
    RUNTIME_ROUTES,
};

use super::{market_data_feed::get_network_data_feed, MarketDataFeed};

pub struct NetworkHandler {
    price_oracle: PriceOracle,
    runtime_config: &'static RuntimeConfig,
    runtime_cache: &'static RuntimeCache,
    data_feed: &'static (dyn MarketDataFeed + Send + Sync),
    _base_transaction: TransactionRequest,
}

impl NetworkHandler {
    #[inline(always)]
    pub fn from_network(
        network: &'static Network,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
    ) -> Option<NetworkHandler> {
        if let Some(data_feed) = get_network_data_feed(network.chain_id) {
            let price_oracle = PriceOracle::new(network, runtime_cache, runtime_config);

            return Some(NetworkHandler {
                runtime_config,
                runtime_cache,
                price_oracle,
                data_feed,
                _base_transaction: TransactionRequest {
                    from: Some(runtime_cache.client.address()),
                    to: Some(ethers::types::NameOrAddress::Address(
                        runtime_config.executor_address,
                    )),
                    gas: None,
                    gas_price: None,
                    value: None,
                    data: None,
                    chain_id: None,
                    nonce: None,
                },
            });
        }

        return None;
    }

    #[inline(always)]
    pub async fn init(&mut self) {
        init_exchange_handlers();
        self.price_oracle.initiate();

        let (sender, mut receiver): (Sender<(Vec<BalanceChange>, U64)>, Receiver<_>) = channel(32);

        let data_feed = self.data_feed;
        let config_reference = self.runtime_config;
        let cache_reference = self.runtime_cache;

        // initiate the data_feed
        let handle = tokio::runtime::Handle::current();
        thread::spawn(move || {
            handle.spawn(async move {
                _ = data_feed
                    .init(sender, config_reference, cache_reference)
                    .await;
            });

            let _guard = handle.enter();
        });

        let mut switch = true;
        while let Some((balance_changes, block_number)) = receiver.recv().await {
            if switch == true {
                switch = false;
                println!("Validation received...\n");
                println!("Listening to market updates...\n")
            } else {
                if balance_changes.len() > 0 {
                    self.handle_market_update(&balance_changes, &block_number)
                        .await;
                }
            }
        }
    }

    #[inline(always)]
    async fn handle_market_update(
        &self,
        balance_changes: &Vec<BalanceChange>,
        _block_number: &U64,
    ) {
        let inst = Instant::now();
        let mut reserve_table = self.price_oracle.get_market_reserves().await;

        for balance_change in balance_changes {
            let market_state = reserve_table[balance_change.market.id].value;
            reserve_table[balance_change.market.id].value =
                market_state.update_with_balance_change(balance_change);
        }

        /*
        // let inst = Instant::now();
        // let f: crate::types::OrganizedList<(ethers::types::U256, ethers::types::U256)> =
        // get_market_reserves(
            //     &self.runtime_cache.markets,
            //     self.runtime_cache,
            //     self.runtime_config,
            // )
            // .await;
            // println!("{:?}", inst.elapsed());

            // for i in 0..reserve_table.len() {
                //     let old_reserves = reserve_table.to_raw_vec()[i];
                //     let new_reserves = f.to_raw_vec()[i];

                //     if !old_reserves.value.0.eq(&new_reserves.value.0)
                //         || !old_reserves.value.1.eq(&new_reserves.value.1)
                //     {
                    //         println!(
                        //             "{:?}",
        //             self.runtime_cache.markets[old_reserves.id]
        //                 .value
        //                 .contract_address
        //         );
        //         println!("{:#?}", old_reserves);
        //         println!("{:#?}", new_reserves);
        //         println!(
            //             "{:#?}",
            //             balance_changes
            //                 .iter()
            //                 .find(|x| x.market.id == old_reserves.id)
            //         );
            //     }
            // }
            // if _reserve_table == f {
                //     println!("t");
                //     process::exit(1);
                // }
                */

        let market_ids: Vec<usize> = balance_changes.par_iter().map(|x| x.market.id).collect();
        let price_table = self.price_oracle.get_ref_price_table().await;

        let route_results: Vec<RouteResult> = RUNTIME_ROUTES
            .read()
            .unwrap()
            .par_iter()
            .filter_map(|x| {
                return x.calculate_result(&reserve_table, &price_table, &market_ids);
            })
            .collect();

        // for market_id in market_ids {
        //     println!("id: {}", market_id);
        //     println!("market {:#?}", self.runtime_cache.markets[*market_id]);
        //     println!("{:#?}", f[*market_id]);
        // }

        // process::exit(0);

        if route_results.len() > 0 {
            let mut best_route_result: &RouteResult = &route_results[0];

            for i in 1..route_results.len() {
                let current_route_result = &route_results[i];
                if current_route_result.ref_profit_loss > best_route_result.ref_profit_loss {
                    best_route_result = current_route_result;
                }
            }

            println!(
                "{} weth in {:?}",
                format_units(best_route_result.ref_profit_loss, 18).unwrap(),
                inst.elapsed()
            );

            // if let Ok(transaction_data) =
            //     self.build_bundled_transaction(best_route_result, self.runtime_config)
            // {
            //     let raw_transaction: TransactionRequest = TransactionRequest {
            //         data: Some(transaction_data),
            //         gas_price: Some(self.price_oracle.get_gas_price().await),
            //         ..self.base_transaction.clone()
            //     };

            //     let s = self
            //         .runtime_cache
            //         .client
            //         .estimate_gas(
            //             &TypedTransaction::Legacy(raw_transaction.clone()),
            //             Some(BlockId::Number(ethers::types::BlockNumber::Latest)),
            //         )
            //         .await;

            //     println!("{:#?}", s);
            //     println!(
            //         "{} weth",
            //         format_units(best_route_result.ref_profit_loss, 18).unwrap()
            //     );
            //     println!("{:?}", inst.elapsed());
            // }
        }
    }

    #[inline(always)]
    fn _build_bundled_transaction(
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
            targets.push(transaction.value.market.value.contract_address);

            if i < transactions.len() - 1 {
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
