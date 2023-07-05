use base64::engine::general_purpose;
use ethers::types::Address;
use ethers::utils::rlp::DecoderError;
use serde;
use serde::Deserialize;
extern crate base64;
use base64::Engine;

use crate::types::Transaction;

use super::decoder::decode_data;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum L1MessageType {
    L2Message = 3,
    EndOfBlock = 6,
    L2FundedByL1 = 7,
    RollupEvent = 8,
    SubmitRetryable = 9,
    BatchForGasEstimation = 10,
    Intiialize = 11,
    EthDeposit = 12,
    BatchPostingReport = 13,
    Invalid = 0xff,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum L2MessageType {
    UnsignedUserTx = 0,
    ContractTx = 1,
    NonmutaticCall = 2,
    Batch = 3,
    SignedTx = 4,
    Heartbeat = 6, // Deprecated
}

#[derive(Deserialize)]
pub struct InternalMessge {
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u32,
    pub message: InternalMessageData,
}

#[derive(Deserialize)]
pub struct InternalMessageData {
    pub message: MessageContent,
    #[serde(rename = "delayedMessagesRead")]
    pub delayed_messages_read: u32,
}

#[derive(Deserialize)]
pub struct MessageContent {
    pub header: DataHeader,
    #[serde(rename = "l2Msg")]
    pub l2_message: String,
}

#[derive(Deserialize)]
pub struct DataHeader {
    pub kind: u32,
    pub sender: Address,
    #[serde(rename = "blockNumber")]
    pub block_number: u32,
    pub timestamp: u32,
}

#[derive(Deserialize)]
pub struct RelayMessage {
    pub version: u32,
    pub messages: Vec<InternalMessge>,
}

impl RelayMessage {
    pub fn from_json(input: &str) -> Option<RelayMessage> {
        let result: Result<RelayMessage, serde_json::Error> = serde_json::from_str(&input);
        if result.is_ok() {
            return Option::Some(result.unwrap());
        } else {
            return Option::None;
        }
    }

    pub fn decode(&self) -> Vec<Transaction> {
        let mut result: Vec<Transaction> = Vec::new();
        if self.messages.len() > 0 {
            for message in &self.messages {
                if message.message.message.header.kind == L1MessageType::L2Message as u32 {
                    let data = general_purpose::GeneralPurpose::decode(
                        &general_purpose::STANDARD,
                        &message.message.message.l2_message,
                    )
                    .unwrap();

                    let (message_kind, message_data) = data.split_first().unwrap();

                    if i8::from_be_bytes([*message_kind]) == L2MessageType::SignedTx as i8 {
                        let transaction: Result<Transaction, DecoderError> =
                            decode_data(message_data);

                        if transaction.is_ok() {
                            result.push(transaction.unwrap());
                        }
                    }
                }
            }
        }

        return result;
    }
}
