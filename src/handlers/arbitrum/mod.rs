use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::env::*;

use super::{types::BalanceChange, NetworkHandler};
mod data_feed;
pub struct ArbitrumHandler;

#[async_trait::async_trait]
impl NetworkHandler for ArbitrumHandler {
    async fn init(&self, runtime_config: Arc<RuntimeConfig>, runtime_cache: Arc<RuntimeCache>) {
        let (sender, mut receiver): (Sender<Vec<BalanceChange>>, Receiver<Vec<_>>) = channel(32);

        // start the data_feed
        _ = tokio::spawn(async move {
            _ = data_feed::init(sender, runtime_config, runtime_cache).await;
        });

        while let Some(_balance_changes) = receiver.recv().await {
            if _balance_changes.len() > 0 {
                println!("found {:#?} logs", _balance_changes);
                // println!("found {} logs", _balance_changes.len());
            }
        }
    }
}
