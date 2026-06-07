use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{BoxedError, ConfigError, ConfigErrorFrom, PackageManager, Result, ScriptRunner};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    pub local: Option<PackageManager>,
    pub global: Option<PackageManager>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub pm: Option<PackageManagerConfig>,

    pub runner: Option<ScriptRunner>,

    pub registry: Option<String>,
}

impl Config {
    pub fn new(arg: Option<&Path>) -> Result<Option<Config>> {
        debug!("config resolve start...");

        // First priority: config file specified via command-line argument
        if let Some(path) = arg {
            return Self::from_arg_path(path);
        }

        // Second priority: config file specified via environment variable
        if let Some(config) = Self::from_env_path()? {
            return Ok(Some(config));
        }

        // Third priority: default config file in platform-specific config directory
        if let Some(config) = Self::from_default_path()? {
            return Ok(Some(config));
        }

        Ok(None)
    }

    fn from_arg_path(path: &Path) -> Result<Option<Self>> {
        debug!("from arg path start...");

        if Self::check(path) {
            let config = Self::load(path).map_err(|source| ConfigError::Load {
                from: ConfigErrorFrom::Arg(path.to_owned()),
                source,
            })?;

            debug!("from arg path end, path: {:?}", path);
            Ok(config)
        } else {
            debug!("from arg path end, not found, path: {:?}", path);
            Err(ConfigError::NotExists {
                from: ConfigErrorFrom::Arg(path.to_owned()),
            })?
        }
    }

    fn from_env_path() -> Result<Option<Self>> {
        debug!("from env path start...");

        let env = std::env::var(crate::constants::ENV_CONFIG_FILE).unwrap_or_default();
        let env = env.trim();
        if env.is_empty() {
            debug!("from env path end, empty env var");
            return Ok(None);
        }

        let path = &PathBuf::from(env);
        debug!("env: {}, path: {:?}", env, path);
        if Self::check(path) {
            let config = Self::load(path).map_err(|source| ConfigError::Load {
                from: ConfigErrorFrom::Env(path.to_owned()),
                source,
            })?;

            debug!("from env path end, path: {:?}", path);
            Ok(config)
        } else {
            debug!("from env path end, not found, path: {:?}", path);
            Err(ConfigError::NotExists {
                from: ConfigErrorFrom::Env(path.to_owned()),
            })?
        }
    }

    fn from_default_path() -> Result<Option<Self>> {
        debug!("from default path start...");

        let path = directories::ProjectDirs::from("org", "unpm", "unpm")
            .map(|dir| dir.config_dir().to_owned().join("config.json"));

        if let Some(path) = &path
            && Self::check(path)
        {
            let config = Self::load(path).map_err(|source| ConfigError::Load {
                from: ConfigErrorFrom::Default(path.to_owned()),
                source,
            })?;

            debug!("from default path end, path: {:?}", path);
            Ok(config)
        } else {
            debug!("from default path end, not found, path: {:?}", path);
            Ok(None)
        }
    }

    fn check(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    fn load(path: &Path) -> Result<Option<Self>, BoxedError> {
        debug!("config loading start...");

        let content = fs::read_to_string(path)?;
        debug!("content: {}", &content[..content.len().min(20)]);

        let config = if content.trim().is_empty() {
            None
        } else {
            serde_json::from_str(content.as_str())?
        };

        debug!("config loading end, config: {:?}", config);
        Ok(config)
    }

    #[allow(dead_code)]
    fn save(&self, path: &Path) -> Result<(), BoxedError> {
        debug!("config saving start...");

        let content = serde_json::to_string_pretty(self)?;
        debug!("content: {}", &content[..content.len().min(20)]);

        fs::write(path, content)?;

        debug!("config saving end, path: {:?}", path);
        Ok(())
    }
}
