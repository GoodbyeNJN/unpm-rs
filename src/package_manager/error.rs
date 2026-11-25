use color_print::cformat;

use super::{PackageManager, PackageManagerOperation, ScriptRunner, ScriptRunnerOperation};

pub fn unsupported_package_manager_operation(
    pm: &PackageManager,
    op: &PackageManagerOperation,
    option: &Option<String>,
) -> String {
    let option = if let Some(option) = option {
        cformat!(" with option '<yellow>{}</>'", option)
    } else {
        String::new()
    };

    format!("{op}{option} is not supported by {pm}")
}

pub fn unsupported_script_runner_operation(
    runner: &ScriptRunner,
    op: &ScriptRunnerOperation,
    option: &Option<String>,
) -> String {
    let option = if let Some(option) = option {
        cformat!(" with option '<yellow>{}</>'", option)
    } else {
        String::new()
    };

    format!("{op}{option} is not supported by {runner}")
}
