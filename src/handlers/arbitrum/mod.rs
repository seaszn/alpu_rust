use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::env::{*, types::RuntimeClient};

use super::types::swap::BalanceChange;

mod data_feed;

pub async fn init(
    runtime_config: RuntimeConfig,
    runtime_cache: Arc<RuntimeCache>
) {
    let (sender, mut receiver): (Sender<Vec<BalanceChange>>, Receiver<Vec<_>>) = channel(32);

    let _data_feed_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        _ = data_feed::init(sender,  runtime_config, runtime_cache).await;
    });

    while let Some(_balance_changes) = receiver.recv().await {
        if _balance_changes.len() > 0 {
            // println!("found {:#?} logs", _balance_changes);
            // println!("found {} logs", _balance_changes.len());
        }
    }
}
