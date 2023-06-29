use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Protocol{
    UniswapV2,
    StableSwap
}

#[derive(Debug, Deserialize)]
pub struct Exchange{
    pub factory_address: String,
    pub min_liquidity: i32,
    pub protocol: Protocol,
    pub base_fee: i32,
    pub stable_fee: Option<i32>,
}