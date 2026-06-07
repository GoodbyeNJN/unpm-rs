use dialoguer::Select;
use strum::IntoEnumIterator;

use crate::{Error, PackageManager, Result};

pub fn interactive_select(prompt: Option<&str>) -> Result<Option<PackageManager>> {
    let prompt = prompt.unwrap_or(color_print::cstr!(
        "<yellow>warn:</> unable to determine package manager

<green>Select one to continue</>"
    ));
    let options: Vec<_> = PackageManager::iter().collect();

    let selection = Select::new()
        .with_prompt(prompt)
        .items(&options)
        .default(0)
        .interact_opt()
        .map_err(|err| Error::Raw(Box::new(err)))?;

    if let Some(selection) = selection {
        Ok(options.get(selection).copied())
    } else {
        Ok(None)
    }
}
