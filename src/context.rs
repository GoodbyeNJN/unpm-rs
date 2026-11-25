use std::path::PathBuf;

use anyhow::Result;

use crate::{Config, NPM_REGISTRY_URL, PackageManager, ScriptRunner};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub cli_pm: Option<PackageManager>,
    pub cli_runner: Option<ScriptRunner>,

    pub config_local_pm: Option<PackageManager>,
    pub config_global_pm: Option<PackageManager>,
    pub config_runner: Option<ScriptRunner>,

    pub interactive: bool,
    pub dry_run: bool,

    pub registry: String,
}

impl AppContext {
    pub fn new(
        config: Option<PathBuf>,
        pm: Option<PackageManager>,
        runner: Option<ScriptRunner>,
        dry_run: bool,
    ) -> Result<Self> {
        let mut ctx = Self {
            cli_pm: pm,
            cli_runner: runner,

            config_local_pm: None,
            config_global_pm: None,
            config_runner: None,

            interactive: true,
            dry_run,

            registry: NPM_REGISTRY_URL.into(),
        };

        let config = Config::new(config.as_deref())?;
        if let Some(config) = config.as_ref() {
            if let Some(pm) = config.pm.as_ref() {
                ctx.config_local_pm = pm.local;
                ctx.config_global_pm = pm.global;
            }

            ctx.config_runner = config.runner;

            if let Some(registry) = config.registry.as_ref() {
                ctx.registry = registry.to_owned();
            }
        }

        Ok(ctx)
    }
}
