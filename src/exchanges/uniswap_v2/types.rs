use ethers::prelude::{k256::ecdsa::SigningKey, *};

abigen!(UniswapV2Factory, "src/exchanges/uniswap_v2/_factory.json");

pub type UniswapV2FactoryContract =
    UniswapV2Factory<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;
