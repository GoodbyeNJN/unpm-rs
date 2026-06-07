use thiserror::Error;

use crate::{ScriptRunner, ScriptRunnerOperation};

#[derive(Debug, Error)]
pub enum ScriptRunnerError {
    #[error(
        "unable to determine script runner{}\n\nSet '{}' or configure '{}' in the config file.",
        if *.interactive { ", and interactive selection is disabled" } else { "" },
        color_print::cstr!("<bold>--runner <<SCRIPT_RUNNER>></>"),
        color_print::cstr!("<bold>runner</>")
    )]
    Undetermined { interactive: bool },

    #[error(
        "{operation}{} is not supported by {}",
        if let Some(option) = .option {
            format!(" with option '{}'", color_print::cformat!("<yellow>{}</>", option))
        } else {
            String::new()
        },
        color_print::cformat!("<yellow>{}</>", .script_runner)
    )]
    Unsupported {
        script_runner: ScriptRunner,
        operation: ScriptRunnerOperation,
        option: Option<String>,
    },
}
