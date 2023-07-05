use std::{ops::Mul, sync::*, vec};

use ethers::{abi::RawLog, prelude::*};
use tokio::task::JoinSet;

use self::types::{
    uniswap_v2_pair, UniswapV2Factory, UniswapV2FactoryContract, UniswapV2Pair,
    UniswapV2PairContract,
};

use super::Exchange;
use crate::{
    env::types::{RuntimeClient, UniswapQueryContract},
    handlers::types::swap::BalanceChange,
    networks::Network,
    types::{market::Market, TransactionLog},
};

mod types;

lazy_static! {
    static ref PAIR_INTERFACE: UniswapV2PairContract = UniswapV2Pair::new(
        *crate::env::ZERO_ADDRESS,
        crate::env::RUNTIME_CACHE.client.clone()
    );
}

pub async fn get_markets(
    exchange: &Exchange,
    network: Arc<Network>,
    client: RuntimeClient,
    uniswap_query: UniswapQueryContract,
) -> Vec<Arc<Market>> {
    let factory_contract: UniswapV2FactoryContract =
        UniswapV2Factory::new(exchange.factory_address, client);

    let batch_size: U256 = U256::from_dec_str("1000").unwrap();
    let market_count: U256 = factory_contract.all_pairs_length().await.unwrap();
    let batch_count: U256 = market_count / batch_size + 1;
    let exchange_fee: i32 = exchange.base_fee;
    let exchange_protocol = exchange.protocol;

    let mut set: JoinSet<Vec<Market>> = JoinSet::new();

    for i in 0..batch_count.as_u32() {
        let query = uniswap_query.clone();
        let factory_address = exchange.factory_address;
        let index = batch_size.mul(i);
        let network = network.clone();
        let tokens: Arc<Vec<Arc<crate::types::Token>>> = Arc::from(network.tokens.clone());

        set.spawn(async move {
            let response: Result<Vec<[H160; 3]>, _> = query
                .get_uniswap_v2_markets(factory_address, index, index + batch_size)
                .await;

            let mut batch_markets: Vec<Market> = vec![];

            if response.is_ok() {
                for element in response.unwrap() {
                    let token0 = tokens.iter().find(|s| s.contract_address.0 == element[0].0);
                    let token1 = tokens.iter().find(|s| s.contract_address.0 == element[1].0);

                    if token0.is_some() && token1.is_some() {
                        batch_markets.push(Market {
                            contract_address: element[2],
                            tokens: [token0.unwrap().clone(), token1.unwrap().clone()],
                            fee: exchange_fee,
                            stable: false,
                            protocol: exchange_protocol,
                        });
                    }
                }
            }

            return batch_markets;
        });
    }

    let mut exchange_markets: Vec<Arc<Market>> = vec![];
    while let Some(res) = set.join_next().await {
        let response = res.unwrap();

        for market in response {
            exchange_markets.push(Arc::new(market));
        }
    }

    return exchange_markets;
}

pub fn parse_balance_changes(logs: &Vec<TransactionLog>) -> Vec<BalanceChange> {
    if logs.len() > 1 {
        let raw_logs: Vec<RawLog> = logs.clone().into_iter().map(|x| x.raw).collect();
        let mut swap_events: Vec<BalanceChange> = vec![];

        for i in 0..raw_logs.len(){
            let log = raw_logs[i].clone();
            let decode_result = ethers::contract::decode_logs::<uniswap_v2_pair::SwapFilter>(&[log]);

            if decode_result.is_ok() {
                let instance: &uniswap_v2_pair::SwapFilter = &decode_result.unwrap()[0];

                swap_events.push(BalanceChange {
                    address: logs[i].address,
                    amount_0_in: instance.amount_0_in,
                    amount_1_in: instance.amount_1_in,
                    amount_0_out: instance.amount_0_out,
                    amount_1_out: instance.amount_1_out,
                });
            }
        }
        
        return swap_events.to_vec();
    }

    return vec![];
}
