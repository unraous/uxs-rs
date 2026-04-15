pub mod url;
pub mod script;

pub use url::{Type, classify};
pub use script::{obtain_through, inject};