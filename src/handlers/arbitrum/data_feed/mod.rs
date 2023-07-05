use std::time::Instant;
use std::vec;

use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env;
use crate::exchanges::parse_exchange_logs;
use crate::exchanges::types::Swap;
use crate::handlers::tracer;
use crate::types::{Transaction, TransactionLog};

use self::types::RelayMessage;

mod decoder;
mod types;

pub async fn init(sender: &Sender<Vec<Swap>>) -> websocket_lite::Result<()> {
    let builder: ClientBuilder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream = builder.async_connect().await?;

    while let Some(msg) = stream.next().await {
        if let Ok(incomming) = msg {
            match incomming.opcode() {
                Opcode::Text => handle_text_message(incomming, &sender.clone()).await,
                Opcode::Ping => stream.send(Message::pong(incomming.into_data())).await?,
                Opcode::Close => break,
                Opcode::Pong | Opcode::Binary => {}
            }
        }
    }

    Ok(())
}

async fn handle_text_message(incomming: Message, sender: &Sender<Vec<Swap>>) {
    let pase_result = RelayMessage::from_json(incomming.as_text().unwrap());
    if pase_result.is_some() {
        let transactions: Vec<Transaction> = pase_result.unwrap().decode();

        if transactions.len() > 0 {
            let timestamp = Instant::now();
            let mut join_set: JoinSet<Option<Vec<TransactionLog>>> = JoinSet::new();

            for tx in transactions {
                join_set.spawn(async move {
                    let response: Option<Vec<TransactionLog>> =
                        tracer::trace_transaction(tx.to_request()).await;

                    if response.is_some() {
                        return Some(response.unwrap());
                    } else {
                        return None;
                    }
                });
            }

            let mut combined_logs: Vec<TransactionLog> = vec![];
            while let Some(Ok(result)) = join_set.join_next().await {
                if result.is_some() {
                    combined_logs.append(result.unwrap().as_mut())
                }
            }

            if combined_logs.len() > 0 {
                let res: Vec<Swap> = parse_exchange_logs(combined_logs);

                println!("{:?}", timestamp.elapsed());
                _ = sender.send(res).await;
            }
        }
    }
}
