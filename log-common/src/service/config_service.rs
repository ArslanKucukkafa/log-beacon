use crate::models::config_model::Config;
use std::fs;

// Config dosyasını okur ve Config struct'ını döner
pub fn load_config() -> Result<Config, String> {
    let content = fs::read_to_string("../../../app-config.toml")
        .map_err(|e| format!("Config file read error: {}", e))?;

    toml::from_str(&content)
        .map_err(|e| format!("TOML parse error: {}", e))
}

// Config struct degiskenlerini toml dosyasına yazdırır.
pub fn save_config(config: Config) -> Result<(), String> {
    let toml = toml::to_string(&config)
        .map_err(|e| format!("TOML serialization failed: {}", e))?;

    fs::write("../../../app-config.toml", toml)
        .map_err(|e| format!("File write failed: {}", e))?;

    Ok(())
}