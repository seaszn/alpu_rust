use crate::env;
use crate::handlers::arbitrum::data_feed::types::TransactionLog;
use ethers::{
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

pub async fn trace_transaction_logs(
    tx: TransactionRequest,
    hash: String,
) -> Option<Vec<TransactionLog>> {
    let client = env::RUNTIME_CACHE.client.clone();
    let response = client
        .debug_trace_call(
            tx,
            Some(BlockId::Number(BlockNumber::Latest)),
            CALL_OPTIONS.clone(),
        )
        .await;

    if response.is_ok() {
        return Some(decode_transaction_logs(response.unwrap(), hash));
    } else {
        return None;
    }
}

fn decode_transaction_logs(trace: GethTrace, hash: String) -> Vec<TransactionLog> {
    let GethTrace::Unknown(value) = trace else {return vec![]};
    let input: &Vec<serde_json::Value> = value.as_array().unwrap();
    let mut logs: Vec<TransactionLog> = vec![];

    for obj in input {
        if obj.is_object() {
            let mut result = TransactionLog {
                address: "".to_string(),
                data: "".to_string(),
                topics: vec![],
            };

            for (key, value) in obj.as_object().unwrap() {
                if key == "address" {
                    result.address = buffer_to_hex(value);
                } else if key == "data" {
                    result.data = buffer_to_hex(value);
                } else {
                    // Decode and add topics
                }
            }

            logs.push(result);
        }
    }

    return logs;
}

fn buffer_to_hex(value: &Value) -> String {
    let mut r: Vec<u8> = vec![];
    for (key, value) in value.as_object().unwrap() {
        let index: usize = key.to_string().parse().unwrap();
        let bytecode: u8 = value.as_u64().unwrap().to_string().parse().unwrap();

        if index >= r.len() {
            r.push(bytecode);
        } else {
            r.insert(index, bytecode)
        }
    }

    return format!("0x{}", hex::encode(r));
}

// fn string_to_hex(value: &Value) -> &str {
//     return value.as_str().unwrap();
// }
