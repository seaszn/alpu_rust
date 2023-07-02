use ethers::{
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, TransactionRequest,
    },
};

use crate::env;

extern crate lazy_static;

const JS_CONTENT: &str = "{data: [],\n
    fault: function (log) {\n
    },\n 
    
    step: function (log) {\n
       var topicCount = (log.op.toString().match(/LOG(\\d)/) || [])[1];\n
        if (topicCount) {\n
            const res = {\n
                address: log.contract.getAddress(),\n
                data: log.memory.slice(parseInt(log.stack.peek(0)), parseInt(log.stack.peek(0)) + parseInt(log.stack.peek(1))),\n
            };\n
            for (var i = 0; i < topicCount; i++)\n
                res['topic' + i.toString()] = log.stack.peek(i + 2);\n
            this.data.push(res);\n
        }\n
    },\n
    result: function () {\n
        return this.data;\n
    }\n}";

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

pub async fn trace_transaction_logs(tx: TransactionRequest) -> Option<GethTrace> {
    let client = env::RUNTIME_CACHE.client.clone();
    let response = client
        .debug_trace_call(
            tx,
            Some(BlockId::Number(BlockNumber::Latest)),
            CALL_OPTIONS.clone(),
        )
        .await;

    if response.is_ok() {
        return Some(response.unwrap());
    } else {
        return None;
    }
}
