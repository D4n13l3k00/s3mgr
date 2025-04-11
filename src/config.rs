use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3Config {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub bucket: String,
    pub endpoint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub s3: S3Config,
    pub upload_chunk_size: usize,
    pub download_chunk_size: usize,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let config_str = std::fs::read_to_string(&config_path).context(format!(
            "Failed to read config file at {}",
            config_path.display()
        ))?;

        let loaded_config: Config = toml::from_str(&config_str).context(format!(
            "Failed to parse config file at {}",
            config_path.display()
        ))?;

        Ok(loaded_config)
    }

    pub fn reset() -> Result<()> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            std::fs::remove_file(&config_path).context(format!(
                "Failed to remove config file at {}",
                config_path.display()
            ))?;
        }

        let default_config = Self::default();
        default_config.save()?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home.join(".config").join("s3mgr");
        std::fs::create_dir_all(&config_dir).context(format!(
            "Failed to create config directory at {}",
            config_dir.display()
        ))?;
        Ok(config_dir.join("config.toml"))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let config_str =
            toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;
        std::fs::write(&config_path, config_str).context(format!(
            "Failed to write config file at {}",
            config_path.display()
        ))?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            s3: S3Config {
                access_key: String::new(),
                secret_key: String::new(),
                region: "us-east-1".to_string(),
                bucket: String::new(),
                endpoint: None,
            },
            upload_chunk_size: 2 * 1024 * 1024,
            download_chunk_size: 2 * 1024 * 1024,
        }
    }
}
