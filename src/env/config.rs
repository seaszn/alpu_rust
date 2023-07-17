use crate::utils::parse;
use dotenv::dotenv;
use ethers::types::Address;
use url::Url;

#[derive(Clone)]
pub struct RuntimeConfig {
    pub chain_id: u32,
    pub rpc_endpoint: Url,
    pub feed_endpoint: Url,
    pub executor_address: Address,
    pub private_key: String,
    pub route_restraints: (usize, usize),
    pub small_chunk_size: usize,
    pub large_chunk_size: usize,
    pub min_market_reserves: String,
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
            route_restraints: (read_u32("MIN_ROUTE_LENGTH") as usize, read_u32("MAX_ROUTE_LENGTH") as usize),
            min_market_reserves: read_string("MIN_MARKET_RESERVES"),
            small_chunk_size: read_u32("SMALL_CHUNK_SIZE") as usize,
            large_chunk_size: read_u32("LARGE_CHUNK_SIZE") as usize
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

fn read_string(input: &str) -> String {
    let read_result: Result<String, _> = std::env::var(input);
    if read_result.is_err() {
        println!("environment variable not set ({})", input);
    }

    return read_result.unwrap();
}
