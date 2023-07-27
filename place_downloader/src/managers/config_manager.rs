use serde::{Deserialize, Serialize};
use std::fs::{File, self};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use crate::models::config_model::AppConfig;

pub struct ConfigManager;

impl ConfigManager {
    pub fn load_config() -> Result<AppConfig, Error> {
        let config_dir = get_config_dir();
        let _ = fs::create_dir_all(config_dir.clone());

        let config_path = Path::new(&config_dir).join("config.json");

        if config_path.exists() {
            let mut file = File::open(config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            let config: AppConfig = serde_json::from_str(&contents)?;

            return Ok(config);
        }
        
        let config = AppConfig::new_empty();
        save_config(&config)?;
        Ok(config)
    }
}

fn save_config(config: &AppConfig) -> Result<(), Error> {
    let config_dir = get_config_dir();
    let _ = fs::create_dir_all(config_dir.clone());

    let config_path = Path::new(&config_dir).join("config.json");
    let serialized_config = serde_json::to_string_pretty(config)?;
    
    let mut file = File::create(config_path)?;
    file.write_all(serialized_config.as_bytes())?;

    Ok(())
}

fn get_config_dir() -> String {
    let config_dir_option = dirs::config_dir();

    let config_dir = match config_dir_option {
        Some(config_dir) => config_dir.display().to_string(),
        None => PathBuf::from("/").display().to_string()
    };

    Path::new(&config_dir).join("rplaceDownloader").display().to_string()
}