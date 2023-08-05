use ethers::types::{Transaction, U64};
use futures::{SinkExt, StreamExt};
use std::time::Instant;

use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env::{RuntimeCache, RuntimeConfig};
use crate::types::{BalanceChange, RelayMessage, TransactionDecodeResult};
use crate::{exchanges, log_tracer, price_oracle};

use super::MarketDataFeed;

pub struct ArbitrumDataFeed;

#[async_trait::async_trait]
impl MarketDataFeed for ArbitrumDataFeed {
    async fn init(
        &self,
        sender: Sender<(Vec<BalanceChange>, U64)>,
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
    sender: &Sender<(Vec<BalanceChange>, U64)>,
    runtime_cache: &'static RuntimeCache,
) {
    if let Some(message_text) = incomming.as_text() {
        if let Some(relay_message) = RelayMessage::from_json(message_text) {
            let decoded_transactions: Vec<TransactionDecodeResult> = relay_message.decode();
            
            if decoded_transactions.len() > 0 {
                let inst = Instant::now();
                let block_number = price_oracle::PriceOracle::get_block_number();
                let mut balance_changes: Vec<BalanceChange> = vec![];
                
                let mut call_set: JoinSet<Vec<BalanceChange>> = JoinSet::new();
                
                for decoded in decoded_transactions {
                    call_set.spawn(async move {
                        if decoded.transaction.to.is_some() {
                            if let Some(transaction_logs) = log_tracer::trace_transaction(
                                Transaction {
                                    block_number: Some(block_number),
                                    ..decoded.transaction
                                },
                                &runtime_cache,
                            )
                            .await
                            {
                                if transaction_logs.len() > 0 {
                                    return exchanges::parse_balance_changes(
                                        &transaction_logs,
                                        &runtime_cache,
                                    );
                                }
                            }
                        }

                        return vec![];
                    });
                }

                while let Some(Ok(mut changes)) = call_set.join_next().await {
                    if changes.len() > 0 {
                        balance_changes.append(&mut changes);
                    }
                }

                if balance_changes.len() > 0 {
                    println!(
                        "handled {} balances changes in: {:?}",
                        balance_changes.len(),
                        inst.elapsed()
                    );

                    _ = sender.send((balance_changes, block_number)).await;
                }
            }
        }
    }
}
