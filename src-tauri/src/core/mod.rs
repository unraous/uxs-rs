pub mod url;
pub mod script;
pub mod config;

pub use url::{Type, classify};
pub use script::{obtain_through, inject};
pub use config::Config;