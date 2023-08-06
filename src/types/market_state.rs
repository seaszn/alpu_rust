use super::BalanceChange;
use crate::exchanges::{UniswapV2MarketState, StableSwapMarketState};

#[derive(Clone, Copy, Debug)]
pub enum MarketState {
    UniswapV2(UniswapV2MarketState),
    StableSwap(StableSwapMarketState)
}

impl MarketState {
    pub fn update_with_balance_change(&self, balance_change: &BalanceChange) -> MarketState {
        match self {
            MarketState::UniswapV2(res) => {
                let mut result = *res;

                result.0 = (result.0 + balance_change.amount_0_in) - balance_change.amount_0_out;
                result.1 = (result.1 + balance_change.amount_1_in) - balance_change.amount_1_out;

                return MarketState::UniswapV2(result);
            }
            MarketState::StableSwap(res) => {
                let mut result = *res;

                result.0 = (result.0 + balance_change.amount_0_in) - balance_change.amount_0_out;
                result.1 = (result.1 + balance_change.amount_1_in) - balance_change.amount_1_out;

                return MarketState::StableSwap(result);
            }
        }
    }

    pub fn get_reserves(&self) -> UniswapV2MarketState {
        match self {
            MarketState::UniswapV2(res) => *res,
            MarketState::StableSwap(res) => (res.0.pow(4.into()), res.1.pow(4.into())),
        }
    }
}
