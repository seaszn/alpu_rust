use ethers::types::*;

#[derive(Clone, Debug)]
pub struct TraceFrame{
    pub typ: String,
    pub from: Address,
    pub to: NameOrAddress,
    pub value: Option<U256>,
    pub gas: U256,
    pub gas_used: U256,
    pub input: Bytes,
    pub output: Option<Bytes>,
    pub error: Option<String>,
}

impl TraceFrame {
    pub fn from_call_frame(top_call_frame: CallFrame) -> TraceFrame{
        return TraceFrame{
            typ: top_call_frame.typ,
            from: top_call_frame.from,
            to: top_call_frame.to.unwrap(),
            value: top_call_frame.value,
            gas: top_call_frame.gas,
            gas_used: top_call_frame.gas_used,
            input: top_call_frame.input,
            output: top_call_frame.output,
            error: top_call_frame.error
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TransferCall{
    pub sender: H160,
    pub recipient: H160,
    pub value: U256
}