mod arbitrum_data_feed;
pub use arbitrum_data_feed::ArbitrumDataFeed;

use tokio::sync::mpsc::Sender;
use websocket_lite::Result;

use crate::env::*;
use crate::types::{OrganizedList, Reserves};

use crate::networks::ARBITRUM_CHAIN_ID;
#[async_trait::async_trait]
pub trait MarketDataFeed{
    async fn init(
        &self,
        sender: Sender<(OrganizedList<Reserves>, Vec<usize>)>,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
    ) -> Result<()>;
}

#[inline(always)]
pub fn get_network_data_feed(chain_id: u32) -> Option<&'static (dyn MarketDataFeed + Send + Sync)> {
    match chain_id {
        ARBITRUM_CHAIN_ID => return Some(&ArbitrumDataFeed),
        0_u32..=42160_u32 | 42162_u32..=u32::MAX => {
            return None;
        }
    }
}
