use std::{io::Error, sync::RwLock};

use env::{RuntimeCache, RuntimeConfig};
use networks::Network;
use types::Route;

use crate::{exchanges::init_exchange_handlers, handlers::NetworkHandler};

#[macro_use]
extern crate lazy_static;
extern crate async_trait;
extern crate base64;

pub mod env;
pub mod exchanges;
mod handlers;
pub mod log_tracer;
pub mod networks;
pub mod price_oracle;
pub mod types;
pub mod utils;

lazy_static! {
    static ref RUNTIME_CONFIG: RuntimeConfig = env::RuntimeConfig::from_dot_env_file();
    static ref RUNTIME_NETWORK: Network = Network::from_chain_id(&RUNTIME_CONFIG.chain_id);
    static ref RUNTIME_CACHE: Result<RuntimeCache, Error> =
        RuntimeCache::new(&RUNTIME_CONFIG, &RUNTIME_NETWORK);
    static ref RUNTIME_ROUTES: RwLock<Vec<Route>> = RwLock::new(Vec::new());
        // Route::generate_from_runtime(&RUNTIME_NETWORK, &RUNTIME_CONFIG, &RUNTIME_CACHE);

    // static ref USDT: Token = Token{
    //        contract_address: H160::from_str("0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9").unwrap(),
    //        decimals: 6,
    //        flash_loan_enabled: true,
    //        ref_symbol: Some("USDT".to_string())
    //    };

    // static ref USDC: Token = Token{
    //     contract_address: H160::from_str("0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8").unwrap(),
    //     decimals: 6,
    //     flash_loan_enabled: true,
    //        ref_symbol: Some("USDC".to_string())
    // };

    // static ref WETH: Token = Token{
    //     contract_address: H160::from_str("0x82aF49447D8a07e3bd95BD0d56f35241523fBab1").unwrap(),
    //     decimals: 18,
    //     flash_loan_enabled: true,
    //     ref_symbol: Some("ETH".to_string())

    // };

    // static ref MARKETS: OrganizedList<Market> = generate_test_markets();

}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Connecting to {} network...\n", &*RUNTIME_NETWORK.name);
    utils::logger::clear_console();

    match &*RUNTIME_CACHE {
        Ok(runtime_cache) => {
            init_exchange_handlers();

            println!("\nQuery: {}", runtime_cache.uniswap_query.address());
            println!("Wallet: {}", runtime_cache.client.address());
            println!("Executor: {}\n", runtime_cache.bundle_executor.address());

            println!("Cached {} tokens..", RUNTIME_NETWORK.tokens.len());
            println!("Cached {} markets..", runtime_cache.markets.len());

            let route_count =
                Route::generate_from_runtime(&*RUNTIME_NETWORK, &*RUNTIME_CONFIG, runtime_cache);
            println!("Cached {} routes..\n", route_count);

            println!("Waiting for validation, this might take a while...");

            if let Some(mut network_handler) =
                NetworkHandler::from_network(&RUNTIME_NETWORK, &RUNTIME_CONFIG, runtime_cache)
            {
                network_handler.init().await;
            }
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}

// fn generate_test_markets() -> OrganizedList<Market> {
//     let mut list: OrganizedList<Market> = OrganizedList::new();

//     // USDT / USDC
//     list.add_value(Market {
//         contract_address: H160::from_str("0xC9445A9AFe8E48c71459aEdf956eD950e983eC5A").unwrap(),
//         tokens: [&*USDT, &*USDC],
//         fee: 4,
//         stable: true,
//         protocol: Protocol::StableSwap,
//         fee_mul: U256::from(10000u128 - 4u128),
//     });

//     // WETH / USDT
//     list.add_value(Market {
//         contract_address: H160::from_str("0xC9445A9AFe8E48c71459aEdf956eD950e983eC5A").unwrap(),
//         tokens: [&*WETH, &*USDT],
//         fee: 20,
//         stable: false,
//         protocol: Protocol::UniswapV2,
//         fee_mul: U256::from(10000u128 - 20u128),
//     });

//     // WETH / USDC
//     list.add_value(Market {
//         contract_address: H160::from_str("0xA2F1C1B52E1b7223825552343297Dc68a29ABecC").unwrap(),
//         tokens: [&*WETH, &*USDC],
//         fee: 20,
//         stable: true,
//         protocol: Protocol::UniswapV2,
//         fee_mul: U256::from(10000u128 - 20u128),
//     });

//     return list;
// }

// fn generate_test_reserve_table() -> OrganizedList<MarketState> {
//     let mut list: OrganizedList<MarketState> = OrganizedList::new();

//     list.add_value(MarketState::StableSwap((
//         301579526325u128.into(),
//         252647399201u128.into(),
//     )));
//     list.add_value(MarketState::UniswapV2((
//         633696491977773674624u128.into(),
//         976317941063u128.into(),
//     )));

//     list.add_value(MarketState::UniswapV2((
//         534967697769533878115u128.into(),
//         979941454039u128.into(),
//     )));

//     return list;
// }

// async fn get_test_price_table() -> PriceTable {
//     let tokens = [&*USDT, &*USDC, &*WETH].to_vec();
//     let json_response =
//         fs::read_to_string("src/price_oracle/response.json").expect("Failed to read token file");
//     let s: Value = serde_json::from_str(json_response.as_str()).unwrap();
//     let value_map = s.as_object().unwrap()["data"].as_object().unwrap()["rates"]
//         .as_object()
//         .unwrap();

//     let mut new_price_table = PriceTable::new();
//     for token in tokens {
//         let symbol: &String = token.ref_symbol.as_ref().unwrap();
//         let token_ref_price = value_map[symbol].as_str();

//         if let Ok(ParseUnits::U256(ref_price)) =
//             parse_units(token_ref_price.unwrap().to_string(), 18)
//         {
//             new_price_table.update_value(&token, ref_price);
//         }
//     }

//     return new_price_table;
// }
