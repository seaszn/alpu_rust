use core::ops::Deref;
use core::ops::DerefMut;
use ethers::prelude::k256::sha2::digest::typenum::private::InternalMarker;
use ethers::{
    providers::Middleware,
    types::U256,
    utils::{parse_units, ParseUnits},
};
use futures::executor::block_on;
use lazy_static::__Deref;
use serde_json::Value;
use tokio::time::Interval;
use std::time::Duration;
use std::time::Instant;
use std::{io::Error, thread};
use tokio::sync::RwLock;

use crate::env::RuntimeConfig;
use crate::exchanges::get_market_reserves;
use crate::{
    env::RuntimeCache,
    networks::Network,
    types::{PriceTable, Token},
};

lazy_static! {
    static ref REF_PRICE_TAB: RwLock<PriceTable> = RwLock::new(PriceTable::new());
}

pub struct PriceOracle {
    // network: &'static Network,
    runtime_cache: &'static RuntimeCache,
    runtime_config: &'static RuntimeConfig,
    join_handle: Option<tokio::task::JoinHandle<()>>,
}
unsafe impl Send for PriceOracle {}

impl PriceOracle {
    pub fn new(
        network: &'static Network,
        runtime_cache: &'static RuntimeCache,
        runtime_config: &'static RuntimeConfig,
    ) -> PriceOracle {
        let result = PriceOracle {
            // network,
            join_handle: None,
            runtime_cache,
            runtime_config,
        };

        return result;
    }

    #[inline(always)]
    pub fn initiate_market_updates(&mut self, interval: Duration) {
        let cache_reference = self.runtime_cache;
        let config_reference = self.runtime_config;

        self.join_handle = Some(tokio::spawn(async move {
            let mut run_interval = tokio::time::interval(interval);
            
            loop {
                run_interval.tick().await;
                let inst = Instant::now();

                let reserve_table = get_market_reserves(
                    &cache_reference.markets,
                    &cache_reference,
                    &config_reference,
                ).await;
                
                println!("test {:?}", inst.elapsed());
                
            }
            // }));
            // {
            //     let mut w_refrence = block_on(REF_PRICE_TAB.write());
            //     w_refrence.update_table(vec![]);
            // }

            // thread::sleep(interval);
            // }
        }));
    }

    // pub async fn update_price_table(&mut self) {
    //     if let Ok(json_response) =
    //         ureq::get("http://api.coinbase.com/v2/exchange-rates?currency=ETH")
    //             .call()
    //             .unwrap()
    //             .into_string()
    //     {
    //         let s: Value = serde_json::from_str(json_response.as_str()).unwrap();
    //         let value_map = s.as_object().unwrap()["data"].as_object().unwrap()["rates"]
    //             .as_object()
    //             .unwrap();

    //         let base_tokens: Vec<&'static Token> = self
    //             .network
    //             .tokens
    //             .iter()
    //             .filter(|x| x.ref_symbol.is_some())
    //             .collect();

    //         let weth_token = &self.network.tokens[0];

    //         let mut new_price_table = PriceTable::new();
    //         for token in base_tokens {
    //             let symbol: &String = token.ref_symbol.as_ref().unwrap();
    //             let token_ref_price =
    //                 1f64 / value_map[symbol].as_str().unwrap().parse::<f64>().unwrap();

    //             if let Ok(ParseUnits::U256(ref_price)) =
    //                 parse_units(token_ref_price.to_string(), weth_token.decimals)
    //             {
    //                 new_price_table.update_value(&token, ref_price);
    //             }
    //         }

    //         self.ref_price_table = new_price_table;

    //         if let Ok(flash_loan_response) = self
    //             .runtime_cache
    //             .bundle_executor
    //             .get_flash_loan_fees()
    //             .await
    //         {
    //             self.flash_loan_fee = U256::from(flash_loan_response)
    //         }

    //         if let Ok(balance_response) = self
    //             .runtime_cache
    //             .client
    //             .get_balance(self.runtime_cache.client.address(), None)
    //             .await
    //         {
    //             self.wallet_balance = balance_response;
    //         }
    //     }
    // }

    #[inline(always)]
    pub async fn get_ref_price_table(&self) -> tokio::sync::RwLockReadGuard<'_, PriceTable> {
        return REF_PRICE_TAB.read().await;
    }

    // #[inline(always)]
    // pub fn get_wallet_balance(&self) -> &U256 {
    //     // return &self.wallet_balance;
    // }

    // #[inline(always)]
    // pub fn get_flash_loan_fee(&self) -> &U256 {
    //     return &self.flash_loan_fee;
    // }

    // #[inline(always)]
    // pub fn get_gas_price(&self) -> &U256{
    //     return &U256::zero();
    //     // return &self.gas_price;
    // }
}
