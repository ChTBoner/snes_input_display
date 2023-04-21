pub mod config {
    use config::Config;
    use std::path::PathBuf;
    use serde::Deserialize;
    use dirs;

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
        pub skin: SkinConfig
    }

    impl AppConfig {
        pub fn new() -> Self {
            let config_file_path = dirs::config_local_dir().unwrap().join("snes_input_viewer").join("settings");
            let config_file_path = config_file_path.to_str().unwrap();
            let s = Config::builder()
                .add_source(config::File::with_name(&config_file_path))

                .build()
                .unwrap();
            
            match s.try_deserialize() {
                Ok(settings) => return settings,
                Err(_) => panic!(),
            }
        }
    }

}