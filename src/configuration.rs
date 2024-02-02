use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{write, File};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct ControllerConfig {
    pub input_config_path: PathBuf,
    pub layout: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_theme: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    pub controller: ControllerConfig,
    pub skin: SkinConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file_path = dirs::config_local_dir()
            .unwrap()
            .join("snes-input-display")
            .join("settings.toml");
        let config_file_path = config_file_path.to_str().unwrap();
        dbg!(config_file_path);
        if !Path::new(&config_file_path).exists() {
            Self::create_default(config_file_path).unwrap();
        }
        let s = Config::builder()
            .add_source(config::File::with_name(config_file_path))
            .build()?;

        s.try_deserialize()
    }

    fn create_default(path: &str) -> Result<(), Box<dyn Error>> {
        println!("Creating a new settings file: {path}");
        let config = AppConfig {
            controller: ControllerConfig {
                input_config_path: PathBuf::from(&path),
                layout: "Default".to_string(),
            },
            skin: SkinConfig {
                skins_path: PathBuf::from(
                    "C:\\Users\\example\\Documents\\retrospy-nintendospy-skins\\skins",
                ),
                skin_name: "skin_folder_name".to_string(),
                skin_theme: "skin_theme".to_string(),
            },
        };
        let toml = toml::to_string(&config)?;
        File::create(path)?;
        write(path, toml)?;
        Ok(())
    }
}
