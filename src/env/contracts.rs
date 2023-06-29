use std::sync::Arc;

use ethers::prelude::*;

abigen!(UniswapQuery, "src/contracts/abi/UniswapQuery.json");
abigen!(BundleExecutor, "src/contracts/abi/BundleExecutor.json");

pub fn get_uniswap_query(provider: Provider<Http>, address: Address) ->  UniswapQuery<Provider<Http>>{
    let client: Arc<Provider<Http>> = Arc::new(provider);
    return UniswapQuery::new(address, client);
}

pub fn get_bundle_executor(provider: Provider<Http>, address: Address) ->  BundleExecutor<Provider<Http>>{
    let client: Arc<Provider<Http>> = Arc::new(provider);
    return BundleExecutor::new(address, client);
}