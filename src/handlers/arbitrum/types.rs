#![allow(non_snake_case)]

use ethers::types::Address;
use serde::Deserialize;

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
pub struct RelayMessage {
    pub version: u32,
    pub messages: Vec<InternalMessge>,
}

#[derive(Deserialize)]
pub struct InternalMessge {
    pub sequenceNumber: u32,
    pub message: InternalMessageData,
}

#[derive(Deserialize)]
pub struct InternalMessageData {
    pub message: MessageContent,
    pub delayedMessagesRead: u32,
}

#[derive(Deserialize)]
pub struct MessageContent {
    pub header: DataHeader,
    pub l2Msg: String,
}

#[derive(Deserialize)]
pub struct DataHeader {
    pub kind: u32,
    pub sender: Address,
    pub blockNumber: u32,
    pub timestamp: u32,
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
}
