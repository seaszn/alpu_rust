use ethers::types::Bytes;

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

// fn remove_zero_padding(data: &[u8]) -> Vec<u8> {
//     let mut result: Vec<u8> = vec![];
//     let mut triggered = false;

//     for byte in data {
//         if byte != &0 {
//             triggered = true
//         }

//         if triggered {
//             result.push(*byte);
//         }
//     }

//     return result;
// }
