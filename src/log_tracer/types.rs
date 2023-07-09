use ethers::types::*;

#[derive(Clone, Debug)]
pub struct LogFrame{
    pub address: H160,
    pub data: Bytes,
    pub topics: Vec<H256>
}