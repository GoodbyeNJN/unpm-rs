use std::io::IsTerminal;
use std::path::Path;

use crate::package_manager::adaptors::CommandAdaptor;
use crate::package_manager::search::PackageInfo;
use crate::{Context, PackageManagerError, Result, bail};

mod adaptors;
mod detect;
mod enums;
mod fs;
mod project;
mod search;
mod select;

pub use enums::*;
pub use project::Project;

#[bon::bon]
impl PackageManager {
    pub fn resolve(ctx: &Context) -> Result<Self> {
        let pm = if ctx.global {
            ctx.cli_pm.or(ctx.config_global_pm)
        } else {
            // Priority: cli > detect > config
            ctx.cli_pm
                .or_else(|| detect::detect_package_manager(None, None))
                .or(ctx.config_local_pm)
        };

        if let Some(package_manager) = pm {
            return Ok(package_manager);
        }

        let is_interactive = ctx.interactive
            && !cfg!(test)
            && std::io::stdin().is_terminal()
            && std::io::stdout().is_terminal();
        if !is_interactive {
            bail!(PackageManagerError::Undetermined { interactive: false });
        }

        select::interactive_select(None)?
            .ok_or_else(|| PackageManagerError::Undetermined { interactive: true }.into())
    }

    pub fn run_operation<T, S>(
        &self,
        ctx: &Context,
        op: PackageManagerOperation,
        args: T,
    ) -> Result<()>
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        CommandAdaptor::for_pm(*self, op)?
            .set_extra_args(args)
            .run(ctx.dry_run)
    }

    #[builder]
    pub fn search_npm_registry(ctx: &Context, query: Option<&str>) -> Option<Vec<PackageInfo>> {
        let query = match query {
            Some(s) if (2..=64).contains(&s.len()) => s,
            _ => return None,
        };

        search::search_npm_registry(query, &ctx.registry).ok()
    }

    #[builder]
    pub fn list_bins(cwd: Option<&Path>) -> Option<Vec<String>> {
        Project::find_nearest(cwd)?.list_bins()
    }

    #[builder]
    pub fn list_scripts(prefix: Option<&str>, cwd: Option<&Path>) -> Option<Vec<(String, String)>> {
        Project::find_nearest(cwd)?.list_scripts(prefix)
    }

    #[builder]
    pub fn list_local_dependencies(
        prefix: Option<&str>,
        cwd: Option<&Path>,
    ) -> Option<Vec<(String, String)>> {
        Project::find_nearest(cwd)?.list_dependencies(prefix)
    }

    #[builder]
    pub fn list_global_dependencies(&self, prefix: Option<&str>) -> Option<Vec<(String, String)>> {
        use PackageManager::*;
        use PackageManagerOperation::*;

        if *self != Npm && *self != Pnpm {
            return None;
        }

        let adaptor = CommandAdaptor::for_pm(*self, GlobalList)
            .ok()?
            .set_extra_args(["--json"]);
        let (_, stdout, _) = adaptor.silent_run().ok()?;

        let json: serde_json::Value = serde_json::from_str(&stdout).ok()?;

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
        let iter = if json.is_array() {
            json.as_array()?.iter().next()?
        } else {
            &json
        }
        .get("dependencies")?
        .as_object()?
        .keys();

        let deps = iter
            .filter(|k| match prefix {
                Some(prefix) => k.starts_with(prefix),
                None => true,
            })
            .map(|k| (k.to_owned(), "global packages".to_owned()))
            .collect();

        Some(deps)
    }
}

impl ScriptRunner {
    pub fn resolve(ctx: &Context) -> Option<Self> {
        ctx.cli_runner.or(ctx.config_runner)
    }

    pub fn run_script<T, S>(&self, ctx: &Context, args: T) -> Result<()>
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        CommandAdaptor::for_runner(*self, ScriptRunnerOperation::Run)?
            .set_extra_args(args)
            .run(ctx.dry_run)
    }
}
