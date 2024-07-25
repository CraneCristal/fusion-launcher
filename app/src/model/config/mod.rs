pub mod persistence;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    language: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            language: "en".to_string(),
        }
    }
}
