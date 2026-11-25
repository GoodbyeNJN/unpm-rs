use anyhow::{Result, anyhow};
use dialoguer::Select;
use strum::IntoEnumIterator;

use crate::PackageManager;

pub fn interactive_select(prompt: Option<&str>) -> Result<PackageManager> {
    let prompt = prompt.unwrap_or(color_print::cstr!(
        "<yellow>warn:</> unable to determine package manager

<green>Select one to continue</>"
    ));
    let options: Vec<_> = PackageManager::iter().collect();

    let selection = Select::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact_opt()?;

    selection
        .map(|i| options.get(i).copied())
        .flatten()
        .ok_or(anyhow!("no package manager selected"))
}
