use std::time::Instant;

use async_trait::async_trait;
use ethers::utils::format_units;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    env::*,
    price_oracle::PriceOracle,
    types::{OrganizedList, PriceTable, Reserves, RouteResult},
    RUNTIME_ROUTES,
};

use super::NetworkHandler;
mod data_feed;
pub struct ArbitrumHandler;

#[async_trait]
impl NetworkHandler for ArbitrumHandler {
    async fn init(
        &self,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
        price_oracle: &'static PriceOracle,
    ) {
        let (sender, mut receiver): (Sender<(OrganizedList<Reserves>, Vec<usize>)>, Receiver<_>) =
            channel(32);

        _ = tokio::spawn(async move {
            _ = data_feed::init(sender, runtime_config, runtime_cache).await;
        });

        let mut first_message = true;
        while let Some((reserve_table, market_ids)) = receiver.recv().await {
            
            // Skip the first message, the node's activity has not been whispered yet, so first message is often significantly delayed
            if !first_message {
                if reserve_table.len() > 0 {
                    let inst = Instant::now();
                    let price_table: &PriceTable = price_oracle.get_ref_price_table();

                    let route_results: Vec<RouteResult> = RUNTIME_ROUTES
                        .par_iter()
                        .filter_map(|x| {
                            return x.calculate_result(&reserve_table, price_table, &market_ids);
                        })
                        .collect();
                        	println!("t");

                    if route_results.len() > 0 {
                        let mut best_route_result = &route_results[0];

                        for i in 1..route_results.len() {
                            let current_route_result = &route_results[i];

                            if current_route_result.ref_profit_loss
                                > best_route_result.ref_profit_loss
                            {
                                best_route_result = current_route_result;
                            }
                        }

                        println!(
                            "calculated {} / {} routes in {:?} ({} WETH)",
                            route_results.len(),
                            RUNTIME_ROUTES.len(),
                            inst.elapsed(),
                            format_units(best_route_result.ref_profit_loss, 18).unwrap()
                        );
                    }
                }
            }
            else {
                println!("Validation received...\n");
                println!("Listening to market changes...\n");
                first_message = false;
            }
        }
    }
}
