use thiserror::Error;

use crate::{PackageManager, PackageManagerOperation};

#[derive(Debug, Error)]
pub enum PackageManagerError {
    #[error(
        "unable to determine package manager{}\n\nSet '{}' or configure '{}' in the config file.",
        if *.interactive { ", and interactive selection is disabled" } else { "" },
        color_print::cstr!("<bold>--pm <<PACKAGE_MANAGER>></>"),
        color_print::cstr!("<bold>pm</>")
    )]
    Undetermined { interactive: bool },

    #[error(
        "{operation}{} is not supported by {}",
        if let Some(option) = .option {
            format!(" with option '{}'", color_print::cformat!("<yellow>{}</>", option))
        } else {
            String::new()
        },
        color_print::cformat!("<yellow>{}</>", .package_manager)
    )]
    Unsupported {
        package_manager: PackageManager,
        operation: PackageManagerOperation,
        option: Option<String>,
    },
}
