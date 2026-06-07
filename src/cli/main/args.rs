use std::path::PathBuf;

use clap::Args;

use crate::{PackageManager, ScriptRunner};

#[derive(Clone, Debug, Args)]
pub struct MainArgs {
    /// Path to the configuration file
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// The package manager to use
    #[arg(long, value_name = "PACKAGE_MANAGER")]
    pub pm: Option<PackageManager>,

    /// The script runner to use
    #[arg(long, value_name = "SCRIPT_RUNNER")]
    pub runner: Option<ScriptRunner>,

    /// Print the command without executing it
    #[arg(long)]
    pub dry_run: bool,
}
