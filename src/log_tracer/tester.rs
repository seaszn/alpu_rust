use std::{process, time::Instant};

use crate::{
    env::{self, types::RuntimeClient},
    types::TransactionLog,
};
use ethers::{
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingCallOptions, GethDebugTracingOptions, TransactionRequest,
    },
};

const CALL_OPTIONS: GethDebugTracingCallOptions = GethDebugTracingCallOptions {
    tracing_options: GethDebugTracingOptions {
        enable_memory: Some(true),
        enable_return_data: Some(true),
        disable_storage: Some(false),
        tracer: Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        )),
        tracer_config: None,
        timeout: None,
        disable_stack: Some(false),
    },
    state_overrides: None,
};

pub async fn trace_transaction(tx: TransactionRequest) -> Option<Vec<TransactionLog>> {
    let client: RuntimeClient = env::RUNTIME_CACHE.client.clone();
    let inst = Instant::now();

    if let Ok(_response) = client
        .debug_trace_call(tx, Some(BlockId::Number(BlockNumber::Latest)), CALL_OPTIONS)
        .await
    {
        // println!("{:#?}", response.unwrap());
    }

    println!("{:#?}", inst.elapsed());
    process::exit(0);

    // return None;
}

// fn decode_trace_logs(trace: &GethTrace) {
//     if let GethTrace::Unknown(trace_value) = trace {
//         if let Some(raw_traces) = trace_value.as_array() {
//             'trace_loop: for trace in raw_traces {
//                 if let Some(trace_value) = trace.as_object() {
//                     let address = parse_address(&trace_value["address"]);
//                     if let Some(market) = from_address(&address) {
//                         if let Some(trace_data) = parse_data(&trace_value["data"]) {
//                             // let mut raw_log = RawLog{
//                             //     data: trace_data,
//                             //     topics: vec![]
//                             // };

//                             // for (key, value) in trace_value{
//                             //     if key != "address" && key != "data"{
//                             //         if let Some(topic_hash) = parse_topic_buffer(value){
//                             //             raw_log.topics.push(topic_hash);
//                             //         }
//                             //         else {
//                             //             continue 'trace_loop;
//                             //         }
//                             //     }
//                             // }

//                             println!("address: {}", market.contract_address);
//                             // println!("{:#?}", raw_log);
//                             // println!("{}", raw_log.data.encode_hex::<String>());
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

/*
fn decode_trace_logs(trace: GethTrace, hash: String) -> Vec<TransactionLog> {
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
                        if !value.is_object() {
                            continue 'input_loop;
                        } else {
                            raw_log.data = parse_buffer(value).to_vec();
                        }
                    } else if key != "address" {
                        if value.is_string() {
                            if let Some(topic_data) = parse_topic_buffer(value) {
                                raw_log.topics.push(topic_data);
                            } else {
                                continue 'input_loop;
                            }
                        } else {
                            continue 'input_loop;
                        }
                    }
                }

                // println!("{:#?}", test(raw_log.data.clone()));
                println!("raw data: {:?}", raw_log.data);
                println!("data: {:?}", H512::from_slice(raw_log.data.as_slice()));
                println!("data 2: 0{}", test(raw_log.data));

                println!("topics: {:?}", raw_log.topics);
                println!("hash : {}", hash);
                println!("address : {}", market.contract_address);

                process::exit(1);

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

fn test(input: Vec<u8>) -> String {
    let mut result: String = "".to_string();

    let f = "".to_string();

    for i in input {
        result.push_str(format_radix(i as u32, 16).as_str());
    }

    return result;
    // let s = result.into_iter().flatten().collect();
}
 */
