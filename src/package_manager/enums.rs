use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Display,
    AsRefStr,
    EnumString,
    EnumIter,
    Serialize,
    Deserialize,
    ValueEnum,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum PackageManager {
    Npm,

    Pnpm,

    /// Yarn Classic (v1)
    Yarn,

    /// Yarn Berry (v2+)
    YarnBerry,

    Bun,
}

#[derive(
    Clone, Copy, Debug, Display, AsRefStr, EnumString, EnumIter, Serialize, Deserialize, ValueEnum,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ScriptRunner {
    /// Built-in script runner provided by Node.js, requires v22+
    Node,
}

#[derive(Clone, Copy, Debug, Display, AsRefStr)]
pub enum PackageManagerOperation {
    #[strum(serialize = "forward to package manager")]
    Pm,

    #[strum(serialize = "install dependencies")]
    Install,
    #[strum(serialize = "frozen install dependencies")]
    FrozenInstall,

    #[strum(serialize = "add packages")]
    Add,
    #[strum(serialize = "add global packages")]
    GlobalAdd,

    #[strum(serialize = "remove packages")]
    Remove,
    #[strum(serialize = "remove global packages")]
    GlobalRemove,

    #[strum(serialize = "list packages")]
    List,
    #[strum(serialize = "list global packages")]
    GlobalList,

    #[strum(serialize = "update packages")]
    Update,
    #[strum(serialize = "update global packages")]
    GlobalUpdate,
    #[strum(serialize = "interactive update packages")]
    InteractiveUpdate,
    #[strum(serialize = "interactive update global packages")]
    GlobalInteractiveUpdate,

    #[strum(serialize = "execute package commands without install")]
    Dlx,
    #[strum(serialize = "execute installed package commands")]
    Exec,
    #[strum(serialize = "run scripts")]
    Run,
}

#[derive(Clone, Copy, Debug, Display, AsRefStr)]
pub enum ScriptRunnerOperation {
    #[strum(serialize = "run scripts")]
    Run,
}
