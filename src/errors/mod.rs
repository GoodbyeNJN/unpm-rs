use thiserror::Error;

mod command;
mod config;
mod package_manager;
mod script_runner;

pub use command::CommandError;
pub use config::{ConfigError, ConfigErrorFrom};
pub use package_manager::PackageManagerError;
pub use script_runner::ScriptRunnerError;

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[macro_export]
macro_rules! bail {
    ($err:expr $(,)?) => {
        return ::core::result::Result::Err(::core::convert::Into::into($err))
    };
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    PackageManager(#[from] PackageManagerError),

    #[error("{0}")]
    ScriptRunner(#[from] ScriptRunnerError),

    #[error("{0}")]
    Command(#[from] CommandError),

    #[error("{0}")]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Raw(BoxedError),
}

impl Error {
    pub fn exit(&self, cmd: &clap::Command) -> ! {
        let err = clap::Error::custom(self.to_string().into());
        err.with_cmd(cmd).exit();
    }
}
