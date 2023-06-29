use ethers::providers::Middleware;

pub mod env;
pub mod exchanges;
pub mod networks;
pub mod utils;

#[tokio::main]
async fn main() {
    utils::logger::clear_console();
    
    let f: env::Environment = env::init_environment().await;
    let r: ethers::types::U64 = f.provider.get_block_number().await.unwrap();

    print!("{}", r)
}
