use crate::utils::parse;
use dotenv::dotenv;
use ethers::types::Address;
use url::Url;

pub struct RuntimeConfig {
    pub chain_id: u32,
    pub rpc_endpoint: Url,
    pub feed_endpoint: Url,
    pub executor_address: Address,
    pub private_key: String,
    pub route_restraints: (u32, u32),
    pub min_market_reserves: f32,
}

impl RuntimeConfig {
    pub fn from_dot_env_file() -> RuntimeConfig {
        dotenv().ok();

        return RuntimeConfig {
            chain_id: read_u32("CHAIN_ID"),
            rpc_endpoint: read_url("RPC_ENDPOINT"),
            feed_endpoint: read_url("FEED_ENDPOINT"),
            executor_address: read_address("BUNDLE_EXECUTOR"),
            private_key: read_string("PRIVATE_KEY"),
            route_restraints: (read_u32("MIN_ROUTE_LENGTH"), read_u32("MAX_ROUTE_LENGTH")),
            min_market_reserves: read_f32("MIN_MARKET_RESERVES"),
        };
    }
}

fn read_address(input: &str) -> Address {
    parse::address(read_string(input))
}

fn read_url(input: &str) -> Url {
    return parse::url(read_string(input));
}

fn read_u32(input: &str) -> u32 {
    return parse::u32(read_string(input));
}

fn read_f32(input: &str) -> f32 {
    return parse::f32(read_string(input));
}

fn read_string(input: &str) -> String {
    let read_result: Result<String, _> = std::env::var(input);
    if read_result.is_err() {
        println!("environment variable not set ({})", input);
    }

    return read_result.unwrap();
}
