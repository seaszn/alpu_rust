use super::Network;
use crate::types::Token;

pub const CHAIN_ID: u32 = 42161;
const NAME: &str = "arbitrum";
const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0x9af2925C7b97b9418c3C0eb759c0E644701b9714";
// const UNISWAP_QUERY_ADDRESS: &str = "0x70FeDD23788d69FDB2B24fcbf2e49eD3b80Ec1F9";
// const UNISWAP_QUERY_ADDRESS: &str = "0x9b7De739c00462a07E77AD1400a995727A45D2B8";
// const UNISWAP_QUERY_ADDRESS: &str = "0x20Be6AE0E737a4D4c09511B345BE49d74312344f";
const UNISWAP_QUERY_ADDRESS: &str = "0xea644C72007a2bBBff36D36bE1DA13fdF6eFCB81";
// 0x921007E81D221C46Ebea31Adf3C2cE47541eE9fA
//0xD691f20b72Ece17688b67d4E92EF9a89ff274D83
//0x20Be6AE0E737a4D4c09511B345BE49d74312344f

pub fn get_instance() -> Network {
    let exchanges: Vec<crate::exchanges::types::Exchange> = super::load_exchanges_from_file(NAME);
    let tokens: Vec<Token> = super::load_tokens_from_file(NAME);

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_string(),
        exchanges,
        tokens,
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
