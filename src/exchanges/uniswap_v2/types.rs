use ethers::prelude::{k256::ecdsa::SigningKey, *};

abigen!(UniswapV2Factory, "src/exchanges/uniswap_v2/_factory.json");
abigen!(UniswapV2Pair, "src/exchanges/uniswap_v2/_pair.json");

pub type UniswapV2FactoryContract =
    UniswapV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub type UniswapV2PairContract =
    UniswapV2Pair<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
