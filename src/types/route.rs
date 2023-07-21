use std::io::Error;

use ethers::types::U256;
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    env::{RuntimeCache, RuntimeConfig},
    networks::Network,
};

use super::{
    market::Market, reserves::ReverseReserves, OrgValue, OrganizedList, PriceTable, Reserves,
    SwapLog, Token,
};

const ZERO_VALUE: U256 = U256::zero();

#[derive(Debug, Clone)]
pub struct Route {
    pub markets: Vec<&'static OrgValue<Market>>,
    pub base_token: &'static Token,

    //private
    market_fee_data: Vec<(&'static U256, &'static U256)>,
    market_ids: Vec<usize>,
}
pub struct RouteResult {
    pub base_token: &'static Token,
    pub start_balance: U256,
    pub end_balance: U256,
    pub profit_loss: U256,
    pub ref_profit_loss: U256,
    pub transactions: OrganizedList<SwapLog>,
}

impl Route {
    #[inline(always)]
    pub fn calculate_result(
        &self,
        reserve_table: &OrganizedList<Reserves>,
        price_table: &'static PriceTable,
        affected_markets: &Vec<usize>,
    ) -> Option<RouteResult> {
        if self.contains_any_market(affected_markets) {
            let liquidity: Reserves = self.calculate_circ_liquidity(reserve_table);
            let (fee_multiplier, multiplier) = self.market_fee_data[0];

            let feed_liquidity_sqrt =
                ((liquidity.0 * liquidity.1 * fee_multiplier) / multiplier).integer_sqrt();

            if feed_liquidity_sqrt > liquidity.0 {
                let input_amount =
                    (feed_liquidity_sqrt - liquidity.0) * multiplier / fee_multiplier;

                return self.calculate_circ_profit(
                    reserve_table,
                    price_table,
                    input_amount,
                    self.base_token,
                );
            }
        }

        return None;
    }

    #[inline(always)]
    fn calculate_circ_liquidity(&self, reserve_table: &OrganizedList<Reserves>) -> Reserves {
        let first_reserve = &reserve_table[self.markets[0].id];
        let mut token_in = self.base_token;

        let mut res: Reserves = first_reserve.value;
        if self.markets[0].value.tokens[0].ne(token_in) {
            res = (first_reserve.value.1, first_reserve.value.0)
        }

        for i in 1..self.markets.len() {
            let market = self.markets[i];
            let (fee_multiplier, mul) = self.market_fee_data[i];
            let market_reserve = &reserve_table[market.id];

            let reserve_0 = &market_reserve.value.0;
            let reserve_1 = &market_reserve.value.1;
            let res_mul = (fee_multiplier * res.1) / mul;

            if token_in.eq(&market.value.tokens[0]) {
                let delta = reserve_0 + res_mul;
                res.0 = (res.0 * reserve_0) / delta;
                res.1 = (res_mul * reserve_1 / mul) / delta;

                token_in = &market.value.tokens[1];
            } else {
                let delta = reserve_1 + res_mul;
                res.0 = (res.0 * reserve_1) / delta;
                res.1 = (res_mul * reserve_0) / delta; //TODO: if results weird, change to reserve 1

                token_in = &market.value.tokens[0];
            }
        }

        return res;
    }

    #[inline(always)]
    fn calculate_circ_profit(
        &self,
        reserve_table: &OrganizedList<Reserves>,
        price_table: &'static PriceTable,
        mut input_amount: U256,
        mut token_in: &'static Token,
    ) -> Option<RouteResult> {
        let _start_balance = input_amount;
        let mut swap_transactions: OrganizedList<SwapLog> = OrganizedList::new();

        for market in &self.markets {
            let reserves: Reserves = reserve_table[market.id].value;
            let market_value = market.value;
            let token_0 = market_value.tokens[0];


            if token_in == token_0 {
                input_amount = market_value.amount_out(&reserves, &input_amount);
                token_in = market_value.tokens[1];
                
                swap_transactions.add_value(SwapLog {
                    market: &market,
                    amount_0_out: ZERO_VALUE,
                    amount_1_out: input_amount,
                });

            } else {
                input_amount = market_value.amount_out(&reserves.reverse(), &input_amount);
                token_in = token_0;

                swap_transactions.add_value(SwapLog {
                    market: &market,
                    amount_0_out: input_amount,
                    amount_1_out: ZERO_VALUE,
                });
            }
        }

        if input_amount > _start_balance {
            let profit_loss = input_amount - _start_balance;
            return Some(RouteResult {
                base_token: self.base_token,
                start_balance: _start_balance,
                end_balance: input_amount,
                profit_loss,
                ref_profit_loss: price_table.get_ref_price(self.base_token, profit_loss),
                transactions: swap_transactions,
            });
        } else {
            None
        }
    }

