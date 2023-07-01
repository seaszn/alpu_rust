use std::sync::Arc;

use super::Network;
use crate::types::Token;

pub const CHAIN_ID: u32 = 42161;
const NAME: &str = "arbitrum";
const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";
const UNISWAP_QUERY_ADDRESS: &str = "0x70FeDD23788d69FDB2B24fcbf2e49eD3b80Ec1F9";

pub fn get_instance() -> Network {
    let exchanges: Vec<crate::exchanges::Exchange> = super::load_exchanges_from_file(NAME);
    let tokens: Vec<Arc<Token>> = super::load_tokens_from_file(NAME);

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_string(),
        exchanges,
        tokens,
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
