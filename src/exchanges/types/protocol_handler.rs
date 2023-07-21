use crate::env::RuntimeCache;
use async_trait::async_trait;
use ethers::{prelude::AbiError, types::Bytes};

#[async_trait]
pub trait ProtocolHandler {
    fn populate_swap(&self, runtime_cache: &'static RuntimeCache) -> Result<Bytes, AbiError>;
    fn get_markets(&self);
}
