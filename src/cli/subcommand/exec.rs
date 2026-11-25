use std::ffi::OsStr;

use anyhow::Result;
use clap::Args;
use clap_complete::CompletionCandidate;

use crate::{Adapter, AppContext, PackageManagerOperation, Project, package_manager, process};

use super::super::unpm_completer;

/// Execute a command from installed packages
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "x",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm exec <underline>tsx --no-cache hello.ts</>
  <dim>$</> unpm --dry-run exec -- <underline>tsx --no-cache hello.ts</>\
"),
)]
pub struct ExecArgs {
    /// Extra arguments forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_ARGUMENT",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        add = unpm_completer(),
    )]
    pub forward_args: Vec<String>,
}

pub fn exec(ctx: &AppContext, args: &ExecArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let pm = package_manager::resolve_local_package_manager(
        ctx.cli_pm,
        ctx.config_global_pm,
        ctx.interactive,
    )?;
    let op = Exec;
    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    let adapter = Adapter::for_package_manager(pm, op)?.set_extra_args(&resolved_args);
    process::spawn_command(adapter, ctx.dry_run)
}

pub fn exec_completer(
    _: &AppContext,
    _: &ExecArgs,
    current: &OsStr,
) -> Result<Vec<CompletionCandidate>> {
    let bins = Project::find_nearest(None)?.list_bins()?;

    let mut candidates: Vec<_> = bins
        .into_iter()
        .filter(|k| match current.to_str() {
            Some(current) => k.starts_with(current),
            None => true,
        })
        .map(CompletionCandidate::new)
        .collect();

    candidates.sort_by_key(|candidate| candidate.get_value().to_owned());
    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use clap::{Parser, error::ErrorKind};

    use crate::Unpm;

    use super::super::Subcommand;

    #[test]
    fn exec_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "exec"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn exec_parses_forwarded_command_line() {
        let cli = Unpm::try_parse_from(["unpm", "exec", "tsx", "--no-cache", "hello.ts"])
            .expect("expected exec parse");

        let Subcommand::Exec(args) = cli.command.expect("expected exec command") else {
            panic!("expected exec command");
        };

        assert_eq!(args.forward_args, vec!["tsx", "--no-cache", "hello.ts"]);
    }
}
