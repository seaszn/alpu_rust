use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::types::Transaction;

mod data_feed;

pub async fn init() {
    let (sender, mut receiver): (Sender<Vec<Transaction>>, Receiver<_>) = channel(32);

    let _data_feed_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        _ = data_feed::init(sender).await;
    });

    while let Some(_balance_changes) = receiver.recv().await  {
        
    }
}
