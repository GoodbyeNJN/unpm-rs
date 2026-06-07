use std::ffi::OsStr;

use clap::Args;
use clap_complete::CompletionCandidate;

use crate::cli::completion::wrap_completer;
use crate::{Context, PackageManager, PackageManagerOperation, Result};

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
        add = wrap_completer(complete)
    )]
    pub forward_args: Vec<String>,
}

pub fn remove(ctx: &Context, args: &RemoveArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = if args.global { GlobalRemove } else { Remove };
    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    pm.run_operation(ctx, op, &resolved_args)
}

fn complete(ctx: &Context, current: &OsStr) -> Option<Vec<CompletionCandidate>> {
    let deps = if !ctx.global {
        PackageManager::list_local_dependencies()
            .maybe_prefix(current.to_str())
            .call()
    } else {
        PackageManager::resolve(ctx)
            .ok()?
            .list_global_dependencies()
            .maybe_prefix(current.to_str())
            .call()
    };

    let candidates = deps?
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
    fn remove_parses_global_and_forwarded_arguments() {
        let cli = Unpm::try_parse_from(["unpm", "remove", "-g", "--dry-run", "tsx"])
            .expect("expected remove arguments to parse");

        let Subcommands::Remove(args) = cli.command.expect("expected command") else {
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
