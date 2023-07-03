use std::sync::Arc;

use ethers::prelude::*;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::{prelude::SignerMiddleware, providers::Provider};

use super::cache::{BundleExecutor, UniswapQuery};

pub type RuntimeClient = Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
pub type BundleExecutorContract = Arc<BundleExecutor<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>;
pub type UniswapQueryContract = Arc<UniswapQuery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>;
