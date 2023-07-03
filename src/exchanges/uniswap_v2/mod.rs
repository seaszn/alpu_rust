use std::{ops::Mul, sync::*};

use ethers::prelude::*;

use self::types::{UniswapV2Factory, UniswapV2FactoryContract};

use super::Exchange;
use crate::{
    env::types::{RuntimeClient, UniswapQueryContract},
    networks::Network,
    types::Market,
};

mod types;

pub async fn get_markets(
    exchange: &Exchange,
    network: Arc<Network>,
    client: RuntimeClient,
    uniswap_query: UniswapQueryContract,
) -> Vec<Arc<Market>> {
    let factory_contract: UniswapV2FactoryContract =
        UniswapV2Factory::new(exchange.factory_address, client);

    let batch_size: U256 = U256::from_dec_str("100").unwrap();
    let market_count: U256 = factory_contract.all_pairs_length().await.unwrap();
    let batch_count: U256 = market_count / batch_size + 1;
    let exchange_fee: i32 = exchange.base_fee;

    // let tokens = &network.tokens;
    for i in 0..batch_count.as_u32() {
        let query = uniswap_query.clone();
        let factory_address = exchange.factory_address;
        let index = batch_size.mul(i);
        let net = network.clone();

        let _handle: tokio::task::JoinHandle<_> = tokio::spawn(async move {
            let response = query
                .get_uniswap_v2_markets(factory_address, index, index + batch_size)
                .await;

            if response.is_ok() {
                let data: Vec<[H160; 3]> = response.unwrap();
                for element in data {
                    let token0 = net
                        .tokens
                        .clone()
                        .into_iter()
                        .find(|x| {
                            x.contract_address.to_string().to_lowercase()
                                == element[0].to_string().to_lowercase()
                        })
                        .ok_or(false);

                    let token1 = net
                        .tokens
                        .clone()
                        .into_iter()
                        .find(|f| {
                            f.contract_address.to_string().to_lowercase()
                                == element[1].to_string().to_lowercase()
                        })
                        .ok_or(false);

                    if !token0.is_err() && !token1.is_err() {
                        let _market = Arc::new(Market {
                            contract_address: element[2],
                            tokens: [token0.unwrap(), token1.unwrap()],
                            fee: exchange_fee,
                            stable: false,
                        });
                    }
                }
            }
        });
    }

    return vec![];
}
