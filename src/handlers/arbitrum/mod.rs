use ethers::abi::Address;

mod data_feed;
mod decoder;
mod types;

const RAW_FROM_ADDRESS: &str = "0xf977814e90da44bfa03b6295a0616a897441acec";

lazy_static! {
    static ref FROM_ADDRESS: Address = RAW_FROM_ADDRESS.to_string().parse().unwrap();
}

pub async fn init() {
    _ = data_feed::init().await
}
