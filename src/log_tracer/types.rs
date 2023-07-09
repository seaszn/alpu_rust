use ethers::types::*;

#[derive(Clone, Debug)]
pub struct LogFrame{
    pub address: H160,
    pub data: Bytes,
    pub topics: Vec<H256>
}

impl LogFrame {
    // pub fn from_call_frame(top_call_frame: CallFrame) -> TraceFrame{
    //     return TraceFrame{
    //         typ: top_call_frame.typ,
    //         from: top_call_frame.from,
    //         to: top_call_frame.to.unwrap(),
    //         value: top_call_frame.value,
    //         gas: top_call_frame.gas,
    //         gas_used: top_call_frame.gas_used,
    //         input: top_call_frame.input,
    //         output: top_call_frame.output,
    //         error: top_call_frame.error
    //     }
    // }
}

#[derive(Clone, Copy, Debug)]
pub struct TransferCall{
    pub sender: H160,
    pub recipient: H160,
    pub value: U256
}