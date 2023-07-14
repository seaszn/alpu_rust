use ethers::types::*;
use url::Url;

pub fn url(input: String) -> Url {
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

pub fn dec_to_u256(dec: &str, unit: u32) -> U256 {
    let dec_secondary_count: u32 = dec.split('.').collect::<Vec<&str>>()[1].len() as u32;

    if dec_secondary_count > unit {
        panic!("decimals overflow unit type");
    }

    let mut tot = dec.replace(".", "").to_owned();
    for _ in 0..unit - dec_secondary_count {
        tot.push_str("0");
    }

    return U256::from_dec_str(&tot).unwrap();
}

pub fn dec_to_u128(dec: &str, unit: u32) -> u128 {
    if let Some(decimal_index) = dec.chars().position(|x| x == '.') {
        let decimal_count = (dec.len() - decimal_index) as u32;
        
        if decimal_count > unit {
            panic!("decimals overflow unit type");
        }

        return dec.replace(".", "").parse::<u128>().unwrap() * 10u128.pow(unit - decimal_count);
    }
    else {
        panic!("wrong input");
    }

}
