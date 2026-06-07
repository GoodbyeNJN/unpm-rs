use std::path::PathBuf;

use thiserror::Error;

use crate::BoxedError;

#[derive(Debug, Error)]
pub enum ConfigErrorFrom {
    #[error(
        "{} (from '{}')",
        color_print::cformat!("<yellow>{}</>", .0.display()),
        color_print::cstr!("<bold>--config <<PATH>></>")
    )]
    Arg(PathBuf),

    #[error(
        "{} (from environment variable '{}')",
        color_print::cformat!("<yellow>{}</>", .0.display()),
        color_print::cformat!("<bold>{}</>", crate::constants::ENV_CONFIG_FILE)
    )]
    Env(PathBuf),

    #[error(
        "{} (from default config file path)",
        color_print::cformat!("<yellow>{}</>", .0.display())
    )]
    Default(PathBuf),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file does not exist: {from}")]
    NotExists { from: ConfigErrorFrom },

    #[error(
        "failed to load config file: {from}\n       {} {source}",
        color_print::cstr!("<red,bold>cause:</>")
    )]
    Load {
        from: ConfigErrorFrom,
        source: BoxedError,
    },

    #[error(
        "failed to save config file: {from}\n       {} {source}",
        color_print::cstr!("<red,bold>cause:</>")
    )]
    Save {
        from: ConfigErrorFrom,
        source: BoxedError,
    },
}
