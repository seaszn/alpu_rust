use ethers::{
    types::*,
    utils::{parse_units, ParseUnits},
};
use itertools::Itertools;

use super::Token;

#[derive(Clone, Debug)]
pub struct PriceTable {
    internal: Vec<(&'static H160, U256)>,
    token_dec_powers: Vec<(&'static H160, U256)>,
}

impl PriceTable {
    pub fn new() -> PriceTable {
        return PriceTable {
            internal: vec![],
            token_dec_powers: vec![],
        };
    }

    #[inline(always)]
    pub fn contains_key(&self, key: &H160) -> bool {
        return self.internal.iter().any(|x| x.0 == key);
    }

    #[inline(always)]
    pub fn get_value(&self, key: &H160) -> &U256 {
        return &self.internal.iter().find(|x| x.0 == key).unwrap().1;
    }

    #[inline(always)]
    pub fn update_value(&mut self, key: &'static Token, value: U256) {
        if let Some((position, _)) = self
            .internal
            .iter()
            .find_position(|x| x.0 == &key.contract_address)
        {
            self.internal[position].1 = value;
        } else {
            self.internal.push((&key.contract_address, value));

            if let Ok(ParseUnits::U256(token_dec_power)) = parse_units("1.0", key.decimals) {
                self.token_dec_powers
                    .push((&key.contract_address, token_dec_power));
            }
        }
    }

    #[inline(always)]
    pub fn get_ref_price(&self, token: &Token, input_amout: U256) -> U256 {
        let value = self.get_value(&token.contract_address);
        let token_dec_power = self
            .token_dec_powers
            .iter()
            .find(|x| x.0 == &token.contract_address)
            .unwrap();

        return (value * input_amout) / token_dec_power.1;
    }
}
