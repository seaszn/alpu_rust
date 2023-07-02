use base64::engine::general_purpose;
use base64::Engine;
use ethers::utils::rlp::DecoderError;
use futures::{SinkExt, StreamExt};

use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;
use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::types::Transaction;
use crate::{env, handlers::arbitrum::types::RelayMessage};

use decoder::decode_transaction;
use super::types;

mod decoder;
pub mod tracer;

pub async fn init(sender: Sender<Vec<Transaction>>) -> websocket_lite::Result<()> {
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
                            let result: Result<(), SendError<Vec<Transaction>>> =
                                sender.send(transactions).await;

                            if result.is_err() {
                                println!("{:?}", result.unwrap())
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

// #[time()]
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
