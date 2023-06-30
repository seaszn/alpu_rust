use std::sync::Arc;

use crate::{
    networks::{get_network_instance, Network},
    types::Token,
};
use dotenv::dotenv;
use ethers::prelude::{k256::ecdsa::SigningKey, *};

use self::runtime::{BundleExecutor, UniswapQuery};

pub mod runtime;
pub struct Environment {
    cache: runtime::Cache,
    pub config: runtime::Config,
    pub network: Network,
}

impl Environment {
    pub async fn init() -> Arc<Environment> {
        return local_init().await;
    }

    pub fn get_client(self) -> Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
        return self.cache.get_client();
    }

    pub fn get_bundle_executor(
        self,
    ) -> Arc<BundleExecutor<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        return self.cache.get_bundle_executor();
    }

    pub fn get_uniswap_query(
        self,
    ) -> Arc<UniswapQuery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        return self.cache.get_uniswap_query();
    }

    pub fn get_tokens(self) -> Arc<Vec<Arc<Token>>> {
        return self.cache.get_tokens();
    }
}

async fn local_init() -> Arc<Environment> {
    dotenv().ok();

    let config: runtime::Config = runtime::get_runtime_config();
    let network: Network = get_network_instance(&config.chain_id);
    let runtime: runtime::Cache = runtime::get_runtime_cache(&config, &network).await;

    return Arc::new(Environment {
        network,
        config,
        cache: runtime,
    });
}
