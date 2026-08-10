use rusb2snes::USB2SnesEndpoint;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, write, File};
use std::path::{Path, PathBuf};

use crate::controller::controller_impl::ControllerConfig;
use crate::input_viewer::APP_NAME;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppConfig {
    pub controller: ControllerConfig,
    pub skin: SkinConfig,
    pub usb2snes: Option<USB2SnesEndpoint>,
    pub log_path: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SkinConfig {
    pub skins_path: PathBuf,
    pub skin_name: String,
    pub skin_background: Option<String>,
}

impl AppConfig {
    pub fn new(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        // compute config_file_path
        let config_dir_path = match dirs::config_local_dir() {
            Some(c) => c,
            None => return Err("Can't figure out configuration directory".into()),
        };

        let config_file_path = match path {
            Some(p) => PathBuf::from(p),
            None => config_dir_path.join(APP_NAME).join("settings.toml"),
        };

        let config_file_path = match config_file_path.to_str() {
            Some(s) => s,
            None => return Err("Cannot compute the configuration file path".into()),
        };

        // check if path exists or create default settings file
        if !Path::new(&config_file_path).exists() {
            Self::create_default(config_file_path)?;
        }

        // read and load config
        let contents = read_to_string(config_file_path)?;
        let config: AppConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn get_log_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        if let Some(ref path) = self.log_path {
            Ok(path.clone())
        } else {
            let config_dir_path = dirs::config_local_dir()
                .ok_or("Can't figure out configuration directory")?;
            Ok(config_dir_path.join(APP_NAME).join("SnITCH.log"))
        }
    }

    pub fn init_logging(&self) -> Result<(), Box<dyn Error>> {
        let log_path = self.get_log_path()?;
        if let Some(parent) = log_path.parent() {
            create_dir_all(parent)?;
        }

        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Info)
            .chain(std::io::stdout())
            .chain(fern::log_file(&log_path)?)
            .apply()?;

        log::info!("Logging initialized. Log file: {}", log_path.display());
        Ok(())
    }

    fn create_default(path: &str) -> Result<(), Box<dyn Error>> {
        log::info!("Creating a new settings file: {path}");
        let documents_dir = match dirs::document_dir() {
            Some(p) => p,
            None => return Err("Could not compute Documents directory".into()),
        };
        let default_dir = documents_dir.join("snes-input-display");
        let default_inputs_file_path = default_dir.join("inputs_addresses.json");
        let default_skins_dir_path = default_dir.join("skins");

        let default_config_dir = dirs::config_local_dir()
            .ok_or("Can't figure out configuration directory")?;
        let default_log_path = default_config_dir.join(APP_NAME).join("SnITCH.log");

        let config = AppConfig {
            controller: ControllerConfig {
                input_config_path: default_inputs_file_path,
                layout: "Default".to_string(),
            },
            skin: SkinConfig {
                skins_path: default_skins_dir_path,
                skin_name: "skin_folder_name".to_string(),
                skin_background: Some("skin_theme".to_string()),
            },
            usb2snes: Some(USB2SnesEndpoint::default()),
            log_path: Some(default_log_path),
        };
        let toml = toml::to_string(&config)?;
        File::create(path)?;
        write(path, toml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_log_path() {
        let default_config_dir = dirs::config_local_dir().unwrap();
        let expected = default_config_dir.join(APP_NAME).join("SnITCH.log");

        let config = AppConfig {
            controller: ControllerConfig {
                input_config_path: PathBuf::from("test"),
                layout: "Default".to_string(),
            },
            skin: SkinConfig {
                skins_path: PathBuf::from("test"),
                skin_name: "test".to_string(),
                skin_background: None,
            },
            usb2snes: None,
            log_path: None,
        };

        let log_path = config.get_log_path().unwrap();
        assert_eq!(log_path, expected);
    }
}

