use clap::{ArgAction, Args};

use crate::{Context, PackageManager, PackageManagerOperation, Result};

/// Execute a command without installing packages
#[derive(Clone, Debug, Default, Args)]
#[command(
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm dlx -G <underline>tsx --no-cache hello.ts</>
  <dim>$</> unpm --dry-run dlx -- <underline>tsx --no-cache hello.ts</>\
"),
)]
pub struct DlxArgs {
    /// Do not use global package manager
    #[arg(short = 'G', long = "no-global", action = ArgAction::SetFalse)]
    pub global: bool,

    /// Extra arguments forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_ARGUMENT",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub forward_args: Vec<String>,
}

pub fn dlx(ctx: &Context, args: &DlxArgs) -> Result<()> {
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
    let op = Dlx;
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
    fn dlx_requires_forwarded_arguments() {
        let err = Unpm::try_parse_from(["unpm", "dlx"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn dlx_parses_no_global_and_forwarded_args() {
        let cli = Unpm::try_parse_from(["unpm", "dlx", "-G", "tsx", "--no-cache"])
            .expect("expected dlx parse");

        let Subcommands::Dlx(args) = cli.command.expect("expected dlx command") else {
            panic!("expected dlx command");
        };

        assert!(!args.global);
        assert_eq!(args.forward_args, vec!["tsx", "--no-cache"]);
    }
}
