use std::{sync::Arc, str::FromStr};

use ethers::{types::H160, abi::Address};

use self::cache::RuntimeCache;
use self::config::RuntimeConfig;
use crate::networks::{self, *};

mod cache;
mod config;
pub mod types;

const ZERO_ADDRESS_CONST: &str = "0x0000000000000000000000000000000000000000";

lazy_static! {
    pub static ref RUNTIME_CONFIG: RuntimeConfig = config::init();
    pub static ref RUNTIME_NETWORK: Arc<Network> = Arc::from(networks::init(&RUNTIME_CONFIG.chain_id));
    pub static ref RUNTIME_CACHE: RuntimeCache = cache::init(&RUNTIME_CONFIG, RUNTIME_NETWORK.clone());

    pub static ref ZERO_ADDRESS: H160 = Address::from_str(ZERO_ADDRESS_CONST).unwrap();
}
