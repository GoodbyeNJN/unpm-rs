use std::ffi::OsStr;

use clap::Args;
use clap_complete::engine::CompletionCandidate;

use crate::cli::completion::wrap_completer;
use crate::{Context, PackageManager, PackageManagerOperation, Result, ScriptRunner};

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
        add = wrap_completer(complete)
    )]
    pub forward_args: Vec<String>,
}

pub fn run(ctx: &Context, args: &RunArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    if let Some(runner) = ScriptRunner::resolve(ctx) {
        return runner.run_script(ctx, &resolved_args);
    }

    let pm = PackageManager::resolve(ctx)?;
    let op = Run;

    pm.run_operation(ctx, op, &resolved_args)
}

fn complete(_: &Context, current: &OsStr) -> Option<Vec<CompletionCandidate>> {
    let scripts = PackageManager::list_scripts()
        .maybe_prefix(current.to_str())
        .call()?;

    let candidates = scripts
        .into_iter()
        .map(|(k, v)| CompletionCandidate::new(k).help(Some(v.into())))
        .collect();

    Some(candidates)
}

#[cfg(test)]
mod tests {
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn run_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "run"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn run_parses_script_and_arguments() {
        let cli =
            Unpm::try_parse_from(["unpm", "run", "build", "--watch"]).expect("expected run parse");

        let Subcommands::Run(args) = cli.command.expect("expected command") else {
            panic!("expected run command");
        };

        assert_eq!(args.forward_args, vec!["build", "--watch"]);
    }
}
