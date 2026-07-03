pub mod config;
pub mod course;
pub mod window;


#[macro_export]
macro_rules! register {
    () => {{
        use $crate::commands::{window, config};
        
        tauri::generate_handler![
            window::close,
            window::minimize,
            
            config::metadata,
            config::switch_provider,
            config::models,
            config::switch_model,
            config::set_key,
        ]
    }};
}