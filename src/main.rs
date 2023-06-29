mod env;
pub mod utils;
pub mod networks;
pub mod exchanges;

fn main() {
    utils::logger::clear_console();

    env::init_environment();
}
