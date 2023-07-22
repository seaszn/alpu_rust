use std::process;
use std::time::Instant;

use ethers::providers::Middleware;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt};

use ethers::prelude::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env::{RuntimeCache, RuntimeConfig};
use crate::exchanges::get_market_reserves;
use crate::types::{BalanceChange, OrganizedList, RelayMessage, Reserves};
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
                    Opcode::Text => {
                        handle_text_message(incomming, &sender, &runtime_config, &runtime_cache)
                            .await
                    }
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
    runtime_config: &'static RuntimeConfig,
    runtime_cache: &'static RuntimeCache,
) {
    if let Some(message_text) = incomming.as_text() {
        if let Some(relay_message) = RelayMessage::from_json(message_text) {
            let transaction_hashes: Vec<H256> = relay_message.decode();

            if transaction_hashes.len() > 0 {
                let inst = Instant::now();
                let mut call_set: JoinSet<Vec<BalanceChange>> = JoinSet::new();
                
                // Itterate the transaction_hashes for balance changes
                for tx_hash in transaction_hashes {
                    call_set.spawn(async move {
                        if let Ok(Some(mut transaction)) =
                            runtime_cache.client.get_transaction(tx_hash).await
                        {
                            if transaction.to.is_some() {
                                if let Some(transaction_logs) =
                                    log_tracer::trace_transaction(&mut transaction, runtime_cache)
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

                // if let Some(mut reserves) = market_reserves {
                //     if balance_changes.len() > 0 && reserves.len() > 0 {
                //         // let mut changed_market_ids: Vec<usize> = vec![];
                //         for change in balance_changes {
                //             // println!("{:#?}", change.tx_hash);
                //             println!("www.arbiscan.io/tx/{:?}", change.tx_hash);
                //             println!("market address: {:?}", change.market.value.contract_address);
                //             println!("old reserves: \n{:#?}", reserves[change.market.id]);

                //             // tx hash
                //             // market contract address
                //             // old
                //             // new with balance changes

                //             let mut f = reserves[change.market.id];
                //             f.value.0 = (f.value.0 + change.amount_0_in) - change.amount_0_out;
                //             f.value.1 = (f.value.1 + change.amount_1_in) - change.amount_1_out;
                //             println!("new reserves: \n{:#?}", f);

                //             println!("change: \n{:#?}", change);

                //             // changed_market_ids.push(change.market.id);
                //             // reserves.update_value_at(change.market.id, |x| {
                //             //     x.value.0 = x.value.0 + change.amount_0_in - change.amount_0_out;
                //             //     x.value.1 = x.value.1 + change.amount_1_in - change.amount_1_out;
                //             // });
                //             process::exit(0);
                //         }

                if balance_changes.len() > 0 {

                    println!("{} balance changes in: {:?}", balance_changes.len(), inst.elapsed());
                    // _ = sender.send(f).await;
                    _ = sender.send(balance_changes).await;
                }
                else {
                    println!("empty");
                }
            }
        }
    }
}
