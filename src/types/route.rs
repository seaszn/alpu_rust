use std::io::Error;

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    env::{RuntimeCache, RuntimeConfig},
    networks::Network,
};

use super::{market::Market, Token};

#[derive(Debug, Clone)]
pub struct Route {
    pub markets: Vec<&'static Market>,
    pub base_token: &'static Token,
}
pub struct RouteResult {}

impl Route {
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

            let runtime_markets = cache.markets.iter().map(|x| x).collect_vec();

            return base_tokens
                .par_iter()
                .flat_map(|base_token| {
                    generate_from_token(
                        runtime_markets.clone(),
                        &base_token,
                        &base_token,
                        config.route_restraints,
                        vec![],
                        runtime_markets.len(),
                    )
                })
                .collect();
        }

        return vec![];
    }
}

fn generate_from_token(
    markets: Vec<&'static Market>,
    token_in: &'static Token,
    base_token: &'static Token,
    route_restraints: (usize, usize),
    route_markets: Vec<&'static Market>,
    market_count: usize,
) -> Vec<Route> {
    let runtime_markets = markets.clone();
    return get_pairable_markets(token_in, runtime_markets)
        .par_iter()
        .flat_map(|&market| {
            let mut routes: Vec<Route> = vec![];

            let pending_current_token: Option<&'static Token>;

            if token_in.contract_address == market.tokens[0].contract_address {
                pending_current_token = Some(&market.tokens[1]);
            } else {
                pending_current_token = Some(&market.tokens[0]);
            }

            if let Some(current_token) = pending_current_token {
                if current_token.eq(&base_token)
                    && (route_markets.len() + 1).ge(&route_restraints.0)
                {
                    let mut current_route_markets = route_markets.clone();
                    current_route_markets.push(&market);

                    routes.push(Route {
                        markets: current_route_markets,
                        base_token,
                    })
                } else if route_restraints.1 >= 1 && market_count > 1 {
                    let filtered_markets: Vec<&'static Market> = markets
                        .iter()
                        .filter(|x| x.contract_address.ne(&market.contract_address.clone()))
                        .map(|x| *x)
                        .collect();

                    let mut current_route_markets = route_markets.clone();
                    current_route_markets.push(&market);

                    let mut child_routes = generate_from_token(
                        filtered_markets,
                        current_token,
                        base_token,
                        (route_restraints.0, route_restraints.1 - 1),
                        current_route_markets,
                        market_count - 1,
                    );

                    routes.append(&mut child_routes);
                }
            }

            routes
        })
        .collect();
}

/*
// pub fn calculate_result(&self, reserve_table: &ReserveTable) -> RouteResult {
//     if let Some(_route_liquidity) = self.calculate_circ_liquidity(reserve_table) {

//     }

//     return RouteResult {};
// }

// fn calculate_circ_liquidity(&self, reserves: &ReserveTable) -> Option<Reserves> {
//     if let Some(first_reserve) = reserves.get_value(&self.markets[0].contract_address) {
//         let mut token_in = &self.base_token;

//         let mut res: Reserves = first_reserve;
//         if self.markets[0].tokens[0].ne(token_in) {
//             res = (first_reserve.1, first_reserve.0)
//         }

//         for market in self.markets.split_first().unwrap().1 {
//             let (fee_multiplier, mul) = market.get_fee_data();
//             let market_reserve = &reserves.get_value(&market.contract_address).unwrap();

//             if token_in.eq(&market.tokens[0]) {
//                 let delta = market_reserve.0 + ((fee_multiplier * res.1) / mul);
//                 res.0 = &(res.0 * market_reserve.0) / delta;
//                 res.1 = &(fee_multiplier * res.1 * market_reserve.1 / mul) / delta;

//                 token_in = &market.tokens[1];
//             } else {
//                 let delta = market_reserve.1 + ((fee_multiplier * res.1) / mul);
//                 res.0 = &(res.0 * market_reserve.1) / delta;
//                 res.1 = &(fee_multiplier * res.1 * market_reserve.0 / mul) / delta;

//                 token_in = &market.tokens[0];
//             }
//         }

//         return Some(res);
//     }

// return None;
// }
// }
*/
// fn generate_internal(
//     markets: Vec<&'static Market>,
//     token_in: &'static Token,
//     base_token: &'static Token,
//     route_restraints: (usize, usize),
//     route_markets: Vec<&'static Market>,
// ) -> Vec<Route> {
//     return get_pairable_markets(token_in, markets.clone())
//         .par_iter()
//         .flat_map(|market| {
//             let mut routes: Vec<Route> = vec![];
//             // what is the current token going out of the market
//             let pending_current_token: Option<&'static Token>;
//             if token_in.contract_address == market.tokens[0].contract_address {
//                 pending_current_token = Some(market.tokens[1]);
//             } else {
//                 pending_current_token = Some(market.tokens[0]);
//             }

//             // check if something is going on properly
//             if let Some(current_token) = pending_current_token {
//                 if current_token
//                     .contract_address
//                     .eq(&base_token.contract_address)
//                     && route_markets.len().ge(&route_restraints.0)
//                 {
//                     routes.push(Route {
//                         markets: route_markets.clone(),
//                         base_token: base_token,
//                     });
//                     // println!("route");
//                 } else if route_restraints.1 >= 1 && markets.len() > 1 {
//                     let filtered_markets: Vec<&'static Market> = markets
//                         .clone()
//                         .into_iter()
//                         .filter(|x| x.contract_address.ne(&market.contract_address))
//                         .collect();

//                     let mut child_routes = generate_internal(
//                         filtered_markets,
//                         current_token,
//                         base_token,
//                         (route_restraints.0, route_restraints.1 - 1),
//                         append_one(&route_markets, &market),
//                     );

//                     routes.append(&mut child_routes);
//                 }
//             }

//             routes
//         })
//         .collect();
// }

fn get_pairable_markets(
    token_in: &'static Token,
    markets: Vec<&'static Market>,
) -> Vec<&'static Market> {
    return markets
        .into_iter()
        .filter(|x| token_in.eq(x.tokens[0]) || token_in.eq(x.tokens[1]))
        .collect();
}
