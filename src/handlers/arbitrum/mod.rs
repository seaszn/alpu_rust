use std::{sync::Arc, time::Instant};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    env::*,
    types::{ReserveTable, Route, RouteResult},
};

use super::NetworkHandler;
mod data_feed;
pub struct ArbitrumHandler;

#[async_trait::async_trait]
impl NetworkHandler for ArbitrumHandler {
    async fn init(&self, runtime_config: Arc<RuntimeConfig>, runtime_cache: Arc<RuntimeCache>) {
        let (sender, mut receiver): (Sender<ReserveTable>, Receiver<_>) = channel(32);

        // start the data_feed
        let thread_config = runtime_config.clone();
        let thread_cache = runtime_cache.clone();
        _ = tokio::spawn(async move {
            _ = data_feed::init(sender, thread_config, thread_cache).await;
        });

        while let Some(reserve_table) = receiver.recv().await {
            if reserve_table.len() > 0 {

                let inst = Instant::now();

                for route in &runtime_cache.routes{

                }
                // let _route_results: Vec<RouteResult> = runtime_cache
                //     .routes
                //     .iter()
                //     .map(| element: &Route| {
                //         let res = element.calculate_result(&reserve_table);

                //         // return Some(res);
                //         return None;
                //     })
                //     .filter(|x| x.is_some())
                //     .map(|x| x.unwrap())
                //     .collect();

                println!("{:?}", inst.elapsed());
            }
        }
    }
}
