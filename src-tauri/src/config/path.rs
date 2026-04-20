
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::io::Error;
use log::error;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathsConfig {
    pub cwd: PathBuf,

    pub logs: PathBuf,
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|e| {
        error!("Failed to get current working directory: {}", e);
        PathBuf::from(".")
    })
}

fn cwd_parent() -> PathBuf {
    cwd().parent()
        .map(Path::to_path_buf) 
        .ok_or(Error::new(
            std::io::ErrorKind::Other,
            "No parent directory"
        ))
        .unwrap_or_else(|e| {
            error!("Failed to get parent directory: {}", e);
            PathBuf::from("..")
        })
}

fn data_dir() -> PathBuf {
    match cfg!(debug_assertions) {
        true => cwd_parent().join("uxs-data"),
        false => cwd().join("uxs-data"),
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            cwd: cwd(),
            logs: data_dir().join("logs"),
        }
    }
}


