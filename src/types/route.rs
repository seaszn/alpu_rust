use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::{append_one, filter_all};

use super::{market::Market, ReserveTable, Reserves, Token};

#[derive(Debug, Clone)]
pub struct Route {
    pub markets: Vec<Arc<Market>>,
    pub base_token: Arc<Token>,
}
pub struct RouteResult {}

impl Route {
    pub fn generate_from_base_token(
        markets: Vec<Arc<Market>>,
        base_token: Arc<Token>,
        route_restraints: (usize, usize),
    ) -> Vec<Route> {
        return generate_internal(
            markets,
            base_token.clone(),
            base_token.clone(),
            route_restraints,
            vec![],
        );
    }

    pub fn calculate_result(&self, reserve_table: &ReserveTable) -> RouteResult {
        if let Some(_route_liquidity) = self.calculate_circ_liquidity(reserve_table) {

        }

        return RouteResult {};
    }

    fn calculate_circ_liquidity(&self, reserves: &ReserveTable) -> Option<Reserves> {
        if let Some(first_reserve) = reserves.get_value(&self.markets[0].contract_address) {
            let mut token_in = &self.base_token;

            let mut res: Reserves = first_reserve;
            if self.markets[0].tokens[0].ne(token_in) {
                res = (first_reserve.1, first_reserve.0)
            }

            for market in self.markets.split_first().unwrap().1 {
                let (fee_multiplier, mul) = market.get_fee_data();
                let market_reserve = &reserves.get_value(&market.contract_address).unwrap();

                if token_in.eq(&market.tokens[0]) {
                    let delta = market_reserve.0 + ((fee_multiplier * res.1) / mul);
                    res.0 = &(res.0 * market_reserve.0) / delta;
                    res.1 = &(fee_multiplier * res.1 * market_reserve.1 / mul) / delta;

                    token_in = &market.tokens[1];
                } else {
                    let delta = market_reserve.1 + ((fee_multiplier * res.1) / mul);
                    res.0 = &(res.0 * market_reserve.1) / delta;
                    res.1 = &(fee_multiplier * res.1 * market_reserve.0 / mul) / delta;

                    token_in = &market.tokens[0];
                }
            }

            return Some(res);
        }

        return None;
    }
}

fn generate_internal(
    markets: Vec<Arc<Market>>,
    token_in: Arc<Token>,
    base_token: Arc<Token>,
    route_restraints: (usize, usize),
    route_markets: Vec<Arc<Market>>,
) -> Vec<Route> {
    return get_pairable_markets(token_in.clone(), markets.clone())
        .par_iter()
        .flat_map(|market| {
            let mut routes: Vec<Route> = vec![];
            // what is the current token going out of the market
            let pending_current_token: Option<Arc<Token>>;
            if token_in.contract_address == market.tokens[0].contract_address {
                pending_current_token = Some(market.tokens[1].clone());
            } else {
                pending_current_token = Some(market.tokens[0].clone());
            }

            // check if something is going on properly
            if let Some(current_token) = pending_current_token {
                if current_token
                    .contract_address
                    .eq(&base_token.contract_address)
                    && route_markets.len().ge(&route_restraints.0)
                {
                    routes.push(Route {
                        markets: route_markets.clone(),
                        base_token: base_token.clone(),
                    });
                    // println!("route");
                } else if route_restraints.1 >= 1 && markets.len() > 1 {
                    let mut child_routes = generate_internal(
                        filter_all(&markets, |x| {
                            x.contract_address.ne(&market.contract_address)
                        }),
                        current_token.clone(),
                        base_token.clone(),
                        (route_restraints.0, route_restraints.1 - 1),
                        append_one(&route_markets, &market),
                    );

                    routes.append(&mut child_routes);
                }
            }

            routes
        })
        .collect();
}

fn get_pairable_markets(token_in: Arc<Token>, markets: Vec<Arc<Market>>) -> Vec<Arc<Market>> {
    return filter_all(&markets, |x| {
        token_in.eq(&x.tokens[0]) || token_in.eq(&x.tokens[1])
    });
}
