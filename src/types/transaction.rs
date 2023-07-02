use ethers::{types::{NameOrAddress, Bytes, U256}};

#[derive(Debug)]
pub struct Transaction{
    pub to: Option<NameOrAddress>,
    pub from: Option<NameOrAddress>,
    pub value: Option<U256>,
    pub data: Option<Bytes>,
}