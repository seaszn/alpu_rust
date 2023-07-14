use async_trait::async_trait;
use std::sync::Arc;
use crate::env::{RuntimeConfig, RuntimeCache};

#[async_trait]
pub trait NetworkHandler {
    async fn init(&self, runtime_config: Arc<RuntimeConfig>, runtime_cache: Arc<RuntimeCache>);
}
