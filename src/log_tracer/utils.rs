use ethers::types::{Bytes, CallFrame};

use super::types::TraceFrame;

pub fn format_data(data: &[u8]) -> Vec<Bytes> {
    let data_chunks: Vec<&[u8]> = data.chunks(32).collect();
    let mut result: Vec<Bytes> = vec![];

    for chunk in data_chunks {
        result.push(Bytes::from(chunk.to_vec()));
    }

    return result;
}

pub fn trim_bytes_to(bytes: Bytes, length: usize) -> Vec<u8> {
    return bytes.split_at(bytes.len() - length).1.to_vec();
}

pub fn flatten_call_frames(top_call_frame: &CallFrame) -> Vec<TraceFrame> {
    let mut result: Vec<TraceFrame> = vec![];

    if let Some(internal_calls) = &top_call_frame.calls {
        for call in internal_calls {
            result.append(&mut flatten_call_frames(call));
        }
    }

    result.push(TraceFrame::from_call_frame(top_call_frame.clone()));

    return result;
}