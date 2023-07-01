use self::cache::RuntimeCache;
use self::config::RuntimeConfig;
use crate::networks::{self, *};

mod cache;
mod config;

lazy_static! {
    pub static ref RUNTIME_CONFIG: RuntimeConfig = config::init();
    pub static ref RUNTIME_NETWORK: Network = networks::init(&RUNTIME_CONFIG.chain_id);
    pub static ref RUNTIME_CACHE: RuntimeCache = cache::init(&RUNTIME_CONFIG, &RUNTIME_NETWORK);
}
