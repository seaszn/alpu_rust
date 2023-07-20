use ethers::utils::{parse_units, ParseUnits};
use serde_json::Value;

use crate::{
    networks::Network,
    types::{PriceTable, Token},
};

pub struct PriceOracle {
    pub running: bool,

    // private
    network: &'static Network,
    ref_price_table: PriceTable,
}
unsafe impl Send for PriceOracle {}

impl PriceOracle {
    pub fn new(network: &'static Network) -> PriceOracle {
        let mut result = PriceOracle {
            running: false,
            network,
            ref_price_table: PriceTable::new(),
        };

        result.update_price_table();
        result.running = true;

        return result;
    }

    pub fn update_price_table(&mut self) {
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
        }
    }

    pub fn get_ref_price_table(&self) -> &PriceTable {
        return &self.ref_price_table;
    }
}
