pub mod config;
pub mod course;
pub mod window;

#[macro_export]
macro_rules! register {
    () => {{
        auto_handler::generate_auto_handler!()
    }};
}
