use super::Network;
use crate::utils;

const CHAIN_ID: i32 = 42161;
const NAME: &str = "arbitrum";

const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";
const UNISWAP_QUERY_ADDRESS: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";

pub fn get_chain_id() -> i32 {
    return CHAIN_ID;
}

pub fn get_instance() -> Network {
    let tokens: Vec<crate::types::Token> =
        utils::json::deserialize_token_file(format!("src/networks/arbitrum/_tokens.json"));
        
    let exchanges: Vec<crate::exchanges::Exchange> =
        utils::json::deserialize_exchange_file(format!("src/networks/arbitrum/_exchanges.json"));

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_owned(),
        tokens,
        exchanges,
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
