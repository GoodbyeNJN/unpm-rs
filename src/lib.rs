mod cli;
pub(crate) mod config;
mod context;
pub(crate) mod fs;
mod package_manager;
pub(crate) mod process;
mod project;
pub(crate) mod registry;
pub(crate) mod terminal;

pub use cli::{Unpm, Unpx, unpm, unpx};
pub(crate) use config::Config;
pub use context::AppContext;
pub(crate) use package_manager::{
    Adapter, PackageManager, PackageManagerOperation, ScriptRunner, ScriptRunnerOperation,
};
pub(crate) use project::Project;

pub(crate) const ENV_CONFIG_FILE: &str = "UNPM_CONFIG_FILE";
pub(crate) const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
pub(crate) const PACKAGE_JSON_NAME: &str = "package.json";
pub(crate) const NODE_MODULES_NAME: &str = "node_modules";
pub(crate) const NODE_MODULES_BIN_NAME: &str = ".bin";
