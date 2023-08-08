use ethers::{
    types::U256,
    types::U512,
    utils::{parse_units, WEI_IN_ETHER},
};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    env::{RuntimeCache, RuntimeConfig},
    exchanges::{self, calculate_circ_liquidity_step},
    networks::Network,
    RUNTIME_ROUTES,
};

use super::{market::Market, MarketState, OrgValue, OrganizedList, PriceTable, SwapLog, Token};

const ZERO_VALUE: U256 = U256::zero();

lazy_static! {}

#[derive(Debug, Clone)]
pub struct Route {
    pub markets: Vec<&'static OrgValue<Market>>,
    pub base_token: &'static Token,

    //private
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
        reserve_table: &OrganizedList<MarketState>,
        price_table: &PriceTable,
    ) -> Option<RouteResult> {
        let circ_liquidity = self.calculate_circ_liquidity(reserve_table);

        let (fee_multiplier, multiplier) = self.markets[0].value.get_fee_data();
        let first_market = &self.markets[0].value;

        let liq_sqrt = match first_market.protocol {
            exchanges::types::Protocol::UniswapV2 => {
                ((circ_liquidity.0 * circ_liquidity.1 * fee_multiplier) / multiplier).integer_sqrt()
            }
            exchanges::types::Protocol::StableSwap => {
                if first_market.stable == true {
                    let token_in_pow: U256 = parse_units(1, first_market.tokens[0].decimals)
                        .unwrap()
                        .into();
                    let token_out_pow: U256 = parse_units(1, first_market.tokens[1].decimals)
                        .unwrap()
                        .into();

                    let reserve_0 = circ_liquidity.0 * WEI_IN_ETHER / token_in_pow;
                    let reserve_1 = circ_liquidity.1 * WEI_IN_ETHER / token_out_pow;

                    let liquidity = exchanges::get_f(&reserve_0, &reserve_1) / WEI_IN_ETHER;

                    ((liquidity * fee_multiplier) / multiplier).integer_sqrt()
                    // ((circ_liquidity.0 * circ_liquidity.1 * fee_multiplier) / multiplier)
                    //     .integer_sqrt()
                } else {
                    ((circ_liquidity.0 * circ_liquidity.1 * fee_multiplier) / multiplier)
                        .integer_sqrt()
                }
            }
        };

        if self.markets.par_iter().any(|x| x.value.stable == true) {
            if liq_sqrt > circ_liquidity.0 {
                let input_amount: U256 =
                    (liq_sqrt - circ_liquidity.0) * multiplier / fee_multiplier;

                let result = match self.calculate_circ_profit(
                    reserve_table,
                    price_table,
                    input_amount,
                    self.base_token,
                ) {
                    Some(res) => res.ref_profit_loss,
                    None => U256::zero(),
                };

                // let result = match self.calculate_circ_profit(
                //     reserve_table,
                //     price_table,
                //     input_amount,
                //     self.base_token){
                //         Some(res) => res.ref_profit_loss,
                //         None => U256::zero(),
                //     };
                // if let Some(s) = self.calculate_circ_profit(
                //     reserve_table,
                //     price_table,
                //     input_amount,
                //     self.base_token,
                // ) {
                //     println!("{:#?}", (s.ref_profit_loss));
                // };
            }
        }
        return None;

        //         // let f = U256::from(input_amount.into());
        //     return self.calculate_circ_profit(
        //         reserve_table,
        //         price_table,
        //         U256::from(input_amount.as_u128()),
        //         self.base_token,
        //     );
        // }
        // else{
        //     return None;
        // }
    }

    #[inline(always)]
    fn calculate_circ_liquidity(&self, reserve_table: &OrganizedList<MarketState>) -> (U256, U256) {
        let first_reserve = &reserve_table[self.markets[0].id];
        let mut token_in = self.base_token;

        let mut res = first_reserve.value.get_reserves();
        if self.markets[0].value.tokens[0].ne(token_in) {
            res = (res.1, res.0);
        }

        for i in 1..self.markets.len() {
            let market = self.markets[i];
            let market_org_value = &reserve_table[market.id];

            let market_reserves = market_org_value.value.get_reserves();
            if market.value.tokens[0].eq(token_in) {
                res = calculate_circ_liquidity_step(&market.value, market_reserves, &res, token_in);
                token_in = market.value.tokens[1];
            } else {
                res = calculate_circ_liquidity_step(
                    &market.value,
                    (market_reserves.1, market_reserves.0),
                    &res,
                    token_in,
                );
                token_in = market.value.tokens[0];
            }
        }

        return res;
    }

    #[inline(always)]
    fn calculate_circ_profit(
        &self,
        reserve_table: &OrganizedList<MarketState>,
        price_table: &PriceTable,
        mut input_amount: U256,
        mut token_in: &'static Token,
    ) -> Option<RouteResult> {
        let _start_balance = input_amount;
        let mut swap_transactions: OrganizedList<SwapLog> = OrganizedList::new();

        for market in &self.markets {
            let market_state = &reserve_table[market.id].value;
            let market_value = &market.value;
            let token_0 = market_value.tokens[0];

            // println!("test");
            if token_in == token_0 {
                input_amount = market_value.amount_out(&market_state, &input_amount, token_in);
                // market_value.amount_out(&market_state, &input_amount, token_in);
                token_in = market_value.tokens[1];

                swap_transactions.add_value(SwapLog {
                    market: &market,
                    amount_0_out: ZERO_VALUE,
                    amount_1_out: input_amount,
                });
            } else {
                // market_value.amount_out(&market_state, &input_amount, token_in);
                input_amount = market_value.amount_out(&market_state, &input_amount, token_in);
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

        // return None;
    }

    #[inline(always)]
    pub fn contains_any_market(&self, market_ids: &Vec<usize>) -> bool {
        for local_market_id in &self.market_ids {
            if market_ids.iter().any(|x| x == local_market_id) {
                return true;
            }
        }

        return false;
    }

    #[inline(always)]
    pub fn new(markets: Vec<&'static OrgValue<Market>>, base_token: &'static Token) -> Route {
        // let market_fee_data: Vec<(&U256, &U256)> =
        // markets.iter().map(|x| x.value.get_fee_data()).collect_vec();

        let market_ids: Vec<usize> = markets.iter().map(|x| x.id).collect_vec();

        return Route {
            markets,
            base_token,
            // market_fee_data,
            market_ids,
        };
    }

    pub fn generate_from_runtime(
        network: &'static Network,
        config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
    ) -> usize {
        let base_tokens: Vec<&'static Token> = network
            .tokens
            .iter()
            .filter(|x| x.flash_loan_enabled)
            .collect();

        let len = runtime_cache.markets.len();
        base_tokens.par_iter().for_each(|&token| {
            generate_from_token(
                runtime_cache.markets.to_vec(),
                token,
                token,
                config.route_restraints,
                vec![],
                len,
            );
        });

        return RUNTIME_ROUTES.read().unwrap().len();
    }
}

fn generate_from_token(
    markets: Vec<&'static OrgValue<Market>>,
    token_in: &'static Token,
    base_token: &'static Token,
    route_restraints: (usize, usize),
    route_markets: Vec<&'static OrgValue<Market>>,
    market_count: usize,
) {
    return get_pairable_markets(token_in, markets.clone())
        .par_iter()
        .for_each(|&market| {
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

                    {
                        let mut f = RUNTIME_ROUTES.write().unwrap();
                        f.push(Route::new(current_route_markets, base_token));
                    }

                    // routes.push(Route::new(current_route_markets, base_token));
                } else if route_restraints.1 >= 1 && market_count > 1 {
                    let filtered_markets: Vec<&OrgValue<Market>> = markets
                        .iter()
                        .filter(|&x| x.value.contract_address.ne(&market.value.contract_address))
                        .map(|&x| x)
                        .collect();

                    let mut current_route_markets = route_markets.clone();
                    current_route_markets.push(&market);

                    generate_from_token(
                        filtered_markets,
                        current_token,
                        base_token,
                        (route_restraints.0, route_restraints.1 - 1),
                        current_route_markets,
                        market_count - 1,
                    );
                }
            }
        });
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
