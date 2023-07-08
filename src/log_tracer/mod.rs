use std::{sync::Arc, time::Instant, u8, process};

use crate::{
    env::{self, types::RuntimeClient},
    types::{market::Market, HexNum, Token, TransactionLog},
};
use ethers::{
    abi::Address,
    providers::Middleware,
    types::{
        BlockId, BlockNumber, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, GethTraceFrame,
        TransactionRequest, U256,
    },
};

use self::{
    types::{TraceFrame, TransferCall},
    utils::{flatten_call_frames, format_data, trim_bytes_to},
};

mod types;
mod utils;

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

const TRANSFER_FROM_SIG: [u8; 4] = [35, 184, 114, 221];
const TRANSFER_SIG: [u8; 4] = [169, 5, 156, 187];
/*
const WITHDRAW_SIG: [u8; 4] = [46, 26, 125, 77];
const APPROVE_SIG: [u8; 4] = [9, 94, 167, 179];
const BALANCE_OF_SIG: [u8; 4] = [112, 160, 130, 49];
const INCREASE_ALLOWANCE_SIG: [u8; 4] = [57, 80, 147, 81];
const DEPOSIT_SIG: [u8; 4] = [208, 227, 13, 176];
*/

pub async fn trace_transaction(tx: TransactionRequest) -> Option<Vec<TransactionLog>> {
    let client: RuntimeClient = env::RUNTIME_CACHE.client.clone();
    let inst = Instant::now();

    if let Ok(response) = client
        .debug_trace_call(tx, Some(BlockId::Number(BlockNumber::Latest)), CALL_OPTIONS)
        .await
    {
        if let GethTrace::Known(trace_frame) = response {
            if let GethTraceFrame::CallTracer(top_call_frame) = trace_frame {
                let trace_frames: Vec<TraceFrame> = flatten_call_frames(&top_call_frame);

                for frame in trace_frames {
                    if let Some(_token) = Token::from_address(&frame.to) {
                        let transfer_calls: Vec<TransferCall> = read_transfer_calls(frame);

                        if transfer_calls.len() > 0 {
                            for tranfer in transfer_calls {
                                let mut affected_market: Option<Arc<Market>> = None;
                                let mut value: Option<HexNum> = None;

                                // balance coming in
                                if let Some(market) = Market::from_address(&tranfer.recipient) {
                                    affected_market = Some(market);
                                    value = Some(HexNum::from(tranfer.value));
                                }

                                // balanche going out
                                if let Some(market) = Market::from_address(&tranfer.sender) {
                                    affected_market = Some(market);
                                    value = Some(HexNum::from_negative(tranfer.value));
                                }

                                // process balance changes
                                if affected_market != None && value != None {
                                    println!("{:#?}", inst.elapsed());
                                    process::exit(0);
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

fn read_transfer_calls(frame: TraceFrame) -> Vec<TransferCall> {
    let (signature, data) = frame.input.0.split_at(4);
    let mut tranfer_calls: Vec<TransferCall> = vec![];

    let call_data = format_data(data);
    if signature == TRANSFER_FROM_SIG {
        tranfer_calls.push(TransferCall {
            sender: Address::from_slice(&trim_bytes_to(call_data[0].clone(), 20)),
            recipient: Address::from_slice(&trim_bytes_to(call_data[1].clone(), 20)),
            value: U256::from_big_endian(&trim_bytes_to(call_data[2].clone(), 32)),
        });
    } else if signature == TRANSFER_SIG {
        tranfer_calls.push(TransferCall {
            sender: frame.from,
            recipient: Address::from_slice(&trim_bytes_to(call_data[0].clone(), 20)),
            value: U256::from_big_endian(&trim_bytes_to(call_data[1].clone(), 32)),
        });
    }

    return tranfer_calls;
}
