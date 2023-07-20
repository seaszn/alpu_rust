use std::time::Instant;

use async_trait::async_trait;
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

        while let Some((reserve_table, market_ids)) = receiver.recv().await {
            if reserve_table.len() > 0 {
                let inst = Instant::now();
                let price_table: &PriceTable = price_oracle.get_ref_price_table();
                
                let _result: Vec<RouteResult> = RUNTIME_ROUTES
                    .par_iter()
                    .filter_map(|x| {
                        return x.calculate_result(&reserve_table, price_table, &market_ids);
                    })
                    .collect();

                // if _result.len() > 0{
                    println!("{:?}", inst.elapsed());
                    println!("({} / {})", _result.len(), RUNTIME_ROUTES.len());
                // }
            }
        }
    }
}
