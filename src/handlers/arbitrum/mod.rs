use std::time::Instant;

use ethers::types::{GethTrace, TransactionRequest};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::types::Transaction;

// use crate::{handlers::arbitrum::tracer::trace_transaction_logs, types::Transaction};

mod data_feed;
mod types;

pub async fn init() {
    let (sender, mut receiver): (Sender<Vec<Transaction>>, Receiver<_>) = channel(32);

    let _data_feed_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        _ = data_feed::init(sender).await;
    });

    while let Some(message) = receiver.recv().await {
        // // println!("tx {:#?}", message)
        for tx in message {
            let request = TransactionRequest {
                from: tx.from,
                to: tx.to,
                value: tx.value,
                data: tx.data,
                gas: tx.gas,
                gas_price: tx.gas_price,
                nonce: tx.nonce,
                chain_id: tx.chain_id,
            };

            let start = Instant::now();
            let response: Option<GethTrace> =
                data_feed::tracer::trace_transaction_logs(request).await;

            if response.is_some() {
                println!("{:#?}", start.elapsed());
            }
        }
    }
}
