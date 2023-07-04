use std::time::Instant;
use std::vec;

use base64::engine::general_purpose;
use base64::Engine;
use ethers::abi::RawLog;
use ethers::types::TransactionRequest;
use ethers::utils::rlp::DecoderError;
use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::env;
use crate::handlers::tracer;
use crate::types::Transaction;

use decoder::decode_transaction;

use self::types::RelayMessage;

mod decoder;
mod types;

pub async fn init(sender: &Sender<Vec<RawLog>>) -> websocket_lite::Result<()> {
    let builder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream = builder.async_connect().await?;

    while let Some(msg) = stream.next().await {
        if let Ok(incomming) = msg {
            match incomming.opcode() {
                // Incomming opcode
                Opcode::Text => {
                    let pase_result = RelayMessage::from_json(incomming.as_text().unwrap());
                    if pase_result.is_some() {
                        let transactions: Vec<Transaction> =
                            handle_incomming_data(&pase_result.unwrap());

                        if transactions.len() > 0 {
                            let timestamp = Instant::now();
                            let mut join_set: JoinSet<Option<Vec<RawLog>>> = JoinSet::new();
                            for tx in transactions {
                                join_set.spawn(async move {
                                    let request: TransactionRequest = TransactionRequest {
                                        from: tx.from,
                                        to: tx.to,
                                        value: tx.value,
                                        data: tx.data,
                                        gas: tx.gas,
                                        gas_price: tx.gas_price,
                                        nonce: tx.nonce,
                                        chain_id: tx.chain_id,
                                    };

                                    let response: Option<Vec<RawLog>> =
                                        tracer::trace_transaction_logs(request).await;

                                    if response.is_some() {
                                        return Some(response.unwrap());
                                    } else {
                                        return None;
                                    }
                                });
                            }

                            let mut combined_logs: Vec<RawLog> = vec![];
                            while let Some(Ok(result)) = join_set.join_next().await {
                                combined_logs.append(result.unwrap().as_mut())
                            }

                            if combined_logs.len() > 0 {
                                // decode all the logs here

                                println!("{:?}", timestamp.elapsed());
                                _ = sender.send(combined_logs).await;
                            }
                        }
                    }
                }

                // Functional opcodes
                Opcode::Ping => stream.send(Message::pong(incomming.into_data())).await?,
                Opcode::Close => {
                    break;
                }
                Opcode::Pong | Opcode::Binary => {}
            }
        }
    }

    Ok(())
}

fn handle_incomming_data(message: &RelayMessage) -> Vec<Transaction> {
    let mut result: Vec<Transaction> = Vec::new();
    if message.messages.len() > 0 {
        for message in &message.messages {
            if message.message.message.header.kind == types::L1MessageType::L2Message as u32 {
                let data = general_purpose::GeneralPurpose::decode(
                    &general_purpose::STANDARD,
                    &message.message.message.l2_message,
                )
                .unwrap();

                let (message_kind, message_data) = data.split_first().unwrap();

                if i8::from_be_bytes([*message_kind]) == types::L2MessageType::SignedTx as i8 {
                    let transaction: Result<Transaction, DecoderError> =
                        decode_transaction(message_data);

                    if transaction.is_ok() {
                        result.push(transaction.unwrap());
                    }
                }
            }
        }
    }

    return result;
}
