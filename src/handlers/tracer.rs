use std::str::FromStr;

use crate::env;
use ethers::{
    abi::AbiEncode,
    prelude::*,
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, TransactionRequest,
    },
};
use serde_json::Value;

use super::types::TransactionLog;

extern crate lazy_static;

const JS_CONTENT: &str = "{\n
    data: [],\n
    fault: function (log) {\n
    },\n 
    
    step: function (log) {\n
       var topicCount = (log.op.toString().match(/LOG(\\d)/) || [])[1];\n
        if (topicCount) {\n
            const res = {\n
                address: log.contract.getAddress(),\n
                data: log.memory.slice(parseInt(log.stack.peek(0)), parseInt(log.stack.peek(0)) + parseInt(log.stack.peek(1))),\n
            };\n
            
            for (var i = 0; i < topicCount; i++){\n
                res[i.toString()] = log.stack.peek(i + 2);\n
            }\n

            this.data.push(res);\n
        }\n
    },\n
    result: function () {\n
        return this.data;\n
    }\n
}";

lazy_static! {
    static ref TYPE: GethDebugTracerType = GethDebugTracerType::JsTracer(JS_CONTENT.to_string());
    static ref OPTIONS: GethDebugTracingOptions = ethers::types::GethDebugTracingOptions {
        enable_memory: Some(true),
        enable_return_data: Some(true),
        disable_storage: Some(false),
        tracer: Some(TYPE.clone()),
        tracer_config: None,
        timeout: None,
        disable_stack: Some(false)
    };
    static ref CALL_OPTIONS: GethDebugTracingCallOptions = GethDebugTracingCallOptions {
        tracing_options: OPTIONS.clone(),
        state_overrides: None
    };
}

pub async fn trace_transaction_logs(tx: TransactionRequest) -> Option<Vec<TransactionLog>> {
    let client = env::RUNTIME_CACHE.client.clone();
    let response = client
        .debug_trace_call(
            tx,
            Some(BlockId::Number(BlockNumber::Latest)),
            CALL_OPTIONS.clone(),
        )
        .await;

    if response.is_ok() {
        return Some(decode_transaction_logs(response.unwrap()));
    } else {
        return None;
    }
}

fn decode_transaction_logs(trace: GethTrace) -> Vec<TransactionLog> {
    let GethTrace::Unknown(value) = trace else {return vec![]};
    let input: &Vec<serde_json::Value> = value.as_array().unwrap();
    let mut logs: Vec<TransactionLog> = vec![];
    let markets = env::RUNTIME_CACHE.market_addressess.clone();

    for obj in input {
        if obj.is_object() {
            let mut result: TransactionLog = TransactionLog {
                address: None,
                data: None,
                topics: vec![],
            };

            for (key, value) in obj.as_object().unwrap() {
                if key == "address" {
                    let address = buffer_to_hex(value);
                    if markets.contains(&H160::from_str(&address.as_str()).unwrap().0) {
                        result.address = Some(address);
                    }
                    else {
                        break;
                    }
                } else if key == "data" {
                    result.data = Some(buffer_to_hex(value));
                } else {
                    result.topics.push(
                        U256::from_dec_str(value.as_str().unwrap())
                            .unwrap()
                            .encode_hex(),
                    );
                }
            }

            if result.address.is_some() {
                logs.push(result);
            }
        }
    }

    return logs;
}

fn buffer_to_hex(value: &Value) -> String {
    let mut r: Vec<u8> = vec![];
    for (key, value) in value.as_object().unwrap() {
        let index: usize = key.parse().unwrap();
        let bytecode: u8 = value.as_u64().unwrap().to_string().parse().unwrap();

        if index >= r.len() {
            r.push(bytecode);
        } else {
            r.insert(index, bytecode)
        }
    }

    return format!("0x{}", hex::encode(r));
}
