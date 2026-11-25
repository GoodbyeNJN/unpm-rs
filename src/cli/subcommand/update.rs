use std::ffi::OsStr;

use anyhow::{Result, bail};
use clap::{ArgAction, Args};
use clap_complete::CompletionCandidate;

use crate::{
    Adapter, AppContext, PackageManager, PackageManagerOperation, Project, package_manager, process,
};

use super::super::unpm_completer;

/// Update installed packages
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "up",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm update <underline>lodash</>
  <dim>$</> unpm --dry-run update -i
  <dim>$</> unpm update <underline>--recursive</>\
"),
)]
pub struct UpdateArgs {
    /// Update global packages
    #[arg(short, long, conflicts_with_all = ["dev", "optional", "peer"])]
    pub global: bool,

    /// Update production dependencies only
    #[arg(short, long, conflicts_with_all = ["dev"])]
    pub prod: bool,

    /// Update development dependencies only
    #[arg(short, long, conflicts_with_all = ["prod"])]
    pub dev: bool,

    /// Exclude optional dependencies
    #[arg(short = 'O', long = "no-optional", action = ArgAction::SetFalse)]
    pub optional: bool,

    /// Exclude peer dependencies
    #[arg(short = 'P', long = "no-peer", action = ArgAction::SetFalse)]
    pub peer: bool,

    /// Use interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Extra options and packages forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_OPTION | PACKAGE",
        trailing_var_arg = true,
        allow_hyphen_values = true,
        add = unpm_completer(),
    )]
    pub forward_args: Vec<String>,
}

pub fn update(ctx: &AppContext, args: &UpdateArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = if args.global {
        package_manager::resolve_global_package_manager(
            ctx.cli_pm,
            ctx.config_global_pm,
            ctx.interactive,
        )?
    } else {
        package_manager::resolve_local_package_manager(
            ctx.cli_pm,
            ctx.config_global_pm,
            ctx.interactive,
        )?
    };
    let op = match (args.global, args.interactive) {
        (true, true) => GlobalInteractiveUpdate,
        (true, false) => GlobalUpdate,
        (false, true) => InteractiveUpdate,
        (false, false) => Update,
    };
    let mut resolved_args = vec![];

    if args.prod {
        match pm {
            Npm | Bun => resolved_args.push("--omit=dev".to_owned()),
            Pnpm => resolved_args.push("--prod".to_owned()),
            Yarn | YarnBerry => {
                bail!(
                    package_manager::error::unsupported_package_manager_operation(
                        &pm,
                        &op,
                        &Some("--prod".to_owned())
                    )
                );
            }
        }
    } else if args.dev {
        match pm {
            Pnpm => resolved_args.push("--dev".to_owned()),
            Npm | Yarn | YarnBerry | Bun => {
                bail!(
                    package_manager::error::unsupported_package_manager_operation(
                        &pm,
                        &op,
                        &Some("--dev".to_owned())
                    )
                );
            }
        }
    }
    if !args.optional {
        match pm {
            Npm | Bun => resolved_args.push("--omit=optional".to_owned()),
            Pnpm => resolved_args.push("--no-optional".to_owned()),
            Yarn | YarnBerry => {
                bail!(
                    package_manager::error::unsupported_package_manager_operation(
                        &pm,
                        &op,
                        &Some("--no-optional".to_owned())
                    )
                );
            }
        }
    }
    if !args.peer {
        match pm {
            Npm | Bun => resolved_args.push("--omit=peer".to_owned()),
            Pnpm | Yarn | YarnBerry => {
                bail!(
                    package_manager::error::unsupported_package_manager_operation(
                        &pm,
                        &op,
                        &Some("--no-peer".to_owned())
                    )
                );
            }
        }
    }

    resolved_args.extend(args.forward_args.to_owned());

    let adapter = Adapter::for_package_manager(pm, op)?.set_extra_args(&resolved_args);
    process::spawn_command(adapter, ctx.dry_run)
}

pub fn update_completer(
    ctx: &AppContext,
    args: &UpdateArgs,
    current: &OsStr,
) -> Result<Vec<CompletionCandidate>> {
    let deps = if args.global {
        let pm = package_manager::resolve_global_package_manager(
            ctx.cli_pm,
            ctx.config_global_pm,
            ctx.interactive,
        )?;
        package_manager::list_global_dependencies(&pm, current.to_str())?
    } else {
        Project::find_nearest(None)?.list_dependencies(current.to_str())?
    };

    let candidates = deps
        .into_iter()
        .map(|(k, v)| CompletionCandidate::new(k).help(Some(v.into())))
        .collect();

    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use clap::{Parser, error::ErrorKind};

    use crate::Unpm;

    use super::super::Subcommand;

    #[test]
    fn update_rejects_prod_and_dev_together() {
        let err = Unpm::try_parse_from(["unpm", "update", "--prod", "--dev"])
            .expect_err("expected parse conflict");

        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn update_parses_interactive_and_forwarded_args() {
        let cli = Unpm::try_parse_from(["unpm", "update", "-i", "-O", "--recursive", "lodash"])
            .expect("expected update parse");

        let Subcommand::Update(args) = cli.command.expect("expected command") else {
            panic!("expected update command");
        };

        assert!(args.interactive);
        assert!(!args.optional);
        assert_eq!(args.forward_args, vec!["--recursive", "lodash"]);
    }
}
