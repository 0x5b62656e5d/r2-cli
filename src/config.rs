use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default: Keys,
}

#[derive(Debug, Deserialize)]
pub struct Keys {
    pub key_id: String,
    pub secret_key: String,
    pub endpoint_url: Option<String>,
    pub is_cloudflare_r2: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Regions {
    #[serde(default)]
    pub buckets: HashMap<String, String>,
}

pub fn get_config() -> Result<Config, anyhow::Error> {
    Ok(toml::de::from_str(&fs::read_to_string(
        get_config_dir().join("config.toml"),
    )?)?)
}

pub fn get_regions() -> Result<Regions, anyhow::Error> {
    Ok(toml::de::from_str(&fs::read_to_string(
        get_config_dir().join("regions.toml"),
    )?)?)
}

pub fn get_config_dir() -> PathBuf {
    env::home_dir().unwrap().join(".config/s3-cli")
}

pub fn init_config() -> Result<(), anyhow::Error> {
    let config_dir: PathBuf = get_config_dir();

    if !config_dir.is_dir() {
        fs::create_dir_all(&config_dir)?;
    }

    if let Ok(res) = fs::exists(config_dir.join("config.toml")) {
        if !res {
            fs::write(
                config_dir.join("config.toml"),
                r#"[default]
key_id = ""
secret_key = ""
endpoint_url = ""
is_cloudflare_r2 = false
"#,
            )?;
        }
    }

    if let Ok(res) = fs::exists(config_dir.join("regions.toml")) {
        if !res {
            fs::write(config_dir.join("regions.toml"), "[buckets]\n")?;
        }
    }

    Ok(())
}
