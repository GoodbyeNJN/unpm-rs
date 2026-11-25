use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;

use crate::{package_manager, process};

use super::{Adapter, PackageManager, PackageManagerOperation};

pub fn list_global_dependencies(
    pm: &PackageManager,
    prefix: Option<&str>,
) -> Result<Vec<(String, &'static str)>> {
    let op = PackageManagerOperation::GlobalList;
    let adapter = match pm {
        PackageManager::Npm => Adapter::for_npm_global_list_json(),
        PackageManager::Pnpm => Adapter::for_pnpm_global_list_json(),
        _ => bail!(package_manager::error::unsupported_package_manager_operation(&pm, &op, &None)),
    };

    let (_, stdout, _) = process::spawn_command_silent(adapter)?;
    let json: Value = serde_json::from_str(&stdout)
        .with_context(|| format!("{op} by {pm} returned invalid output"))?;

    /*
     * Global list json output examples:

     * npm:
    {
        ...
        "dependencies": {
            "prettier": {
                "version": "3.8.3",
                ...
            }
        }
    }

     * pnpm v10:
    [
        {
            ...
            "dependencies": {
                "prettier": {
                    "from": "prettier",
                    "version": "3.8.3",
                    ...
                }
            }
        }
    ]

     * pnpm v11:
    [
        {
            ...
            "dependencies": {
                "prettier": {
                    "from": "prettier",
                    "version": "3.8.3",
                    ...
                }
            }
        }
    ]
     */
    let root = if let Some(array) = json.as_array() {
        let Some(root) = array.first() else {
            return Ok(vec![]);
        };
        root
    } else {
        &json
    };
    let dependencies = root
        .get("dependencies")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            anyhow!("missing dependencies object")
                .context(format!("{op} from {pm} returned invalid output"))
        })?;

    let deps = dependencies
        .keys()
        .filter(|k| match prefix {
            Some(prefix) => k.starts_with(prefix),
            None => true,
        })
        .map(|k| (k.to_owned(), "global packages"))
        .collect();

    Ok(deps)
}
