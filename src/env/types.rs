use ethers::abi::{AbiParser, Function};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::{prelude::SignerMiddleware, providers::Provider};

use super::cache::{BundleExecutor, UniswapQuery};

pub type RuntimeClient = SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>;
pub type BundleExecutorContract = BundleExecutor<RuntimeClient>;
pub type UniswapQueryContract = UniswapQuery<RuntimeClient>;

lazy_static! {
    pub static ref EXECUTE_TX_BUNDLE_FUNCTION: Function = AbiParser::default()
        .parse_function("executeTxBundle(address,uint256,address[],bytes[])")
        .unwrap();
}
