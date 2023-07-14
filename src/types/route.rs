use std::sync::Arc;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::{market::Market, Token};

#[derive(Debug, Clone)]
pub struct Route {
    pub markets: Vec<Arc<Market>>,
    pub base_token: Arc<Token>,
}

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

fn append_one<T>(s: &Vec<T>, ele: &T) -> Vec<T>
where
    T: Clone,
{
    let mut res = s.clone();
    res.push(ele.clone());

    return res.to_vec();
}

fn filter_all<F, T>(source: &Vec<T>, mut predicate: F) -> Vec<T>
where
    F: FnMut(&T) -> bool,
    T: Clone,
{
    let mut res: Vec<T> = vec![];
    for ele in source {
        if predicate(ele) {
            res.push(ele.clone());
        }
    }

    return res;
}
