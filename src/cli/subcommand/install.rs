use anyhow::{Result, bail};
use clap::{ArgAction, Args};

use crate::{
    Adapter, AppContext, PackageManager, PackageManagerOperation, package_manager, process,
};

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

pub fn install(ctx: &AppContext, args: &InstallArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = package_manager::resolve_local_package_manager(
        ctx.cli_pm,
        ctx.config_global_pm,
        ctx.interactive,
    )?;
    let op = if args.frozen { FrozenInstall } else { Install };
    let mut resolved_args = vec![];

    if args.prod {
        match pm {
            Npm | Bun => resolved_args.push("--omit=dev".to_owned()),
            Pnpm => resolved_args.push("--prod".to_owned()),
            Yarn => resolved_args.push("--production".to_owned()),
            YarnBerry => {
                bail!(
                    package_manager::error::unsupported_package_manager_operation(
                        &pm,
                        &op,
                        &Some("--prod".to_owned())
                    )
                );
            }
        }
    }
    if !args.optional {
        match pm {
            Npm | Bun => resolved_args.push("--omit=optional".to_owned()),
            Pnpm => resolved_args.push("--no-optional".to_owned()),
            Yarn => resolved_args.push("--ignore-optional".to_owned()),
            YarnBerry => {
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::Unpm;

    use super::super::Subcommand;

    #[test]
    fn install_uses_inclusive_defaults() {
        let cli = Unpm::try_parse_from(["unpm", "install"]).expect("expected install parse");

        let Subcommand::Install(args) = cli.command.expect("expected install command") else {
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

        let Subcommand::Install(args) = cli.command.expect("expected install command") else {
            panic!("expected install command");
        };

        assert!(args.prod);
        assert!(!args.optional);
        assert!(!args.peer);
        assert!(args.frozen);
        assert_eq!(args.forward_args, vec!["--ignore-scripts"]);
    }
}
