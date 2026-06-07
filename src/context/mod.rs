use crate::cli::MainArgs;
use crate::context::config::Config;
use crate::{PackageManager, Result, ScriptRunner};

mod config;

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub cli_pm: Option<PackageManager>,
    pub cli_runner: Option<ScriptRunner>,

    pub config_local_pm: Option<PackageManager>,
    pub config_global_pm: Option<PackageManager>,
    pub config_runner: Option<ScriptRunner>,

    pub interactive: bool,
    pub global: bool,
    pub dry_run: bool,

    pub registry: String,
}

impl Context {
    pub fn new(args: &MainArgs) -> Result<Self> {
        let mut ctx = Self {
            cli_pm: args.pm,
            cli_runner: args.runner,

            interactive: true,
            dry_run: args.dry_run,

            registry: crate::constants::NPM_REGISTRY_URL.into(),

            ..Default::default()
        };

        let config = Config::new(args.config.as_deref())?;
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
