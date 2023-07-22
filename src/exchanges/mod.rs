use std::{io::Error, vec};

use ethers::{
    prelude::AbiError,
    types::{BlockNumber, Bytes, U64, Transaction},
    types::{H160, U256},
};

use crate::{
    env::{RuntimeCache, RuntimeConfig, EXECUTE_TX_BUNDLE_FUNCTION},
    exchanges::types::Protocol,
    networks::Network,
    types::{
        market::Market, BalanceChange, OrgValue, OrganizedList, Reserves, SwapLog, TransactionLog,
    },
};

use self::types::Exchange;

mod stable_swap;
pub mod types;
mod uniswap_v2;

/*
pub struct ExchangeHandler {
    network: &'static Network,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
    internal_handlers: HashMap<Protocol, &'static (dyn ProtocolHandler + Send + Sync)>,
}
impl ExchangeHandler {
    #[inline(always)]
    pub fn new(
        network: &'static Network,
        runtime_cache: &'static RuntimeCache,
        runtime_config: &'static RuntimeConfig,
    ) -> Option<ExchangeHandler> {
        let internal_handlers = {
            let mut res: HashMap<Protocol, &'static (dyn ProtocolHandler + Send + Sync)> =
                HashMap::new();

            // res.insert(Protocol::StableSwap, None);
            // res.insert(Protocol::UniswapV2, None);

            res
        };

        return Some(ExchangeHandler {
            internal_handlers,
            network,
            runtime_cache,
            runtime_config,
        });
    }

    #[inline(always)]
    pub async fn get_markets(&self) -> Result<Vec<Market>, Error> {
        let mut result: Vec<Market> = vec![];

        for exchange in &self.network.exchanges {
            let market_response: Result<Vec<Market>, Error> = self.internal_handlers
                [&exchange.protocol]
                .get_markets(
                    exchange,
                    &self.network,
                    &self.runtime_cache,
                    &self.runtime_config,
                )
                .await;

            if let Ok(mut markets) = market_response {
                result.append(&mut markets);
            } else {
                return market_response;
            }
        }

        return Ok(result);
    }

    #[inline(always)]
    pub fn calculate_amount_out(
        &self,
        reserves: &Reserves,
        input_amount: &U256,
        market: &Market,
    ) -> U256 {
        return self.internal_handlers[&market.protocol].calculate_amount_out(
            market,
            reserves,
            input_amount,
        );
    }

    #[inline(always)]
    pub fn populate_swap(&self, swap_log: &SwapLog, recipient: &H160) -> Result<Bytes, AbiError> {
        return self.internal_handlers[&swap_log.market.value.protocol]
            .populate_swap(swap_log, recipient);
    }

    #[inline(always)]
    pub fn parse_balance_changes(&self, logs: &Vec<TransactionLog>) -> Vec<BalanceChange> {
        return logs
            .into_par_iter()
            .flat_map(|x| {
                self.internal_handlers[&x.protocol].parse_balance_change(x, &self.runtime_cache)
            })
            .collect();
    }
}

#[async_trait::async_trait]
trait ProtocolHandler {
    async fn get_markets(
        &self,
        exchange: &Exchange,
        network: &'static Network,
        runtime_cache: &'static RuntimeCache,
        runtime_config: &'static RuntimeConfig,
    ) -> Result<Vec<Market>, Error>;

    fn calculate_amount_out(
        &self,
        market: &Market,
        reserves: &Reserves,
        input_amount: &U256,
    ) -> U256;

    fn populate_swap(&self, swap_log: &SwapLog, recipient: &H160) -> Result<Bytes, AbiError>;

    fn parse_balance_change(
        &self,
        logs: &TransactionLog,
        runtime_cache: &'static RuntimeCache,
    ) -> Vec<BalanceChange>;

    async fn get_market_reserves(
        &self,
        markets: Vec<&'static OrgValue<Market>>,
        runtime_cache: &'static RuntimeCache,
        runtime_config: &RuntimeConfig,
    ) -> OrganizedList<Reserves>;
}

*/

pub async fn get_exchange_markets(
    network: &'static Network,
    runtime_cache: &RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> Result<Vec<Market>, Error> {
    let mut result: Vec<Market> = vec![];

    for exchange in &network.exchanges {
        if exchange.protocol == Protocol::UniswapV2 {
            if let Ok(mut response) =
                uniswap_v2::get_markets(exchange, network, runtime_cache, runtime_config).await
            {
                result.append(&mut response);
            };
        } else if exchange.protocol == Protocol::StableSwap {
            // return stable_swap::get_markets(exchange);
        }
    }

    return Ok(result);
}

pub fn init_exchange_handlers() {
    let _ = &EXECUTE_TX_BUNDLE_FUNCTION.name;
    uniswap_v2::init_handler();
}

#[inline(always)]
pub fn parse_balance_changes(
    logs: &Vec<TransactionLog>,
    runtime_cache: &'static RuntimeCache,
) -> Vec<BalanceChange> {
    let mut result: Vec<BalanceChange> = vec![];

    // Uniswap V2
    result.append(&mut uniswap_v2::parse_balance_changes(
        logs.iter()
            .filter(|x| x.protocol == Protocol::UniswapV2)
            .collect(),
        runtime_cache,
    ));

    return result;
}

#[inline(always)]
pub async fn get_market_reserves(
    markets: &'static OrganizedList<Market>,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
) -> OrganizedList<Reserves> {
    let filtered_markets: Vec<&OrgValue<Market>> = markets.filter(|x| {
        x.value.protocol == Protocol::UniswapV2 || x.value.protocol == Protocol::StableSwap
    });

    // Uniswap V2
    let uniswap_v2_markets =
        uniswap_v2::get_market_reserves(filtered_markets, runtime_cache, runtime_config).await;

    return uniswap_v2_markets;
}

#[inline(always)]
pub fn calculate_amount_out(reserves: &Reserves, input_amount: &U256, market: &Market) -> U256 {
    let protocol = &market.protocol;

    if protocol == &Protocol::UniswapV2
        || (protocol == &Protocol::StableSwap && market.stable == false)
    {
        return uniswap_v2::calculate_amount_out(market, reserves, input_amount);
    }

    return U256::zero();
}

#[inline(always)]
pub fn populate_swap(swap_log: &SwapLog, recipient: &H160) -> Result<Bytes, AbiError> {
    match swap_log.market.value.protocol {
        Protocol::StableSwap => return Err(AbiError::WrongSelector),
        Protocol::UniswapV2 => {
            return uniswap_v2::populate_swap(&swap_log, recipient);
        }
    }
}
