use std::sync::Arc;

use self::cache::RuntimeCache;
use self::config::RuntimeConfig;
use crate::networks::{self, *};

mod cache;
mod config;
pub mod types;

lazy_static! {
    pub static ref RUNTIME_CONFIG: RuntimeConfig = config::init();
    pub static ref RUNTIME_NETWORK: Arc<Network> = Arc::from(networks::init(&RUNTIME_CONFIG.chain_id));
    pub static ref RUNTIME_CACHE: RuntimeCache = cache::init(&RUNTIME_CONFIG, RUNTIME_NETWORK.clone());
}
