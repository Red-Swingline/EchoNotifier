use serde::{Deserialize,Serialize};
use std::fs;
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize)] 
pub struct AppConfig {
    pub apps: Vec<AppSoundConfig>,
    pub app_settings: AppSettings,
}

#[derive(Serialize, Deserialize)] 
pub struct AppSoundConfig {
    pub app: String,
    pub sound_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub debounce_period: u64,
}

pub fn read_config(config_path: &str) -> Result<AppConfig, Box<dyn Error>> {
    let config_data = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_json::from_str(&config_data)?;
    Ok(config)
}

pub fn write_config<P: AsRef<Path>>(config: &AppConfig, config_path: P) -> Result<(), Box<dyn Error>> {
    let config_data = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_data)?;
    Ok(())
}

pub fn update_app_sound_path(config: &mut AppConfig, app_name: &str, new_sound_path: &str) -> Result<(), String> {
    if let Some(app) = config.apps.iter_mut().find(|a| a.app == app_name) {
        app.sound_path = new_sound_path.to_owned();
        Ok(())
    } else {
        Err("App not found".to_string())
    }
}
pub fn delete_app(config: &mut AppConfig, app_name: &str) -> Result<(), String> {
    if let Some(index) = config.apps.iter().position(|a| a.app == app_name) {
        config.apps.remove(index);
        Ok(())
    } else {
        Err("App not found".to_string())
    }
}
