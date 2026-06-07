use clap::Subcommand;
use strum::EnumString;

mod add;
mod complete;
mod dlx;
mod exec;
mod install;
mod list;
mod pm;
mod remove;
mod run;
mod update;

pub use crate::cli::sub::add::{AddArgs, add};
pub use crate::cli::sub::complete::{CompleteArgs, complete};
pub use crate::cli::sub::dlx::{DlxArgs, dlx};
pub use crate::cli::sub::exec::{ExecArgs, exec};
pub use crate::cli::sub::install::{InstallArgs, install};
pub use crate::cli::sub::list::{ListArgs, list};
pub use crate::cli::sub::pm::{PmArgs, pm};
pub use crate::cli::sub::remove::{RemoveArgs, remove};
pub use crate::cli::sub::run::{RunArgs, run};
pub use crate::cli::sub::update::{UpdateArgs, update};

#[derive(Clone, Debug, EnumString, Subcommand)]
#[strum(serialize_all = "kebab-case")]
pub enum Subcommands {
    Pm(PmArgs),
    Install(InstallArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    List(ListArgs),
    Update(UpdateArgs),
    Dlx(DlxArgs),
    Exec(ExecArgs),
    Run(RunArgs),
    Complete(CompleteArgs),
}
