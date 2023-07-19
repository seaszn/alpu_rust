use std::time::Instant;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    env::*,
    types::{OrganizedList, Reserves, RouteResult},
    RUNTIME_ROUTES,
};

use super::NetworkHandler;
mod data_feed;
pub struct ArbitrumHandler;

#[async_trait::async_trait]
impl NetworkHandler for ArbitrumHandler {
    async fn init(
        &self,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
    ) {
        let (sender, mut receiver): (Sender<(OrganizedList<Reserves>, Vec<usize>)>, Receiver<_>) =
            channel(32);

        _ = tokio::spawn(async move {
            _ = data_feed::init(sender, runtime_config, runtime_cache).await;
        });

        while let Some((reserve_table, market_ids)) = receiver.recv().await {
            if reserve_table.len() > 0 {
                let inst = Instant::now();
                let _result: Vec<RouteResult> = RUNTIME_ROUTES
                    .par_iter()
                    .filter_map(|x| return x.calculate_result(&reserve_table, &market_ids))
                    .collect();

                println!("{:?}", inst.elapsed());
                println!("({} / {})", _result.len(), RUNTIME_ROUTES.len());
            }
        }
    }
}
