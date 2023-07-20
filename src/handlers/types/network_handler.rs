use crate::env::{RuntimeCache, RuntimeConfig};
use async_trait::async_trait;
use crate::PriceOracle;

#[async_trait]
pub trait NetworkHandler {
    async fn init(
        &self,
        runtime_config: &'static RuntimeConfig,
        runtime_cache: &'static RuntimeCache,
        price_oracle: &'static PriceOracle,
    );
}
