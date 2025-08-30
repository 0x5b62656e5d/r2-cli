use serde::Deserialize;
use std::{env, fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default: Keys,
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub key_id: String,
    pub secret_key: String,
    pub endpoint_url: String,
    pub region: String,
}

pub fn get_config() -> Result<Config, anyhow::Error> {
    Ok(toml::de::from_str(&fs::read_to_string(
        get_config_dir().join("config.toml"),
    )?)?)
}

pub fn get_config_dir() -> PathBuf {
    env::home_dir().unwrap().join(".config/r2-cli")
}
