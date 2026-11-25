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

pub use add::{AddArgs, add, add_completer};
pub use complete::{CompleteArgs, complete};
pub use dlx::{DlxArgs, dlx};
pub use exec::{ExecArgs, exec, exec_completer};
pub use install::{InstallArgs, install};
pub use list::{ListArgs, list};
pub use pm::{PmArgs, pm};
pub use remove::{RemoveArgs, remove, remove_completer};
pub use run::{RunArgs, run, run_completer};
pub use update::{UpdateArgs, update, update_completer};

#[derive(Clone, Debug, EnumString, clap::Subcommand)]
#[strum(serialize_all = "kebab-case")]
pub enum Subcommand {
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
