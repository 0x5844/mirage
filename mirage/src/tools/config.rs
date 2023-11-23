use lazy_static::lazy_static;

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, sync::Mutex};

use super::{Tool, Tools};

lazy_static! {
    static ref CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::default());
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum EncryptionLevel {
    Level1,
    Level2,
}

impl std::fmt::Display for EncryptionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EncryptionLevel::Level1 => write!(f, "Level 1"),
            EncryptionLevel::Level2 => write!(f, "Level 2"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Features {
    pub parallel_processing: bool,
    pub reconnaissance: bool,
    pub stealth: bool,
    pub enhanced_seed_generation: bool,
    pub enhanced_heap_modification: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Parameters {
    pub encryption_level: EncryptionLevel,
    pub observer_temperature: f32,
    pub memory_scramble_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub features: Features,
    pub parameters: Parameters,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            features: Features {
                parallel_processing: false,
                reconnaissance: false,
                stealth: false,
                enhanced_seed_generation: false,
                enhanced_heap_modification: false,
            },
            parameters: Parameters {
                encryption_level: EncryptionLevel::Level1,
                observer_temperature: 5.0,
                memory_scramble_size: 10,
            },
        }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json =
            serde_json::to_string_pretty(self).unwrap_or_else(|_| "Invalid JSON".to_string());
        write!(f, "{}", json)
    }
}

pub struct Config;

impl Tool for Config {
    fn name(&self) -> &Tools {
        &Tools::Config
    }

    fn start(&mut self) {
        let config_path = "mirage/config.json";
        if !Path::new(config_path).exists() {
            info!(
                "[Tools/{}] Config file not found at '{}', creating it...",
                Tools::Config,
                config_path
            );
            Self::create_config(config_path);
        }

        let config_contents = match fs::read_to_string(config_path) {
            Ok(contents) => contents,
            Err(e) => {
                error!("[Tools/{}] Failed to read config: {}", Tools::Config, e);
                Self::create_config(config_path);
                return;
            }
        };

        let config = match serde_json::from_str::<AppConfig>(&config_contents) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("[Tools/{}] Failed to parse config: {}", Tools::Config, e);
                Self::create_config(config_path);
                return;
            }
        };

        let mut cfg = CONFIG.lock().unwrap();
        *cfg = config;

        info!("[Tools/{}] Configuration loaded: {}", Tools::Config, *cfg);
    }
}

impl Config {
    pub fn new() -> Self {
        if cfg!(feature = "development") {
            debug!("[Tools/{}] Initializing...", Tools::Config);
        }
        Config
    }

    fn create_config(config_path: &str) {
        let default_config = AppConfig::default();
        match serde_json::to_string(&default_config) {
            Ok(config_str) => {
                if let Err(e) = fs::write(config_path, config_str) {
                    error!(
                        "[Tools/{}] Failed to create default config file: {}",
                        Tools::Config,
                        e
                    );
                } else {
                    info!("[Tools/{}] Default config file created", Tools::Config);
                }
            }
            Err(e) => error!(
                "[Tools/{}] Failed to serialize default config: {}",
                Tools::Config,
                e
            ),
        }
    }

    pub fn get_features() -> Features {
        CONFIG.lock().unwrap().features.clone()
    }

    pub fn get_parameters() -> Parameters {
        CONFIG.lock().unwrap().parameters.clone()
    }
}
