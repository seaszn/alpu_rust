use base64::engine::general_purpose;
use base64::Engine;
use ethers::types::TransactionRequest;
use ethers::utils::rlp::DecoderError;
use ethers::{
    types::transaction::eip1559,
    utils::{hex, keccak256, rlp},
};
use futures::{SinkExt, StreamExt};

use websocket_lite::{ClientBuilder, Message, Opcode};

use crate::{env, handlers::arbitrum::types::RelayMessage};

use super::types;

pub async fn init() -> websocket_lite::Result<()> {
    let builder = ClientBuilder::from_url(env::RUNTIME_CONFIG.feed_endpoint.clone());
    let mut stream = builder.async_connect().await?;

    while let Some(msg) = stream.next().await {
        if let Ok(m) = msg {
            match m.opcode() {
                Opcode::Text => {
                    let result = RelayMessage::from_json(m.as_text().unwrap());
                    if result.is_some() {
                        handle_relay_message(&result.unwrap());
                    }
                }
                Opcode::Ping => stream.send(Message::pong(m.into_data())).await?,
                Opcode::Close => {
                    break;
                }
                Opcode::Pong | Opcode::Binary => {}
            }
        }
    }

    Ok(())
}

fn handle_relay_message(message: &RelayMessage) {
    if message.messages.len() > 0 {
        for message in &message.messages {
            if message.message.message.header.kind == types::L1MessageType::L2Message as u32 {
                let data = general_purpose::GeneralPurpose::decode(
                    &general_purpose::STANDARD,
                    &message.message.message.l2Msg,
                )
                .unwrap();

                let (message_kind, message_data) = data.split_first().unwrap();

                if i8::from_be_bytes([*message_kind]) == types::L2MessageType::SignedTx as i8 {
                    let _ = decode_tx(message_data);
                }
            }
        }
    }
}

fn decode_tx(data: &[u8]) {
    let _tx_hash = hex::encode(keccak256(data));

    let legacy_transaction: Result<TransactionRequest, DecoderError> = rlp::decode(data);
    if legacy_transaction.is_ok() {
        println!("legacy transaction")
        // build the transaction
    }

    let eip1559_transaction: Result<eip1559::Eip1559TransactionRequest, DecoderError> =
        rlp::decode(data.split_first().unwrap().1);
    if eip1559_transaction.is_ok() {
        // build the eip-1559 transaction
        println!("eip-1559 transaction")
    }
}
