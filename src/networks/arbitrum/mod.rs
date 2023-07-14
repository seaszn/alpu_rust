use std::sync::Arc;

use super::Network;
use crate::types::Token;

pub const CHAIN_ID: u32 = 42161;
const NAME: &str = "arbitrum";
const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0x9af2925C7b97b9418c3C0eb759c0E644701b9714";
const UNISWAP_QUERY_ADDRESS: &str = "0x70FeDD23788d69FDB2B24fcbf2e49eD3b80Ec1F9";

pub fn get_instance() -> Network {
    let exchanges: Vec<crate::exchanges::types::Exchange> = super::load_exchanges_from_file(NAME);
    let tokens: Vec<Arc<Token>> = super::load_tokens_from_file(NAME);

    let lower_token_addressess: Vec<String> = tokens.iter().map(|x: &Arc<Token>| x.contract_address.to_string().to_lowercase()).collect();

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_string(),
        exchanges,
        tokens,
        lower_token_addressess,
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
