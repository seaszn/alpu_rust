use std::str::FromStr;

use super::Network;
use crate::{environment::Environment, utils};

const CHAIN_ID: u32 = 42161;
const NAME: &str = "arbitrum";

const FLASHLOAN_POOL_ADDRESS_PROVIDER: &str = "0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb";
const UNISWAP_QUERY_ADDRESS: &str = "0x70FeDD23788d69FDB2B24fcbf2e49eD3b80Ec1F9";

// lazy_static! {
//     static ref ARBITRUM_ONE: Network = {
//         let exchanges: Vec<crate::exchanges::Exchange> =
//         utils::json::deserialize_exchange_file(format!("src/networks/arbitrum/_exchanges.json"));

//         let m: Network = Network {
//             chain_id: 42161,
//             name: "arbitrum_one".to_string(),
//             exchanges: exchanges,
//             flashloan_pool_address_provider: ethers::types::H160::from_str("0xa97684ead0e402dC232d5A977953DF7ECBaB3CDb").unwrap(),
//             uniswap_query_address: ethers::types::H160::from_str("0x70FeDD23788d69FDB2B24fcbf2e49eD3b80Ec1F9").unwrap()
//         }

//             return  m;
//     };
// }

pub fn chain_id() -> u32 {
    return CHAIN_ID;
}

pub fn get_instance() -> Network {
    let exchanges: Vec<crate::exchanges::Exchange> =
        utils::json::deserialize_exchange_file(format!("src/networks/arbitrum/_exchanges.json"));

    return Network {
        chain_id: CHAIN_ID,
        name: NAME.to_owned(),
        exchanges,
        uniswap_query_address: UNISWAP_QUERY_ADDRESS.parse().unwrap(),
        flashloan_pool_address_provider: FLASHLOAN_POOL_ADDRESS_PROVIDER.parse().unwrap(),
    };
}
