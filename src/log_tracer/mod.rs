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
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

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

const GETH_DEBUG_TRACING_OPTIONS:ethers::types::GethDebugTracingOptions = ethers::types::GethDebugTracingOptions {
    enable_memory: Some(true),
    enable_return_data: Some(true),
    disable_storage: Some(false),
    tracer: None,
    tracer_config: None,
    timeout: None,
    disable_stack: Some(false),
};

#[inline(always)]
pub async fn trace_transaction(
    tx: Transaction,
    runtime_cache: &'static RuntimeCache,
) -> Option<Vec<TransactionLog>> {
    // get the transaction traces
    let request = TransactionRequest {
        from: Some(tx.from),
        to: Some(NameOrAddress::Address(tx.to.unwrap())),
        gas: Some(tx.gas),
        gas_price: tx.gas_price,
        value: Some(tx.value),
        data: Some(tx.input),
        nonce: None,
        chain_id: None,
    };

    let tracing_options = GethDebugTracingCallOptions{
        state_overrides: None,
        tracing_options: GethDebugTracingOptions {
            tracer: Some(GethDebugTracerType::JsTracer(JS_CONTENT.to_string())) ,
            ..GETH_DEBUG_TRACING_OPTIONS
        }
    };

    if let Ok(geth_trace) = runtime_cache
        .client
        .debug_trace_call(
            request,
            Some(BlockId::Number(BlockNumber::Number(
                tx.block_number.unwrap() - 1,
            ))),
            tracing_options,
        )
        .await
    {
        // Check if the result is valid
        if let GethTrace::Unknown(trace_container) = geth_trace {
            // All raw traces should be contained in an array
            if let Some(raw_traces) = trace_container.as_array() {
                // Only itterate if length > 0
                if raw_traces.len() > 0 {
                    let res: Vec<TransactionLog> = raw_traces
                        .par_iter()
                        .filter_map(|trace_object| {
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

                                    Some(TransactionLog {
                                        address: call_address,
                                        protocol: market.value.protocol,
                                        raw: raw_log,
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect();

                    return Some(res);
                }
            }
        }
    }

    return None;
}
