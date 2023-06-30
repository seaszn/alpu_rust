#[doc(hidden)]
// #[macro_export]
// macro_rules! config {super(use config) }

#[doc(inline)]
pub use config::Config as Config;
pub use config::get_runtime_config as get_runtime_config;

pub use cache::Cache as Cache;
pub use cache::get_runtime_cache as get_runtime_cache;

mod cache;
mod config;