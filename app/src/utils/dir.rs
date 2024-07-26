use std::env;
use std::path::PathBuf;
use directories::ProjectDirs;

pub fn get_app_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "", "fusion-launcher").unwrap()
}

pub fn get_assets_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("app/assets")
    } else {
        get_assets_dir().join("assets")
    }
}