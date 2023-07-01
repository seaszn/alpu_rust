use ethers::types::Address;
use url::Url;

pub fn url(input: String) -> Url{
    let parse_result: Result<Url, _> = Url::parse(input.as_str());
    if parse_result.is_err() {
        panic!("not a valid url: {}", input)
    }

    return parse_result.unwrap();
}

pub fn u32(input: String) -> u32 {
    let parse_result: Result<u32, _> = input.parse();
    if parse_result.is_err() {
        panic!("not a valid u32: {}", input)
    }

    return parse_result.unwrap();
}

pub fn f32(input: String) -> f32 {
    let parse_result: Result<f32, _> = input.parse();
    if parse_result.is_err() {
        panic!("not a valid f32: {}", input)
    }

    return parse_result.unwrap();
}

pub fn address(input: String) -> Address {
    let parse_result: Result<Address, _> = input.parse();

    if parse_result.is_err() {
        panic!("not a valid address: {}", input)
    }

    return parse_result.unwrap();
}
