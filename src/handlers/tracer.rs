use crate::{
    env::{self, types::RuntimeClient},
    types::{self, TransactionLog},
};
use ethers::{
    abi::RawLog,
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, TransactionRequest,
    },
};

use super::utils::{parse_address, parse_buffer, parse_topic_buffer};

extern crate lazy_static;

const JS_CONTENT: &str = "{
    data: [],
    fault: function (log) {
    },
    
    step: function (log) {
       var topicCount = (log.op.toString().match(/LOG(\\d)/) || [])[1];
        if (topicCount) {
            const res = {
                address: log.contract.getAddress(),
                data: log.memory.slice(parseInt(log.stack.peek(0)), parseInt(log.stack.peek(0)) + parseInt(log.stack.peek(1))),
            };
            
            for (var i = 0; i < topicCount; i++){
                res[i.toString()] = log.stack.peek(i + 2);
            }

            this.data.push(res);
        }
    },

    result: function () {
        return this.data;
    }
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

pub async fn trace_transaction(tx: TransactionRequest) -> Option<Vec<TransactionLog>> {
    let client: RuntimeClient = env::RUNTIME_CACHE.client.clone();
    let response = client
        .debug_trace_call(
            tx,
            Some(BlockId::Number(BlockNumber::Latest)),
            CALL_OPTIONS.clone(),
        )
        .await;

    if response.is_ok() {
        return Some(decode_trace(response.unwrap()));
    } else {
        return None;
    }
}

fn decode_trace(trace: GethTrace) -> Vec<TransactionLog> {
    let GethTrace::Unknown(value) = trace else {return vec![]};
    let input_array: &Vec<serde_json::Value> = value.as_array().unwrap();
    let mut transaction_logs: Vec<TransactionLog> = vec![];

    'input_loop: for input_element in input_array {
        if input_element.is_object() {
            let element_obj: &serde_json::Map<String, serde_json::Value> =
                input_element.as_object().unwrap();
            let address: &ethers::types::H160 = &parse_address(&element_obj["address"]);

            if let Some(market) = types::market::from_address(address) {
                let mut raw_log: RawLog = RawLog {
                    data: vec![],
                    topics: vec![],
                };

                for (key, value) in input_element.as_object().unwrap() {
                    if key == "data" {
                        if value.is_object() {
                            raw_log.data = parse_buffer(value);
                        } else {
                            continue 'input_loop;
                        }
                    } else if key != "address" {
                        let topic_data: Option<ethers::types::H256> = parse_topic_buffer(value);

                        if topic_data.is_some() && value.is_string() {
                            raw_log.topics.push(topic_data.unwrap());
                        } else {
                            continue 'input_loop;
                        }
                    }
                }

                if raw_log.data.len() > 0 {
                    transaction_logs.push(TransactionLog {
                        address: *address,
                        protocol: market.protocol,
                        raw: raw_log,
                    });
                }
            }
        }
    }
    return transaction_logs;
}
