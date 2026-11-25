use anyhow::Result;
use clap::Args;

use crate::{Adapter, AppContext, PackageManagerOperation, package_manager, process};

/// Forward options and arguments directly to the package manager
#[derive(Clone, Debug, Default, Args)]
#[command(
        after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm pm <underline>--version</>
  <dim>$</> unpm pm -g -- <underline>--version</>\
"),
)]
pub struct PmArgs {
    /// Use global package manager
    #[arg(short, long)]
    pub global: bool,

    /// Extra arguments forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_ARGUMENT",
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub forward_args: Vec<String>,
}

pub fn pm(ctx: &AppContext, args: &PmArgs) -> Result<()> {
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
    let op = Pm;
    let mut resolved_args = vec![];

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
    fn pm_allows_empty_forwarded_arguments() {
        let cli = Unpm::try_parse_from(["unpm", "pm"]).expect("expected pm parse");

        let Subcommand::Pm(args) = cli.command.expect("expected pm command") else {
            panic!("expected pm command");
        };

        assert!(!args.global);
        assert!(args.forward_args.is_empty());
    }

    #[test]
    fn pm_parses_global_and_forwarded_arguments() {
        let cli =
            Unpm::try_parse_from(["unpm", "pm", "-g", "--version"]).expect("expected pm parse");

        let Subcommand::Pm(args) = cli.command.expect("expected pm command") else {
            panic!("expected pm command");
        };

        assert!(args.global);
        assert_eq!(args.forward_args, vec!["--version"]);
    }
}
