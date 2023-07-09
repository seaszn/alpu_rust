use std::{ops::Sub, process, time::Instant};

use crate::{
    env::{self, types::RuntimeClient},
    types::{market::Market, TransactionLog},
};
use ethers::{
    providers::Middleware,
    types::{
        BlockId, BlockNumber, Bytes, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, NameOrAddress, Transaction, TransactionRequest, H256,
    },
};

use self::{
    types::LogFrame,
    utils::{parse_address, parse_number_array, parse_topic_buffer},
};

mod types;
mod utils;

const JS_CONTENT: &str = "{
    data: [],
    fault: function (log) {
    },
    
    step: function (log) {
       var topicCount = (log.op.toString().match(/LOG(\\d)/) || [])[1];
        if (topicCount) {
            const parseData = (data) => {
                const res = [];
                const len = Object.keys(data).length;

                for(i = 0; i < len; i++){
                    res.push(data[i.toString()]);
                }

                return res;
            }

            const res = {
                address: parseData(log.contract.getAddress()),
                data: parseData(log.memory.slice(parseInt(log.stack.peek(0)), parseInt(log.stack.peek(0)) + parseInt(log.stack.peek(1)))),
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

pub async fn trace_transaction(tx: &mut Transaction) -> Option<Vec<TransactionLog>> {
    let client: RuntimeClient = env::RUNTIME_CACHE.client.clone();
    let inst = Instant::now();

    let typed = TransactionRequest {
        from: Some(tx.from),
        to: Some(NameOrAddress::Address(tx.to.unwrap())),
        gas: Some(tx.gas),
        gas_price: Some(tx.gas_price.unwrap()),
        value: Some(tx.value),
        data: Some(tx.input.clone()),
        nonce: None,
        chain_id: None,
    };

    if let Ok(geth_trace) = client
        .debug_trace_call(
            typed,
            Some(BlockId::Number(BlockNumber::Number(
                tx.block_number.unwrap().sub(1),
            ))),
            CALL_OPTIONS.clone(),
        )
        .await
    {
        if let GethTrace::Unknown(trace_container) = geth_trace {
            if let Some(raw_traces) = trace_container.as_array() {
                if raw_traces.len() > 0 {
                    for trace_object in raw_traces {
                        if let Some(trace) = trace_object.as_object() {
                            let call_address = parse_address(trace["address"].clone());
                            if let Some(_market) = Market::from_address(&call_address) {
                                let data = Bytes::from(parse_number_array(trace["data"].clone()));
                                let mut topics: Vec<H256> = vec![];

                                for i in 0..trace.len() - 2 {
                                    if let Some(topic) = parse_topic_buffer(&trace[&i.to_string()])
                                    {
                                        topics.push(topic);
                                    }
                                }

                                if topics.len() > 1 {
                                    let _f: LogFrame = LogFrame {
                                        address: call_address,
                                        data,
                                        topics,
                                    };

                                    println!("{:?}", inst.elapsed());
                                    process::exit(1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return None;
}