    #[inline(always)]
    fn contains_any_market(&self, market_ids: &Vec<usize>) -> bool {
        for local_market_id in &self.market_ids {
            if market_ids.iter().any(|x| x == local_market_id) {
                return true;
            }
        }

        return false;
    }

    #[inline(always)]
    pub fn new(markets: Vec<&'static OrgValue<Market>>, base_token: &'static Token) -> Route {
        let market_fee_data: Vec<(&U256, &U256)> =
            markets.iter().map(|x| x.value.get_fee_data()).collect_vec();

        let market_ids: Vec<usize> = markets.iter().map(|x| x.id).collect_vec();

        return Route {
            markets,
            base_token,
            market_fee_data,
            market_ids,
        };
    }

    pub fn generate_from_runtime(
        network: &'static Network,
        config: &'static RuntimeConfig,
        runtime_cache: &'static Result<RuntimeCache, Error>,
    ) -> Vec<Route> {
        if let Ok(cache) = runtime_cache {
            let base_tokens: Vec<&'static Token> = network
                .tokens
                .iter()
                .filter(|x| x.flash_loan_enabled)
                .collect();

            let len = cache.markets.len();
            return base_tokens
                .par_iter()
                .flat_map(|base_token| {
                    generate_from_token(
                        cache.markets.to_vec(),
                        &base_token,
                        &base_token,
                        config.route_restraints,
                        vec![],
                        len,
                    )
                })
                .collect();
        }

        return vec![];
    }
}

fn generate_from_token(
    markets: Vec<&'static OrgValue<Market>>,
    token_in: &'static Token,
    base_token: &'static Token,
    route_restraints: (usize, usize),
    route_markets: Vec<&'static OrgValue<Market>>,
    market_count: usize,
) -> Vec<Route> {
    return get_pairable_markets(token_in, markets.clone())
        .par_iter()
        .flat_map(|&market| {
            let mut routes: Vec<Route> = vec![];

            let pending_current_token: Option<&'static Token>;
            if token_in.contract_address == market.value.tokens[0].contract_address {
                pending_current_token = Some(&market.value.tokens[1]);
            } else {
                pending_current_token = Some(&market.value.tokens[0]);
            }

            if let Some(current_token) = pending_current_token {
                if current_token.eq(&base_token)
                    && (route_markets.len() + 1).ge(&route_restraints.0)
                {
                    let mut current_route_markets = route_markets.clone();
                    current_route_markets.push(&market);

                    routes.push(Route::new(current_route_markets, base_token));
                } else if route_restraints.1 >= 1 && market_count > 1 {
                    let filtered_markets: Vec<&OrgValue<Market>> = markets
                        .iter()
                        .filter(|&x| x.value.contract_address.ne(&market.value.contract_address))
                        .map(|&x| x)
                        .collect();

                    let mut current_route_markets = route_markets.clone();
                    current_route_markets.push(&market);

                    let mut m = generate_from_token(
                        filtered_markets,
                        current_token,
                        base_token,
                        (route_restraints.0, route_restraints.1 - 1),
                        current_route_markets,
                        market_count - 1,
                    );

                    routes.append(&mut m);
                }
            }

            routes
        })
        .collect();
}

#[inline(always)]
fn get_pairable_markets(
    token_in: &'static Token,
    markets: Vec<&'static OrgValue<Market>>,
) -> Vec<&'static OrgValue<Market>> {
    let mut result: Vec<&OrgValue<Market>> = vec![];
    for market in markets {
        if token_in.eq(market.value.tokens[0]) || token_in.eq(market.value.tokens[1]) {
            result.push(market);
        }
    }

    return result;
}
