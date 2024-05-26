use std::{fs::File, io::Read};

use crate::debug;

const APP_CONFIG_FILE_NAME: &str = "ollama_assistant.json";

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone,
)]
pub struct ApplicationSettings {
    pub ai_model: String,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self { ai_model: "llama3:latest".to_string() }
    }
}

/// Getting config's location
fn get_config_path() -> String {
    use std::env::var;

    let config_home =
        var("XDG_CONFIG_HOME").or_else(|_| {
            var("HOME")
                .map(|home| format!("{}/.config", home))
        });

    format!(
        "{}/{}",
        config_home.unwrap(),
        APP_CONFIG_FILE_NAME
    )
}

/// Getting current user's settings
pub fn load_settings() -> ApplicationSettings {
    let mut file = match File::open(get_config_path()) {
        Ok(file) => file,
        Err(_) => return ApplicationSettings::default(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    debug!(&contents);

    let settings: ApplicationSettings =
        serde_json::from_str(&contents).unwrap_or_default();

    debug!(&settings);

    settings
}

/// Store new user settings
pub fn save_settings(settings: ApplicationSettings) {
    let settings =
        serde_json::to_string(&settings).unwrap();
    std::fs::write(get_config_path(), settings).unwrap();
}
