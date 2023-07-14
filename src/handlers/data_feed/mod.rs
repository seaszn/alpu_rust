use std::sync::Arc;

use tokio::sync::mpsc::Sender;

// pub mod relay_message;
use crate::env::*;

use super::types::*;

#[async_trait::async_trait]
pub trait DataFeed {
    async fn init(
        &self,
        sender: Sender<Vec<BalanceChange>>,
        runtime_config: &RuntimeConfig,
        runtime_cache: Arc<RuntimeCache>,
    ) -> websocket_lite::Result<()>;
}
