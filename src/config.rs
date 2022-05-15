use serde::{Serialize, Deserialize};

use crate::APP_NAME;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub refresh_secs: u64,
    pub url: String,
    pub interval_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            refresh_secs: 30,
            url: String::from("https://newsapi.org/v2/everything"),
            interval_secs: 900,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        confy::load::<AppConfig>(APP_NAME).unwrap()
    }
}
