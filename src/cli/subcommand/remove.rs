use std::ffi::OsStr;

use anyhow::Result;
use clap::Args;
use clap_complete::CompletionCandidate;

use crate::{Adapter, AppContext, PackageManagerOperation, Project, package_manager, process};

use super::super::unpm_completer;

/// Remove packages from the project or globally
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "rm",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm remove -g <underline>tsx</>
  <dim>$</> unpm --dry-run remove <underline>lodash</>
  <dim>$</> unpm remove <underline>--dry-run lodash</>\
"),
)]
pub struct RemoveArgs {
    /// Remove from global packages
    #[arg(short, long)]
    pub global: bool,

    /// Extra options and packages forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_OPTION | PACKAGE",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        add = unpm_completer(),
    )]
    pub forward_args: Vec<String>,
}

pub fn remove(ctx: &AppContext, args: &RemoveArgs) -> Result<()> {
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
    let op = if args.global { GlobalRemove } else { Remove };
    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    let adapter = Adapter::for_package_manager(pm, op)?.set_extra_args(&resolved_args);
    process::spawn_command(adapter, ctx.dry_run)
}

pub fn remove_completer(
    ctx: &AppContext,
    args: &RemoveArgs,
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
    fn remove_parses_global_and_forwarded_arguments() {
        let cli = Unpm::try_parse_from(["unpm", "remove", "-g", "--dry-run", "tsx"])
            .expect("expected remove arguments to parse");

        let Subcommand::Remove(args) = cli.command.expect("expected command") else {
            panic!("expected remove command");
        };

        assert!(args.global);
        assert_eq!(args.forward_args, vec!["--dry-run", "tsx"]);
    }

    #[test]
    fn remove_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "remove"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }
}
