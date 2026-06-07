use std::ffi::OsStr;

use clap::{ArgAction, Args};
use clap_complete::CompletionCandidate;

use crate::cli::completion::wrap_completer;
use crate::{Context, PackageManager, PackageManagerError, PackageManagerOperation, Result, bail};

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
        add = wrap_completer(complete)
    )]
    pub forward_args: Vec<String>,
}

pub fn update(ctx: &Context, args: &UpdateArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
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
                bail!(PackageManagerError::Unsupported {
                    package_manager: pm,
                    operation: op,
                    option: Some("--prod".to_owned()),
                });
            }
        }
    } else if args.dev {
        match pm {
            Pnpm => resolved_args.push("--dev".to_owned()),
            Npm | Yarn | YarnBerry | Bun => {
                bail!(PackageManagerError::Unsupported {
                    package_manager: pm,
                    operation: op,
                    option: Some("--dev".to_owned()),
                });
            }
        }
    }
    if !args.optional {
        match pm {
            Npm | Bun => resolved_args.push("--omit=optional".to_owned()),
            Pnpm => resolved_args.push("--no-optional".to_owned()),
            Yarn | YarnBerry => {
                bail!(PackageManagerError::Unsupported {
                    package_manager: pm,
                    operation: op,
                    option: Some("--no-optional".to_owned()),
                });
            }
        }
    }
    if !args.peer {
        match pm {
            Npm | Bun => resolved_args.push("--omit=peer".to_owned()),
            Pnpm | Yarn | YarnBerry => {
                bail!(PackageManagerError::Unsupported {
                    package_manager: pm,
                    operation: op,
                    option: Some("--no-peer".to_owned()),
                });
            }
        }
    }

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
    fn update_rejects_prod_and_dev_together() {
        let err = Unpm::try_parse_from(["unpm", "update", "--prod", "--dev"])
            .expect_err("expected parse conflict");

        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn update_parses_interactive_and_forwarded_args() {
        let cli = Unpm::try_parse_from(["unpm", "update", "-i", "-O", "--recursive", "lodash"])
            .expect("expected update parse");

        let Subcommands::Update(args) = cli.command.expect("expected command") else {
            panic!("expected update command");
        };

        assert!(args.interactive);
        assert!(!args.optional);
        assert_eq!(args.forward_args, vec!["--recursive", "lodash"]);
    }
}
