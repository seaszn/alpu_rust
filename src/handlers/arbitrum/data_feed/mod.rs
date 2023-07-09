use ethers::providers::Middleware;
use ethers::types::H256;
use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::Sender;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::handlers::types::swap::BalanceChange;
use crate::{env, log_tracer};

use self::types::RelayMessage;

mod types;

pub async fn init(sender: &Sender<Vec<BalanceChange>>) -> websocket_lite::Result<()> {
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

async fn handle_text_message(incomming: Message, _sender: &Sender<Vec<BalanceChange>>) {
    if let Some(message_text) = incomming.as_text() {
        if let Some(relay_message) = RelayMessage::from_json(message_text) {
            let transaction_hashes: Vec<H256> = relay_message.decode();

            if transaction_hashes.len() > 0 {
                let client = env::RUNTIME_CACHE.client.clone();

                for tx_hash in transaction_hashes {
                    if let Ok(Some(mut transaction)) = client.get_transaction(tx_hash).await {
                        if let Some(_transaction_logs) =
                            log_tracer::trace_transaction(&mut transaction).await
                        {
                            // process::exit(1);
                        }
                    }
                    // if(s.is_ok()){
                    //     println!("{:?}", inst.elapsed());
                    //     // println!("{:#?}", s);
                    //     process::exit(1);
                    // }

                    // let transaction_request: ethers::types::TransactionRequest = tx.to_request();

                    // println!(" ---- {:#?}", tx.hash);
                    // if let Some(_transaction_logs) =
                    // log_tracer::trace_transaction(transaction_request.clone()).await
                    // {
                    //     // process::exit(1);
                    // }

                    // join_set.spawn(async move {s
                    //         if response.len() > 0 {
                    //             // println!("tx: {}", tx.hash);
                    //             // println!("address: {:#?}", response[0].address);
                    //             // println!(
                    //                 // "data: {:#?}",
                    //                 // H512::from_slice(response[0].raw.data.as_slice())
                    //             // );
                    //             // println!(
                    //                 // "data raw: {:#?}",
                    //                 // response[0].raw.data.as_slice()
                    //             // );

                    //             process::exit(1);
                    //         }

                    //         return Some(response);
                    //     } else {
                    //         return None;
                    //     }
                    // });
                }

                // // Get all the transaction logs of the
                // let mut combined_logs: Vec<TransactionLog> = vec![];
                // while let Some(Ok(result)) = join_set.join_next().await {
                //     if result.is_some() {
                //         combined_logs.append(result.unwrap().as_mut())
                //     }
                // }

                // if combined_logs.len() > 0 {
                //     let balance_changes: Vec<BalanceChange> = parse_balance_changes(combined_logs);

                //     println!("{:?}", timestamp.elapsed());
                //     _ = sender.send(balance_changes).await;
                // }
            }
        }
    }
}
