mod env;
pub mod utils;
pub mod networks;

fn main() {
    utils::logger::clear_console();

    env::init_environment();
}
