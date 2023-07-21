use ethers::{
    abi::{Address, Token, Tokenize},
    types::{Bytes, U256},
};

pub struct BundleExecutionCall {
    pub token: Address,
    pub amount_to_first_market: U256,
    pub targets: Vec<Address>,
    pub payloads: Vec<Bytes>,
}

impl BundleExecutionCall {

    #[inline(always)]
    fn get_target_tokens(&self) -> Vec<Token>{
        return self.targets.iter().map(|x| Token::Address(*x)).collect();
    }
    
    #[inline(always)]
    fn get_payload_tokens(&self) -> Vec<Token>{
        return self.payloads.iter().map(|x| Token::Bytes(x.to_vec())).collect();
    }
    
}

impl Tokenize for BundleExecutionCall {
    #[inline(always)]
    fn into_tokens(self) -> Vec<ethers::abi::Token> {
        return Vec::from([
            Token::Address(self.token),
            Token::Uint(self.amount_to_first_market),
            Token::Array(self.get_target_tokens()),
            Token::Array(self.get_payload_tokens()),
        ]);
    }

}
