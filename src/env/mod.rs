use crate::networks::{*, self};
use self::config::RuntimeConfig;
use self::cache::RuntimeCache;

mod cache;
mod config;

lazy_static! {
    pub static ref RUNTIME_CONFIG: RuntimeConfig = config::init();
    pub static ref NETWORK: Network = networks::get_instance(&RUNTIME_CONFIG.chain_id);
    pub static ref RUNTIME_CACHE: RuntimeCache  = cache::init(&RUNTIME_CONFIG, &NETWORK);
}