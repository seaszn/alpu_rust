use super::Exchange;
use crate::types::Market;


pub fn get_markets<'c>(_exchange: Exchange) -> Vec<&'c Market<'c>> {
    return vec![];
}
