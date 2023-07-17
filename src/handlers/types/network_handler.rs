use async_trait::async_trait;
use crate::env::{RuntimeConfig, RuntimeCache};

#[async_trait]
pub trait NetworkHandler {
    async fn init(&self, runtime_config: &'static  RuntimeConfig, runtime_cache: &'static RuntimeCache);
}
