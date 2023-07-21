use std::io::Error;

use ethers::{
    providers::Middleware,
    types::U256,
    utils::{parse_units, ParseUnits},
};
use futures::executor::block_on;
use serde_json::Value;

use crate::{
    env::RuntimeCache,
    networks::Network,
    types::{PriceTable, Token},
};

pub struct PriceOracle {
    // private
    network: &'static Network,
    runtime_cache: &'static RuntimeCache,
    ref_price_table: PriceTable,
    flash_loan_fee: U256,
    wallet_balance: U256,
}
unsafe impl Send for PriceOracle {}

impl PriceOracle {
    pub fn new(
        network: &'static Network,
        runtime_cache: &'static Result<RuntimeCache, Error>,
    ) -> Option<PriceOracle> {
        if let Ok(cache) = runtime_cache {
            let mut result = PriceOracle {
                network,
                runtime_cache: cache,
                ref_price_table: PriceTable::new(),
                flash_loan_fee: U256::zero(),
                wallet_balance: U256::zero(),
            };

            block_on(result.update_price_table());
            return Some(result);
        }

        return None;
    }

    pub async fn update_price_table(&mut self) {
        if let Ok(json_response) =
            ureq::get("http://api.coinbase.com/v2/exchange-rates?currency=ETH")
                .call()
                .unwrap()
                .into_string()
        {
            let s: Value = serde_json::from_str(json_response.as_str()).unwrap();
            let value_map = s.as_object().unwrap()["data"].as_object().unwrap()["rates"]
                .as_object()
                .unwrap();

            let base_tokens: Vec<&'static Token> = self
                .network
                .tokens
                .iter()
                .filter(|x| x.ref_symbol.is_some())
                .collect();

            let weth_token = &self.network.tokens[0];

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

            self.ref_price_table = new_price_table;

            if let Ok(flash_loan_response) = self
                .runtime_cache
                .bundle_executor
                .get_flash_loan_fees()
                .await
            {
                self.flash_loan_fee = U256::from(flash_loan_response)
            }

            if let Ok(balance_response) = self
                .runtime_cache
                .client
                .get_balance(self.runtime_cache.client.address(), None)
                .await
            {
                self.wallet_balance = balance_response;
            }
        }
    }

    #[inline(always)]
    pub fn get_ref_price_table(&self) -> &PriceTable {
        return &self.ref_price_table;
    }
    
    #[inline(always)]
    pub fn get_wallet_balance(&self) -> &U256{
        return &self.wallet_balance;
    }

    #[inline(always)]
    pub fn get_flash_loan_fee(&self) -> &U256{
        return &self.flash_loan_fee;
    }
}
