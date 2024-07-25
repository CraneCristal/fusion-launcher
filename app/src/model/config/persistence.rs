use crate::error::Error;
use crate::model::config::AppConfig;
use crate::utils::get_app_dirs;
use crate::utils::json::{deserialize_from_file, serialize_to_file};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "config.json";

fn get_config_file_path() -> PathBuf {
    let app_data_dir = get_app_dirs().data_dir().to_path_buf();
    let config_file_path = app_data_dir.join(CONFIG_FILENAME);
    if !config_file_path.exists() {
        fs::create_dir_all(app_data_dir).expect("Failed to create data directory");
        File::create(&config_file_path).expect("Failed to create games.json file");
    }

    config_file_path
}

pub fn get_app_config() -> Result<AppConfig, Error> {
    let config_file_path = get_config_file_path();
    match deserialize_from_file(config_file_path) {
        Ok(config) => Ok(config),
        Err(error) => match error {
            Error::Json(_) => {
                let default_config = AppConfig::new();
                save_app_config(&default_config)?;
                Ok(default_config)
            }
            _ => Err(error),
        },
    }
}

pub fn save_app_config(config: &AppConfig) -> Result<(), Error> {
    let config_file_path = get_config_file_path();
    serialize_to_file(&config, &config_file_path)
}
