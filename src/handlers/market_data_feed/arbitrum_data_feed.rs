use std::time::Instant;

use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env::{RuntimeCache, RuntimeConfig};
use crate::price_oracle::PriceOracle;
use crate::types::{BalanceChange, RelayMessage, TransactionDecodeResult};
use crate::{exchanges, log_tracer};

use super::MarketDataFeed;

pub struct ArbitrumDataFeed;

#[async_trait::async_trait]
impl MarketDataFeed for ArbitrumDataFeed {
    async fn init(
        &self,
        sender: Sender<Vec<BalanceChange>>,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
    ) -> websocket_lite::Result<()> {
        let builder: ClientBuilder = ClientBuilder::from_url(runtime_config.feed_endpoint.clone());
        let mut stream = builder.async_connect().await?;

        while let Some(msg) = stream.next().await {
            if let Ok(incomming) = msg {
                match incomming.opcode() {
                    Opcode::Text => handle_text_message(incomming, &sender, &runtime_cache).await,
                    Opcode::Ping => stream.send(Message::pong(incomming.into_data())).await?,
                    Opcode::Close => break,
                    Opcode::Pong | Opcode::Binary => {}
                }
            }
        }

        Ok(())
    }
}

#[inline(always)]
async fn handle_text_message(
    incomming: Message,
    sender: &Sender<Vec<BalanceChange>>,
    runtime_cache: &'static RuntimeCache,
) {
    if let Some(block) = PriceOracle::get_block_number().await {
        if let Some(message_text) = incomming.as_text() {
            if let Some(relay_message) = RelayMessage::from_json(message_text) {
                
                let decode_results: Vec<TransactionDecodeResult> = relay_message.decode(&block);

                if decode_results.len() > 0 {
                    let inst = Instant::now();
                    let mut call_set: JoinSet<Vec<BalanceChange>> = JoinSet::new();

                    // Itterate the transaction_hashes for balance changes
                    for mut decode_result in decode_results {
                        let block_clone = block.clone();
                        call_set.spawn(async move {
                            if decode_result.transaction.to.is_some() {
                                if let Some(transaction_logs) = log_tracer::trace_transaction(
                                    &mut decode_result.transaction,
                                    runtime_cache,
                                    block_clone,
                                )
                                .await
                                {
                                    if transaction_logs.len() > 0 {
                                        return exchanges::parse_balance_changes(
                                            &transaction_logs,
                                            runtime_cache,
                                        );
                                    }
                                }
                            }

                            return vec![];
                        });
                    }

                    // Get all the transaction logs of the
                    let mut balance_changes: Vec<BalanceChange> = vec![];
                    while let Some(Ok(mut changes)) = call_set.join_next().await {
                        if changes.len() > 0 {
                            balance_changes.append(&mut changes);
                        }
                    }

                    if balance_changes.len() > 0 {
                        println!(
                            "{} balance changes in: {:?}",
                            balance_changes.len(),
                            inst.elapsed()
                        );
                        _ = sender.send(balance_changes).await;
                    }
                }
            }
        }
    }
}
