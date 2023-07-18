use ethers::prelude::*;
use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::io::{Error, ErrorKind};
use tokio::task::JoinSet;

use self::types::{uniswap_v2_pair, UniswapV2Factory, UniswapV2FactoryContract};

use super::Exchange;
use crate::{
    env::{RuntimeCache, RuntimeConfig},
    handlers::types::BalanceChange,
    networks::Network,
    types::{market::Market, OrgValue, OrganizedList, Reserves, TransactionLog},
};

mod types;

#[inline(always)]
pub async fn get_markets(
    exchange: &'static Exchange,
    network: &'static Network,
    runtime_cache: &RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> Result<Vec<Market>, Error> {
    let factory_contract: UniswapV2FactoryContract =
        UniswapV2Factory::new(exchange.factory_address, runtime_cache.client.clone());
    let mut result: Vec<Market> = vec![];

    if let Ok(market_count) = factory_contract.all_pairs_length().await {
        let total_market_count: u128 = market_count.as_u128();

        for chunk in &(0..total_market_count)
            .into_iter()
            .chunks(runtime_config.large_chunk_size)
        {
            let (start, stop) = {
                let chunk = chunk.collect_vec();
                (U256::from(chunk[0]), U256::from(*chunk.last().unwrap()))
            };

            match runtime_cache
                .uniswap_query
                .get_uniswap_v2_markets(exchange.factory_address, start, stop)
                .await
            {
                Ok(response) => {
                    for element in response {
                        let token_0 = network
                            .tokens
                            .iter()
                            .find(|s| s.contract_address.0 == element[0].0);
                        let token_1 = network
                            .tokens
                            .iter()
                            .find(|s| s.contract_address.0 == element[1].0);

                        if token_0.is_some() && token_1.is_some() {
                            result.push(Market::new(
                                element[2],
                                [token_0.unwrap(), token_1.unwrap()],
                                exchange.base_fee,
                                false,
                                exchange.protocol,
                            ));
                        }
                    }
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::ConnectionRefused, err));
                }
            }
        }
    }

    return Ok(result);
}

#[inline(always)]
pub fn parse_balance_changes(
    logs: &Vec<TransactionLog>,
    runtime_cache: &'static RuntimeCache,
) -> Vec<BalanceChange> {
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
                            market: runtime_cache
                                .markets
                                .filter(|&x| x.value.contract_address.eq(&transaction_log.address))
                                [0],
                            amount_0_in: swap.amount_0_in.as_u128(),
                            amount_1_in: swap.amount_1_in.as_u128(),
                            amount_0_out: swap.amount_0_out.as_u128(),
                            amount_1_out: swap.amount_1_out.as_u128(),
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
pub async fn get_market_reserves(
    markets: Vec<&'static OrgValue<Market>>,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &RuntimeConfig,
) -> OrganizedList<Reserves> {
    let mut join_set: JoinSet<Vec<(usize, Reserves)>> = JoinSet::new();

    for market_addressess in &markets.into_iter().chunks(runtime_config.small_chunk_size) {
        let market_values = market_addressess.collect_vec();
        let addressess: Vec<H160> = market_values
            .iter()
            .map(|x| x.value.contract_address)
            .collect();

        join_set.spawn(async move {
            match runtime_cache
                .uniswap_query
                .get_reserves_by_pairs(addressess.clone())
                .await
            {
                Ok(response) => {
                    let mut result: Vec<(usize, Reserves)> = Vec::new();
                    for i in 0..market_values.len() {
                        let raw_reserves: [u128; 3] = response[i];
                        result.push((
                            market_values[i].id,
                            (U256::from(raw_reserves[0]), U256::from(raw_reserves[1])),
                        ))
                    }

                    return result;
                }
                Err(_) => {
                    return vec![];
                }
            }
        });
    }

    let mut res: OrganizedList<Reserves> = OrganizedList::new();
    while let Some(Ok(result)) = join_set.join_next().await {
        for i in 0..result.len() {
            res.add_pair(OrgValue {
                id: result[i].0,
                value: result[i].1,
            });
        }
    }

    res.sort();
    return res;
}
