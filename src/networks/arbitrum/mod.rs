use super::Network;

const CHAIN_ID: i32 = 42161;
const NAME: &str = "arbitrum";

const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";
const UNISWAP_QUERY_ADDRESS: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";

pub fn get_chain_id() -> i32 {
    return CHAIN_ID;
}

pub fn get_instance() -> Network {
    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_owned(),
        tokens: vec![],
        exchanges: vec![],
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
