use crate::networks;

use self::{arbitrum::ArbitrumHandler, types::NetworkHandler};

mod arbitrum;
mod data_feed;
pub mod types;

pub struct Handler {}

impl Handler {
    pub async fn new<'a>(chain_id: u32) -> Option<&'a dyn NetworkHandler> {
        match chain_id {
            networks::arbitrum => return Some(&ArbitrumHandler),
            0_u32..=42160_u32 | 42162_u32..=u32::MAX => {
                return None;
            }
        }
    }
}