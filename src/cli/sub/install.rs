use clap::{ArgAction, Args};

use crate::{Context, PackageManager, PackageManagerError, PackageManagerOperation, Result, bail};

/// Install project dependencies
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "i",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm install -p --frozen
  <dim>$</> unpm install <underline>--os linux</>
  <dim>$</> unpm install -- <underline>--no-optional</>\
"),
)]
pub struct InstallArgs {
    /// Install production dependencies only
    #[arg(short, long)]
    pub prod: bool,

    /// Exclude optional dependencies
    #[arg(short = 'O', long = "no-optional", action = ArgAction::SetFalse)]
    pub optional: bool,

    /// Exclude peer dependencies
    #[arg(short = 'P', long = "no-peer", action = ArgAction::SetFalse)]
    pub peer: bool,

    /// Do not update lockfile
    #[arg(long)]
    pub frozen: bool,

    /// Extra arguments forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_ARGUMENT",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub forward_args: Vec<String>,
}

pub fn install(ctx: &Context, args: &InstallArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = if args.frozen { FrozenInstall } else { Install };
    let mut resolved_args = vec![];

    if args.prod {
        match pm {
            Npm | Bun => resolved_args.push("--omit=dev".to_owned()),
            Pnpm => resolved_args.push("--prod".to_owned()),
            Yarn => resolved_args.push("--production".to_owned()),
            YarnBerry => {
                bail!(PackageManagerError::Unsupported {
                    package_manager: pm,
                    operation: op,
                    option: Some("--prod".to_owned()),
                });
            }
        }
    }
    if !args.optional {
        match pm {
            Npm | Bun => resolved_args.push("--omit=optional".to_owned()),
            Pnpm => resolved_args.push("--no-optional".to_owned()),
            Yarn => resolved_args.push("--ignore-optional".to_owned()),
            YarnBerry => {
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

#[cfg(test)]
mod tests {
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::Parser;

    #[test]
    fn install_uses_inclusive_defaults() {
        let cli = Unpm::try_parse_from(["unpm", "install"]).expect("expected install parse");

        let Subcommands::Install(args) = cli.command.expect("expected install command") else {
            panic!("expected install command");
        };

        assert!(!args.prod);
        assert!(args.optional);
        assert!(args.peer);
        assert!(!args.frozen);
        assert!(args.forward_args.is_empty());
    }

    #[test]
    fn install_parses_negated_dependency_flags_and_forwarded_args() {
        let cli = Unpm::try_parse_from([
            "unpm",
            "install",
            "-p",
            "-O",
            "-P",
            "--frozen",
            "--ignore-scripts",
        ])
        .expect("expected install parse");

        let Subcommands::Install(args) = cli.command.expect("expected install command") else {
            panic!("expected install command");
        };

        assert!(args.prod);
        assert!(!args.optional);
        assert!(!args.peer);
        assert!(args.frozen);
        assert_eq!(args.forward_args, vec!["--ignore-scripts"]);
    }
}
