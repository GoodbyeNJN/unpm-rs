use clap::Args;

use crate::{Context, PackageManager, PackageManagerOperation, Result};

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

pub fn pm(ctx: &Context, args: &PmArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = Pm;
    let mut resolved_args = vec![];

    resolved_args.extend(args.forward_args.to_owned());

    pm.run_operation(ctx, op, &resolved_args)
}

#[cfg(test)]
mod tests {
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::Parser;

    #[test]
    fn pm_allows_empty_forwarded_arguments() {
        let cli = Unpm::try_parse_from(["unpm", "pm"]).expect("expected pm parse");

        let Subcommands::Pm(args) = cli.command.expect("expected pm command") else {
            panic!("expected pm command");
        };

        assert!(!args.global);
        assert!(args.forward_args.is_empty());
    }

    #[test]
    fn pm_parses_global_and_forwarded_arguments() {
        let cli =
            Unpm::try_parse_from(["unpm", "pm", "-g", "--version"]).expect("expected pm parse");

        let Subcommands::Pm(args) = cli.command.expect("expected pm command") else {
            panic!("expected pm command");
        };

        assert!(args.global);
        assert_eq!(args.forward_args, vec!["--version"]);
    }
}
