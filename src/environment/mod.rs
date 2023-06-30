use crate::networks::{get_network_instance, Network};
use dotenv::dotenv;
mod runtime;

pub struct Environment {
    pub cache: runtime::Cache,
    pub config: runtime::Config,
    pub network: Network,
}

pub fn init() -> Environment {
    dotenv().ok();

    let config: runtime::Config = runtime::get_runtime_config();
    let network: Network = get_network_instance(&config.chain_id);
    let runtime: runtime::Cache = runtime::get_runtime_cache(&config, &network);

    return Environment {
        network,
        config,
        cache: runtime,
    };
}