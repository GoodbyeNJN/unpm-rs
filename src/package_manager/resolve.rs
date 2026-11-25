use std::io::IsTerminal;

use anyhow::{Context, Result, bail};
use color_print::cstr;

use crate::terminal;

use super::{PackageManager, ScriptRunner, detect};

pub fn resolve_local_package_manager(
    cli: Option<PackageManager>,
    config: Option<PackageManager>,
    interactive: bool,
) -> Result<PackageManager> {
    // Priority: cli > detect > config
    let pm = cli
        .or_else(|| detect::detect_package_manager(None, None))
        .or(config);
    if let Some(package_manager) = pm {
        return Ok(package_manager);
    }

    interactive_select(interactive)
}

pub fn resolve_global_package_manager(
    cli: Option<PackageManager>,
    config: Option<PackageManager>,
    interactive: bool,
) -> Result<PackageManager> {
    let pm = cli.or(config);
    if let Some(package_manager) = pm {
        return Ok(package_manager);
    }

    interactive_select(interactive)
}

pub fn resolve_script_runner(
    cli: Option<ScriptRunner>,
    config: Option<ScriptRunner>,
) -> Option<ScriptRunner> {
    cli.or(config)
}

fn interactive_select(interactive: bool) -> Result<PackageManager> {
    let is_interactive = interactive
        && !cfg!(test)
        && std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal();
    if !is_interactive {
        bail!(
            "unable to determine package manager, and interactive selection was disabled\n\nSet '{}' or configure '{}' in the config file.",
            cstr!("<bold>--pm <<PACKAGE_MANAGER>></>"),
            cstr!("<bold>pm</>")
        );
    }

    terminal::interactive_select(None).with_context(|| {
        format!(
            "unable to determine package manager\n\nSet '{}' or configure '{}' in the config file.",
            cstr!("<bold>--pm <<PACKAGE_MANAGER>></>"),
            cstr!("<bold>pm</>")
        )
    })
}
