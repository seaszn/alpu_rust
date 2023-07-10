mod cache;
mod config;
pub mod types;

pub use cache::RuntimeCache;
pub use config::RuntimeConfig;

// const ZERO_ADDRESS_CONST: &str = "0x0000000000000000000000000000000000000000";

// lazy_static! {
//     // pub static ref RUNTIME_CONFIG: RuntimeConfig = config::init();
//     pub static ref RUNTIME_NETWORK: Arc<Network> = Arc::from(networks::init(&RUNTIME_CONFIG.chain_id));
//     pub static  ref RUNTIME_CACHE: RuntimeCache = cache::init(&RUNTIME_CONFIG, RUNTIME_NETWORK.clone());

//     pub static ref ZERO_ADDRESS: H160 = Address::from_str(ZERO_ADDRESS_CONST).unwrap();
// }
