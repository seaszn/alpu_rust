use std::time::Instant;

use ethers::providers::Middleware;
use ethers::types::H256;
use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env::types::RuntimeClient;
use crate::handlers::types::swap::BalanceChange;
use crate::{env, exchanges, log_tracer};

use self::types::RelayMessage;

mod types;

pub async fn init(sender: &Sender<Vec<BalanceChange>>) -> websocket_lite::Result<()> {
    let builder: ClientBuilder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream = builder.async_connect().await?;
    let client = env::RUNTIME_CACHE.client.clone();

    while let Some(msg) = stream.next().await {
        if let Ok(incomming) = msg {
            match incomming.opcode() {
                Opcode::Text => handle_text_message(incomming, &sender.clone(), &client).await,
                Opcode::Ping => stream.send(Message::pong(incomming.into_data())).await?,
                Opcode::Close => break,
                Opcode::Pong | Opcode::Binary => {}
            }
        }
    }

    Ok(())
}

#[inline(always)]
async fn handle_text_message(
    incomming: Message,
    sender: &Sender<Vec<BalanceChange>>,
    runtime_client: &RuntimeClient,
) {
    if let Some(message_text) = incomming.as_text() {
        if let Some(relay_message) = RelayMessage::from_json(message_text) {
            let transaction_hashes: Vec<H256> = relay_message.decode();

            if transaction_hashes.len() > 0 {
                let inst = Instant::now();
                let mut call_set: JoinSet<Vec<BalanceChange>> = JoinSet::new();

                for tx_hash in transaction_hashes {
                    let client: RuntimeClient = runtime_client.clone();

                    call_set.spawn(async move {
                        if let Ok(Some(mut transaction)) = client.get_transaction(tx_hash).await {
                            if transaction.to.is_some() {
                                if let Some(transaction_logs) =
                                    log_tracer::trace_transaction(&mut transaction).await
                                {
                                    if transaction_logs.len() > 0 {
                                        return exchanges::parse_balance_changes(transaction_logs);
                                    }
                                }
                            }
                        }

                        return vec![];
                    });
                }

                // // Get all the transaction logs of the
                let mut balance_changes: Vec<BalanceChange> = vec![];
                while let Some(Ok(mut result)) = call_set.join_next().await {
                    if result.len() > 0 {
                        balance_changes.append(&mut result)
                    }
                }

                if balance_changes.len() > 0 {
                    println!("found {} balance changes in {:#?}", balance_changes.len(), inst.elapsed());
                    _ = sender.send(balance_changes).await;
                }
            }
        }
    }
}
