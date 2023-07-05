use crate::{
    env,
    types::{self, TransactionLog},
};
use ethers::{
    abi::{AbiEncode, RawLog},
    prelude::*,
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, TransactionRequest,
    },
};
use serde_json::Value;

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

    'input_loop: for obj in input {
        if obj.is_object() {
            let mut result: RawLog = RawLog {
                topics: vec![],
                data: vec![],
            };

            let object = obj.as_object().unwrap();
            let address = &parse_address(&object["address"]);

            if let Some(market) = types::market::from_address(address) {
                for (key, value) in obj.as_object().unwrap() {
                    if key == "data" {
                        if value.is_object() {
                            result.data = sort_buffer(value);
                        } else {
                            continue 'input_loop;
                        }
                    } else if key != "address" {
                        let topic_data = parse_topic_buffer(value);

                        if topic_data.is_some() && value.is_string() {
                            result.topics.push(topic_data.unwrap());
                        } else {
                            continue 'input_loop;
                        }
                    }
                }

                if result.data.len() > 0 {
                    logs.push(TransactionLog {
                        address: *address,
                        protocol: market.protocol,
                        raw: result,
                    });
                }
            }
        }
    }
    return logs;
}

fn sort_buffer(value: &Value) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![];
    for (key, value) in value.as_object().unwrap() {
        let index: usize = key.parse().unwrap();
        let bytecode: u8 = value.as_u64().unwrap().to_string().parse().unwrap();

        if index >= buffer.len() {
            buffer.push(bytecode);
        } else {
            buffer.insert(index, bytecode)
        }
    }

    return buffer;
}

fn parse_address(value: &Value) -> H160 {
    return H160::from_slice(&sort_buffer(value));
}

fn parse_topic_buffer(value: &Value) -> Option<H256> {
    let s: Result<U256, abi::ethereum_types::FromDecStrErr> =
        U256::from_dec_str(value.as_str().unwrap());

    if s.is_ok() {
        return Some(H256::from_slice(s.unwrap().encode().as_slice()));
    }

    return None;
}
