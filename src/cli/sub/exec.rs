use std::ffi::OsStr;

use clap::Args;
use clap_complete::CompletionCandidate;

use crate::cli::completion::wrap_completer;
use crate::{Context, PackageManager, PackageManagerOperation, Result};

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
        // add = ArgValueCompleter::new(exec_completer)
        add = wrap_completer(complete)
    )]
    pub forward_args: Vec<String>,
}

pub fn exec(ctx: &Context, args: &ExecArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = Exec;
    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    pm.run_operation(ctx, op, &resolved_args)
}

#[cfg(test)]
mod tests {
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn exec_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "exec"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn exec_parses_forwarded_command_line() {
        let cli = Unpm::try_parse_from(["unpm", "exec", "tsx", "--no-cache", "hello.ts"])
            .expect("expected exec parse");

        let Subcommands::Exec(args) = cli.command.expect("expected exec command") else {
            panic!("expected exec command");
        };

        assert_eq!(args.forward_args, vec!["tsx", "--no-cache", "hello.ts"]);
    }
}

fn complete(_: &Context, current: &OsStr) -> Option<Vec<CompletionCandidate>> {
    let bins = PackageManager::list_bins().call()?;

    let mut candidates: Vec<_> = bins
        .into_iter()
        .filter(|k| match current.to_str() {
            Some(current) => k.starts_with(current),
            None => true,
        })
        .map(CompletionCandidate::new)
        .collect();

    candidates.sort_by_key(|candidate| candidate.get_value().to_owned());
    Some(candidates)
}
