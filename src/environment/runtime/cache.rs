use ethers::{
    contract::abigen,
    middleware::SignerMiddleware,
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
};
use std::sync::Arc;

use crate::{
    networks::Network,
    types::Token,
    utils::json::deserialize_token_file,
};

use super::config::Config;

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

pub struct Cache {
    pub client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub uniswap_query: Arc<UniswapQuery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    pub bundle_executor: Arc<BundleExecutor<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>,
    pub tokens: Vec<Arc<Token>>,
}

pub async fn get_runtime_cache(config: &Config, network: &Network) -> Cache {
    let provider: Provider<Http> =
        Provider::<Http>::try_from(config.rpc_endpoint.clone()).expect("msg");

    let wallet = config
        .private_key
        .parse::<LocalWallet>()
        .expect("PRIVATE_KEY is not a valid private key");

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    let client = Arc::new(client);

    let uniswap_query = Arc::new(UniswapQuery::new(
        network.uniswap_query_address,
        client.clone(),
    ));
    let bundle_executor = Arc::new(BundleExecutor::new(config.executor_address, client.clone()));

    let tokens: Vec<Token> =
        deserialize_token_file(format!("src/networks/{}/_tokens.json", network.name));
    let ref_tokens: Vec<Arc<Token>> = tokens.into_iter().map(|f: Token| Arc::new(f)).collect();

    return Cache {
        client,
        uniswap_query,
        bundle_executor,
        tokens: ref_tokens,
    };
}
