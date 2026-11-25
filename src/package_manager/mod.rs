mod adaptors;
mod dependency;
mod detect;
pub mod error;
mod manager;
mod operation;
mod resolve;

pub use adaptors::Adapter;
pub use dependency::list_global_dependencies;
pub use manager::{PackageManager, ScriptRunner};
pub use operation::{PackageManagerOperation, ScriptRunnerOperation};
pub use resolve::{
    resolve_global_package_manager, resolve_local_package_manager, resolve_script_runner,
};
