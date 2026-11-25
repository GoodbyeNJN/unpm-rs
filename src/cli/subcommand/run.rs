use std::ffi::OsStr;

use anyhow::Result;
use clap::Args;
use clap_complete::engine::CompletionCandidate;

use crate::{
    Adapter, AppContext, PackageManagerOperation, Project, ScriptRunnerOperation, package_manager,
    process,
};

use super::super::unpm_completer;

/// Run a script defined in package.json
#[derive(Clone, Debug, Default, Args)]
#[command(
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm run <underline>build</>
  <dim>$</> unpm --dry-run run -- <underline>build --watch</>\
"),
)]
pub struct RunArgs {
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

pub fn run(ctx: &AppContext, args: &RunArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    if let Some(runner) = package_manager::resolve_script_runner(ctx.cli_runner, ctx.config_runner)
    {
        let adapter = Adapter::for_script_runner(runner, ScriptRunnerOperation::Run)?
            .set_extra_args(&resolved_args);
        return process::spawn_command(adapter, ctx.dry_run);
    }

    let pm = package_manager::resolve_local_package_manager(
        ctx.cli_pm,
        ctx.config_global_pm,
        ctx.interactive,
    )?;
    let op = Run;

    let adapter = Adapter::for_package_manager(pm, op)?.set_extra_args(&resolved_args);
    process::spawn_command(adapter, ctx.dry_run)
}

pub fn run_completer(
    _: &AppContext,
    _: &RunArgs,
    current: &OsStr,
) -> Result<Vec<CompletionCandidate>> {
    let scripts = Project::find_nearest(None)?.list_scripts(current.to_str())?;

    let candidates = scripts
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
    fn run_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "run"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn run_parses_script_and_arguments() {
        let cli =
            Unpm::try_parse_from(["unpm", "run", "build", "--watch"]).expect("expected run parse");

        let Subcommand::Run(args) = cli.command.expect("expected command") else {
            panic!("expected run command");
        };

        assert_eq!(args.forward_args, vec!["build", "--watch"]);
    }
}
