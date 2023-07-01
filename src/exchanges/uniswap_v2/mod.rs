use std::sync::*;

use ethers::prelude::*;
// use ethers::types::U256;

use super::Exchange;
use crate::types::Market;

abigen!(UniswapV2Factory, "src/exchanges/uniswap_v2/_factory.json");

pub async fn get_markets(_exchange: &Exchange) -> Vec<Arc<Market>> {
    // let factory_contract = UniswapV2Factory::new(exchange.factory_address);

    // let _start: U256 = U256::zero();
    // let _market_count: U256 = factory_contract.all_pairs_length().await.expect("");

    // let batch_count: U256 = market_count / start + 1;
    // let mut result: Vec<Arc<Market>> = vec![];

    // // let calls: &mut Vec<_> = &mut vec![];
    //     let han = uniswap_query.get_uniswap_v2_markets(
    //         exchange.factory_address,
    //         U256::from(0),
    //         U256::from(batch_size),
    //     ).await;

    // for element in response {
    //     let token0: Result<&Arc<Token>, bool> = tokens
    //         .into_iter()
    //         .find(|f: &&Arc<Token>| {
    //             f.contract_address.to_string().to_lowercase()
    //                 == element[0].to_string().to_lowercase()
    //         })
    //         .ok_or(false);

    //     let token1: Result<&Arc<Token>, bool> = tokens
    //         .into_iter()
    //         .find(|f: &&Arc<Token>| {
    //             f.contract_address.to_string().to_lowercase()
    //                 == element[1].to_string().to_lowercase()
    //         })
    //         .ok_or(false);

    //     if !token0.is_err() && !token1.is_err() {
    //         result.push(Arc::new(Market {
    //             contract_address: element[2],
    //             tokens: [
    //                 Arc::downgrade(token0.unwrap()),
    //                 Arc::downgrade(token1.unwrap()),
    //             ],
    //             fee: exchange.base_fee,
    //             stable: false,
    //         }));
    //     }
    // }

    return vec![];
}
