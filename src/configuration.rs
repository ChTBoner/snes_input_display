use config::{Config, ConfigError};

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ControllerConfig {
    pub input_config_path: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_theme: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub controller: ControllerConfig,
    pub skin: SkinConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file_path = dirs::config_local_dir()
            .unwrap()
            .join("snes-input-display")
            .join("settings");
        let config_file_path = config_file_path.to_str().unwrap();
        dbg!(&config_file_path);
        let s = Config::builder()
            .add_source(config::File::with_name(config_file_path))
            .build()?;

        s.try_deserialize()
    }
}
