use std::fs;

pub fn get_base_price_table() -> String {
    return fs::read_to_string("src/price_oracle/response.json").expect("Failed to read token file");
}
