use ethers::{
    types::{Address, Bytes, NameOrAddress, U256, U64},
};

#[derive(Debug)]
pub struct Transaction {
    pub hash: String,
    pub to: Option<NameOrAddress>,
    pub from: Option<Address>,
    pub value: Option<U256>,
    pub data: Option<Bytes>,
    pub gas: Option<U256>,
    pub gas_price: Option<U256>,
    pub nonce: Option<U256>,
    pub chain_id: Option<U64>
}
