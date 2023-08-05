use ethers::prelude::*;

use crate::env::types::RuntimeClient;

abigen!(StableSwapFactory, "src/exchanges/stable_swap/_factory.json");
abigen!(StableSwapPair, "src/exchanges/stable_swap/_pair.json");

pub type StableSwapFactoryContract = StableSwapFactory<RuntimeClient>;
pub type StableSwapMarketState = (U256, U256);