use std::ops::Sub;

use crate::{
    env::RuntimeCache,
    types::{market::Market, TransactionLog},
};
use ethers::{
    abi::RawLog,
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, NameOrAddress, Transaction, TransactionRequest,
    },
};

use self::utils::{parse_address, parse_buffer, parse_topic_buffer};

mod types;
mod utils;

const JS_CONTENT: &str = "{
    data: [],
    fault: function (log) {
    },
    step: function (log) {
       var topicCount = (log.op.toString().match(/LOG(\\d)/) || [])[1];
        if (topicCount) {
            const peek_0 = parseInt(log.stack.peek(0));
            const res = {
                address: log.contract.getAddress(),
                data: log.memory.slice(peek_0, peek_0 + parseInt(log.stack.peek(1))),
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

pub fn init(){
    let _ = CALL_OPTIONS.state_overrides;
}

#[inline(always)]
pub async fn trace_transaction(
    tx: &mut Transaction,
    runtime_cache: &'static RuntimeCache,
) -> Option<Vec<TransactionLog>> {
    // get the transaction traces
    if let Ok(geth_trace) = runtime_cache
        .client
        .debug_trace_call(
            TransactionRequest {
                from: Some(tx.from),
                to: Some(NameOrAddress::Address(tx.to.unwrap())),
                gas: Some(tx.gas),
                gas_price: tx.gas_price,
                value: Some(tx.value),
                data: Some(tx.input.clone()),
                nonce: None,
                chain_id: None,
            },
            Some(BlockId::Number(BlockNumber::Number(
                tx.block_number.unwrap().sub(1),
            ))),
            CALL_OPTIONS.clone(),
        )
        .await
    {
        // Check if the result is valid
        if let GethTrace::Unknown(trace_container) = geth_trace {
            // All raw traces should be contained in an array
            if let Some(raw_traces) = trace_container.as_array() {
                // Only itterate if length > 0
                if raw_traces.len() > 0 {
                    let mut transaction_logs: Vec<TransactionLog> = vec![];

                    // decode all the raw traces
                    for trace_object in raw_traces {
                        if let Some(trace) = trace_object.as_object() {
                            let call_address = parse_address(&trace["address"]);

                            // If this a tracked market, decode the transaction log details
                            if let Some(market) =
                                Market::from_address(&call_address, &runtime_cache)
                            {
                                let mut raw_log: RawLog = RawLog {
                                    topics: vec![],
                                    data: parse_buffer(&trace["data"]),
                                };

                                let topic_count: usize = trace.len() - 2;
                                if topic_count >= 1 {
                                    for i in 0..topic_count {
                                        if let Some(topic) =
                                            parse_topic_buffer(&trace[&i.to_string()])
                                        {
                                            raw_log.topics.push(topic);
                                        }
                                    }
                                }

                                transaction_logs.push(TransactionLog {
                                    address: call_address,
                                    protocol: market.value.protocol,
                                    raw: raw_log,
                                });
                            }
                        } else {
                            return None;
                        }
                    }

                    return Some(transaction_logs);
                }
            }
        }
    }

    return None;
}
