use std::time::Instant;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{env::*, types::{ OrganizedList, Reserves}};

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
        let (sender, mut receiver): (Sender<OrganizedList<Reserves>>, Receiver<_>) = channel(32);

        _ = tokio::spawn(async move {
            _ = data_feed::init(sender, runtime_config, runtime_cache).await;
        });

        while let Some(reserve_table) = receiver.recv().await {
            if reserve_table.len() > 0 {
                let inst = Instant::now();

                println!("{:?}", runtime_cache.markets[0].value.contract_address);
                println!("{:#?}", reserve_table[0].value);

                // for _route in &runtime_cache.routes {}
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
