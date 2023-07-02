use ethers::abi::Address;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::types::Transaction;

mod data_feed;
mod decoder;
mod types;

const RAW_FROM_ADDRESS: &str = "0xf977814e90da44bfa03b6295a0616a897441acec";

lazy_static! {
    static ref FROM_ADDRESS: Address = RAW_FROM_ADDRESS.to_string().parse().unwrap();
}

pub async fn init() {
    let (sender, mut receiver): (Sender<Vec<Transaction>>, Receiver<_>) = channel(32);

    tokio::spawn(async move {
        _ = data_feed::init(sender).await;
    });

    while let Some(message) = receiver.recv().await {
        println!("tx {:#?}", message)
    }
}
