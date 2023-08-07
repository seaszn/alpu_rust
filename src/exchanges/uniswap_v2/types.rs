use ethers::prelude::*;

use crate::env::types::RuntimeClient;

abigen!(UniswapV2Factory, "src/exchanges/uniswap_v2/_factory.json");
abigen!(UniswapV2Pair, "src/exchanges/uniswap_v2/_pair.json");

pub type UniswapV2FactoryContract = UniswapV2Factory<RuntimeClient>;
pub type UniswapV2MarketState = (U256, U256);