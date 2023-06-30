use crate::networks::{get_network_instance, Network};
use dotenv::dotenv;
mod runtime;

pub struct Environment<'c> {
    pub cache: runtime::Cache<'c>,
    pub config: runtime::Config,
    pub network: Network,
}

pub fn init<'c>() -> Environment<'c> {
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
