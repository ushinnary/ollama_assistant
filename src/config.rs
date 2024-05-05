use std::{fs::OpenOptions, io::Read};

use crate::debug;

const APP_CONFIG_FILE_NAME: &str = "settings.json";

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone,
)]
pub struct ApplicationSettings {
    pub ai_model: String,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self { ai_model: "qwen:0.5b".to_string() }
    }
}

pub fn load_settings() -> ApplicationSettings {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(APP_CONFIG_FILE_NAME)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let settings: ApplicationSettings =
        serde_json::from_str(&contents).unwrap_or_default();

    debug!(&settings);

    settings
}

pub fn save_settings(settings: ApplicationSettings) {
    let settings =
        serde_json::to_string(&settings).unwrap();
    std::fs::write(APP_CONFIG_FILE_NAME, settings).unwrap();
}
