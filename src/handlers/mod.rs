use crate::networks;

mod arbitrum;

pub async fn init(chain_id: u32) {
    if chain_id == networks::arbitrum {
        _ = arbitrum::init().await;
    }
}