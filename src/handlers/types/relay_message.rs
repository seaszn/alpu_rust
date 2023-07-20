use base64::engine::general_purpose;
use ethers::types::{Address, H256};
use ethers::utils::keccak256;
use serde;
use serde::Deserialize;
extern crate base64;
use base64::Engine;

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
pub struct InternalMessage {
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
    pub messages: Vec<InternalMessage>,
}

impl RelayMessage {
    #[inline(always)]
    pub fn from_json(input: &str) -> Option<RelayMessage> {
        match serde_json::from_str(&input) {
            Ok(res) => {
                return Some(res);
            }
            Err(_) => {
                return None;
            }
        }
    }

    #[inline(always)]
    pub fn decode(&self) -> Vec<H256> {
        if self.messages.len() > 0 {
            let mut result: Vec<H256> = vec![];

            for message in &self.messages{
                if let Ok(data) = general_purpose::STANDARD.decode(&message.message.message.l2_message){
                    let (message_kind, message_data) = data.split_first().unwrap();
                    if i8::from_be_bytes([*message_kind]) == L2MessageType::SignedTx as i8 {
                        result.push(H256::from(keccak256(message_data)))
                    }
                }
            }

            return  result;
        }

        return vec![];
    }
}
