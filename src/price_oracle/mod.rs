use ethers::providers::Middleware;
use ethers::types::U256;
use ethers::types::U64;
use ethers::utils::parse_units;
use ethers::utils::ParseUnits;
use serde_json::Value;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::RwLock;

use crate::env::RuntimeConfig;
use crate::exchanges::get_market_reserves;
use crate::types::MarketState;
use crate::types::OrganizedList;
use crate::types::PriceTable;
use crate::types::Token;
use crate::{env::RuntimeCache, networks::Network};

use self::base_table::get_base_price_table;
mod base_table;

lazy_static! {
    static ref MARKET_RESERVE_TABLE: RwLock<OrganizedList<MarketState>> =
        RwLock::new(OrganizedList::new());
    static ref REF_PRICE_TABLE: RwLock<PriceTable> = RwLock::new(PriceTable::new());
    static ref NEW_BLOCK_NUMBER: atomic::AtomicU64 = AtomicU64::new(0);
    static ref FLASH_LOAN_FEE: RwLock<U256> = RwLock::new(U256::zero());
    static ref WALLET_BALANCE: RwLock<U256> = RwLock::new(U256::zero());
    static ref GAS_PRICE: RwLock<U256> =
        RwLock::new(U256::from(parse_units("0.1", "gwei").unwrap()));
}

pub struct PriceOracle {
    network: &'static Network,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
    market_join_handle: Option<thread::JoinHandle<()>>,
    daily_join_handle: Option<thread::JoinHandle<()>>,
}
unsafe impl Send for PriceOracle {}

impl PriceOracle {
    pub fn new(
        network: &'static Network,
        runtime_cache: &'static RuntimeCache,
        runtime_config: &'static RuntimeConfig,
    ) -> PriceOracle {
        let result = PriceOracle {
            network,
            runtime_cache,
            runtime_config,
            market_join_handle: None,
            daily_join_handle: None,
        };

        return result;
    }

    #[inline(always)]
    pub fn initiate(&mut self) {
        // self.initiate_block_updates();
        self.initiate_daily_updates(Duration::from_secs(60 * 60 * 24));
        self.initiate_market_updates(Duration::from_millis(1000));
    }

    #[inline(always)]
    fn initiate_market_updates(&mut self, interval: Duration) {
        let cache_reference = self.runtime_cache;
        let config_reference = self.runtime_config;

        let handle: Handle = Handle::current();
        let mut run_interval = tokio::time::interval(interval);

        self.market_join_handle = Some(thread::spawn(move || {
            handle.spawn(async move {
                loop {
                    let mut reserve_table: crate::types::OrganizedList<MarketState> =
                        get_market_reserves(
                            &cache_reference.markets,
                            &cache_reference,
                            &config_reference,
                        )
                        .await;

                    {
                        let mut w_refrence = MARKET_RESERVE_TABLE.write().await;
                        w_refrence.update_all(&mut reserve_table);
                    }

                    if let Ok(block_number) = cache_reference.client.get_block_number().await {
                        NEW_BLOCK_NUMBER.store(block_number.as_u64(), Ordering::SeqCst)
                    }

                    run_interval.tick().await;
                }
            });

            let _guard = handle.enter();
        }));
    }

    fn initiate_daily_updates(&mut self, interval: Duration) {
        let cache_reference = self.runtime_cache;
        let network_reference = self.network;

        let handle = Handle::current();
        let mut run_interval = tokio::time::interval(interval);

        self.daily_join_handle = Some(thread::spawn(move || {
            handle.spawn(async move {
                loop {
                    let json_response = {
                        if let Ok(res) =
                            ureq::get("http://api.coinbase.com/v2/exchange-rates?currency=ETH")
                                .call()
                        {
                            res.into_string().unwrap()
                        } else {
                            get_base_price_table()
                        }
                    };

                    let s: Value = serde_json::from_str(json_response.as_str()).unwrap();
                    let value_map = s.as_object().unwrap()["data"].as_object().unwrap()["rates"]
                        .as_object()
                        .unwrap();

                    let base_tokens: Vec<&'static Token> = network_reference
                        .tokens
                        .iter()
                        .filter(|x| x.ref_symbol.is_some())
                        .collect();

                    let weth_token = &network_reference.tokens[0];

                    let mut new_price_table = PriceTable::new();
                    for token in base_tokens {
                        let symbol: &String = token.ref_symbol.as_ref().unwrap();
                        let token_ref_price =
                            1f64 / value_map[symbol].as_str().unwrap().parse::<f64>().unwrap();

                        if let Ok(ParseUnits::U256(ref_price)) =
                            parse_units(token_ref_price.to_string(), weth_token.decimals)
                        {
                            new_price_table.update_value(&token, ref_price);
                        }
                    }

                    {
                        let mut w_refrence = REF_PRICE_TABLE.write().await;
                        *w_refrence = new_price_table;
                    }

                    if let Ok(flash_loan_response) =
                        cache_reference.bundle_executor.get_flash_loan_fees().await
                    {
                        {
                            let mut w_refrence = FLASH_LOAN_FEE.write().await;
                            *w_refrence = U256::from(flash_loan_response);
                        }
                    }

                    if let Ok(balance_response) = cache_reference
                        .client
                        .get_balance(cache_reference.client.address(), None)
                        .await
                    {
                        {
                            let mut w_refrence = WALLET_BALANCE.write().await;

                            *w_refrence = balance_response;
                        }
                    }

                    run_interval.tick().await;
                }
            });

            let _guard = handle.enter();
        }));
    }

    #[inline(always)]
    pub async fn get_market_reserves(&self) -> OrganizedList<MarketState> {
        return MARKET_RESERVE_TABLE.read().await.clone();
    }

    #[inline(always)]
    pub async fn get_wallet_balance(&self) -> U256 {
        return WALLET_BALANCE.read().await.clone();
    }

    #[inline(always)]
    pub async fn get_flash_loan_fee(&self) -> U256 {
        return FLASH_LOAN_FEE.read().await.clone();
    }

    #[inline(always)]
    pub fn get_block_number() -> U64 {
        return U64::from(NEW_BLOCK_NUMBER.load(Ordering::SeqCst));
    }

    #[inline(always)]
    pub async fn get_gas_price(&self) -> U256 {
        return GAS_PRICE.read().await.clone();
    }

    pub async fn get_ref_price_table(&self) -> PriceTable{
        return REF_PRICE_TABLE.read().await.clone();
    }
}
