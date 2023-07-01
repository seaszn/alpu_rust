use ethers::types::Address;
use dotenv::dotenv;

pub struct RuntimeConfig {
    pub chain_id: u32,
    pub rpc_endpoint: String,
    pub executor_address: Address,
    pub private_key: String,
    pub route_restraints: (i32, i32),
    pub min_market_reserves: f32,
}

pub fn init() -> RuntimeConfig {
    dotenv().ok();

    let chain_id: u32 = std::env::var("CHAIN_ID")
        .expect("CHAIN_ID must be set")
        .parse()
        .expect("CHAIN_ID must be a number");

    let rpc_endpoint: String = std::env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set");
    let private_key: String = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let executor_address: Address = std::env::var("BUNDLE_EXECUTOR")
        .expect("BUNDLE_EXECUTOR must be set")
        .parse()
        .expect("BUNDLE_EXECUTOR is not a valid address");

    let min_route_length: i32 = std::env::var("MIN_ROUTE_LENGTH")
        .expect("MIN_ROUTE_LENGTH must be set")
        .parse()
        .expect("MIN_ROUTE_LENGTH is not a number");

    let max_route_length: i32 = std::env::var("MAX_ROUTE_LENGTH")
        .expect("MAX_ROUTE_LENGTH must be set")
        .parse()
        .expect("MAX_ROUTE_LENGTH is not a number");

    let route_restraints: (i32, i32) = (min_route_length, max_route_length);

    let min_market_reserves: f32 = std::env::var("MIN_MARKET_RESERVES")
        .expect("MIN_MARKET_RESERVES must be set")
        .parse()
        .expect("MIN_MARKET_RESERVES must be a number");

    return RuntimeConfig {
        chain_id,
        rpc_endpoint,
        executor_address,
        private_key,
        route_restraints,
        min_market_reserves,
    };
}
