use itertools::Itertools;
use std::{
    io::{Error, ErrorKind},
    ops::Mul,
    sync::*,
};

use ethers::prelude::*;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use tokio::task::JoinSet;

use self::types::{uniswap_v2_pair, UniswapV2Factory, UniswapV2FactoryContract};

use super::Exchange;
use crate::{
    env::RuntimeCache,
    handlers::types::swap::BalanceChange,
    networks::Network,
    types::{market::Market, ReserveTable, Reserves, TransactionLog},
};

mod types;

#[inline(always)]
pub async fn get_markets(
    exchange: &Exchange,
    network: &Network,
    runtime_cache: &RuntimeCache,
) -> Result<Vec<Arc<Market>>, Error> {
    let factory_contract: UniswapV2FactoryContract =
        UniswapV2Factory::new(exchange.factory_address, runtime_cache.client.clone());

    if let Ok(market_count) = factory_contract.all_pairs_length().await {
        let batch_size: U256 = U256::from_dec_str("100").unwrap();
        let batch_count: U256 = market_count / batch_size + 1;
        let exchange_fee: i32 = exchange.base_fee;
        let exchange_protocol = exchange.protocol;

        let mut set: JoinSet<Vec<Market>> = JoinSet::new();

        for i in 0..batch_count.as_u32() {
            let query = runtime_cache.uniswap_query.clone();
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
                        let token0 = tokens
                            .par_iter()
                            .find_first(|s| s.contract_address.0 == element[0].0);
                        let token1 = tokens
                            .par_iter()
                            .find_first(|s| s.contract_address.0 == element[1].0);

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

        return Ok(exchange_markets);
    }

    return Err(Error::new(
        ErrorKind::ConnectionRefused,
        "Failed to connect to network",
    ));
}

#[inline(always)]
pub fn parse_balance_changes(logs: &Vec<TransactionLog>) -> Vec<BalanceChange> {
    if logs.len() > 1 {
        let mut stacked_balance_changes: Vec<Vec<BalanceChange>> = vec![];
        logs.into_par_iter()
            .map(move |transaction_log| -> Vec<BalanceChange> {
                if let Ok(filters) =
                    ethers::contract::decode_logs::<uniswap_v2_pair::SwapFilter>(&[transaction_log
                        .raw
                        .clone()])
                {
                    let mut swap_events: Vec<BalanceChange> = vec![];

                    for swap in filters {
                        swap_events.push(BalanceChange {
                            address: transaction_log.address,
                            amount_0_in: swap.amount_0_in,
                            amount_1_in: swap.amount_1_in,
                            amount_0_out: swap.amount_0_out,
                            amount_1_out: swap.amount_1_out,
                        });
                    }

                    return swap_events;
                }

                return vec![];
            })
            .collect_into_vec(&mut stacked_balance_changes);

        let mut result: Vec<BalanceChange> = vec![];
        for mut change_stack in stacked_balance_changes {
            if change_stack.len() > 0 {
                result.append(&mut change_stack);
            }
        }

        return result;
    }

    return vec![];
}

#[inline(always)]
pub async fn get_market_reserves(markets: Vec<H160>, runtime_cache: &RuntimeCache) -> ReserveTable {
    let mut join_set: JoinSet<Vec<Reserves>> = JoinSet::new();
    let cache = Arc::downgrade(&runtime_cache.uniswap_query).clone();

    for market_addressess in &markets.clone().into_iter().chunks(52) {
        let addressess: Vec<H160> = market_addressess.collect();
        let uniswap_query = unsafe { &*cache.as_ptr() };

        join_set.spawn(async move {
            match uniswap_query.get_reserves_by_pairs(addressess).await {
                Ok(response) => {
                    return response
                        .into_iter()
                        .map(|element: [U256; 3]| (element[0], element[1]))
                        .collect::<Vec<Reserves>>();
                }
                Err(_) => {
                    return vec![];
                }
            }
        });
    }

    let mut map: ReserveTable = ReserveTable::new();
    while let Some(Ok(result)) = join_set.join_next().await {
        if result.len() > 0 {
            for i in 0..result.len() {
                map.add(markets[i].0, result[i])
            }
        }
    }

    return map;
}
