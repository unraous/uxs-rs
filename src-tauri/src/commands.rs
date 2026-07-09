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
            config::providers,
            config::current_provider,
            config::switch_provider,
            config::models,
            config::current_model,
            config::switch_model,
            config::api_key,
            config::set_key,
        ]
    }};
}