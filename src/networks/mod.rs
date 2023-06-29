use self::types::{ Network};

mod arbitrum;
pub mod types;

pub fn get_network(chain_id: i32) -> Network {
    println!("getting network");

    // Arbitrum One
    if chain_id == arbitrum::get_chain_id() {
        return arbitrum::get_instance();
    }

    panic!("d");
}
