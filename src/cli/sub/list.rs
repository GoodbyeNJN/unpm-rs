use clap::{ArgAction, Args};

use crate::{Context, PackageManager, PackageManagerError, PackageManagerOperation, Result, bail};

/// List installed packages
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "ls",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm list -O -P
  <dim>$</> unpm list <underline>--json</>\
"),
)]
pub struct ListArgs {
    /// List global packages
    #[arg(short, long)]
    pub global: bool,

    /// List production dependencies only
    #[arg(short, long, conflicts_with_all = ["dev"])]
    pub prod: bool,

    /// List development dependencies only
    #[arg(short, long, conflicts_with_all = ["prod"])]
    pub dev: bool,

    /// Exclude optional dependencies
    #[arg(short = 'O', long = "no-optional", action = ArgAction::SetFalse)]
    pub optional: bool,

    /// Exclude peer dependencies
    #[arg(short = 'P', long = "no-peer", action = ArgAction::SetFalse)]
    pub peer: bool,

    /// Extra options and packages forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_OPTION | PACKAGE",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub forward_args: Vec<String>,
}

pub fn list(ctx: &Context, args: &ListArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = if args.global { GlobalList } else { List };
    let mut resolved_args = vec![];

    if args.prod {
        match pm {
            Npm => resolved_args.push("--omit=dev".to_owned()),
            Pnpm => resolved_args.push("--prod".to_owned()),
            Yarn | YarnBerry | Bun => {
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
            Npm => resolved_args.push("--omit=optional".to_owned()),
            Pnpm => resolved_args.push("--no-optional".to_owned()),
            Yarn | YarnBerry | Bun => {
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
            Npm => resolved_args.push("--omit=peer".to_owned()),
            Pnpm => resolved_args.push("--exclude-peers".to_owned()),
            Yarn | YarnBerry | Bun => {
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

#[cfg(test)]
mod tests {
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn list_rejects_prod_and_dev_together() {
        let err = Unpm::try_parse_from(["unpm", "list", "--prod", "--dev"])
            .expect_err("expected parse conflict");

        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn list_parses_global_negated_flags_and_forwarded_args() {
        let cli = Unpm::try_parse_from(["unpm", "list", "-g", "-O", "-P", "--json"])
            .expect("expected list parse");

        let Subcommands::List(args) = cli.command.expect("expected list command") else {
            panic!("expected list command");
        };

        assert!(args.global);
        assert!(!args.optional);
        assert!(!args.peer);
        assert_eq!(args.forward_args, vec!["--json"]);
    }
}
