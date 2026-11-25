use std::env;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use color_print::cformat;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{ENV_CONFIG_FILE, PackageManager, ScriptRunner};

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
            let config = Self::load(path)
                .with_context(|| format!("failed to load config file: {}", PathFrom::Arg(path)))?;

            debug!("from arg path end, path: {:?}", path);
            Ok(config)
        } else {
            debug!("from arg path end, not found, path: {:?}", path);
            bail!("config file does not exiist: {}", PathFrom::Arg(path));
        }
    }

    fn from_env_path() -> Result<Option<Self>> {
        debug!("from env path start...");

        let env = env::var(ENV_CONFIG_FILE).unwrap_or_default();
        let env = env.trim();
        if env.is_empty() {
            debug!("from env path end, empty env var");
            return Ok(None);
        }

        let path = &PathBuf::from(env);
        debug!("env: {}, path: {:?}", env, path);
        if Self::check(path) {
            let config = Self::load(path)
                .with_context(|| format!("failed to load config file: {}", PathFrom::Env(path)))?;

            debug!("from env path end, path: {:?}", path);
            Ok(config)
        } else {
            debug!("from env path end, not found, path: {:?}", path);
            bail!("config file does not exist: {}", PathFrom::Env(path));
        }
    }

    fn from_default_path() -> Result<Option<Self>> {
        debug!("from default path start...");

        let path = directories::ProjectDirs::from("org", "unpm", "unpm")
            .map(|dir| dir.config_dir().to_owned().join("config.json"));

        if let Some(path) = &path
            && Self::check(path)
        {
            let config = Self::load(path).with_context(|| {
                format!("failed to load config file: {}", PathFrom::Default(path))
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

    fn load(path: &Path) -> Result<Option<Self>> {
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
    fn save(&self, path: &Path) -> Result<()> {
        debug!("config saving start...");

        let content = serde_json::to_string_pretty(self)?;
        debug!("content: {}", &content[..content.len().min(20)]);

        fs::write(path, content)?;

        debug!("config saving end, path: {:?}", path);
        Ok(())
    }
}

enum PathFrom<'a> {
    Arg(&'a Path),
    Env(&'a Path),
    Default(&'a Path),
}

impl Display for PathFrom<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = match self {
            PathFrom::Arg(path) | PathFrom::Env(path) | PathFrom::Default(path) => {
                cformat!("<yellow>{}</>", path.display())
            }
        };
        let from = match self {
            PathFrom::Arg(..) => cformat!("from argument '<bold>--config <<PATH>></>'",),
            PathFrom::Env(..) => {
                cformat!("from environment variable '<bold>{}</>'", ENV_CONFIG_FILE)
            }
            PathFrom::Default(..) => cformat!("from default path"),
        };

        write!(f, "'{path}' ({from})")
    }
}
