use ethers::prelude::*;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::{prelude::SignerMiddleware, providers::Provider};

use super::cache::{BundleExecutor, UniswapQuery};

pub type RuntimeClient = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;
pub type BundleExecutorContract = BundleExecutor<RuntimeClient>;
pub type UniswapQueryContract = UniswapQuery<RuntimeClient>;
