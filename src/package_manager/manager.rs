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
