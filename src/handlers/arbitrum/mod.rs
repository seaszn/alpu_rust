use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::exchanges::types::Swap;

mod data_feed;

pub async fn init() {
    let (sender, mut receiver): (Sender<Vec<Swap>>, Receiver<Vec<_>>) = channel(32);

    let _data_feed_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        _ = data_feed::init(&sender).await;
    });

    while let Some(_balance_changes) = receiver.recv().await {
        if _balance_changes.len() > 0 {
            println!("found {:#?} logs", _balance_changes);
            println!("found {} logs", _balance_changes.len());
        }
    }
}
