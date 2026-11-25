use anyhow::Result;
use clap::{Command, Parser};

use crate::AppContext;

use super::unpm::UnpmArgs;
use super::{DlxArgs, dlx};

#[derive(Debug, Parser)]
#[command(
    name = "unpx",
    about = "Execute a command without installing packages (alias for `unpm dlx`)",
    version = concat!(color_print::cstr!("<strong,underline>Version:</>"), " ", env!("CARGO_PKG_VERSION")),
    help_template = "\
{before-help}{about-with-newline}
{usage-heading} {usage}

{version}

{all-args}{after-help}\
",
    disable_help_flag = true,
    disable_version_flag = true,
)]
pub struct Unpx {
    #[command(flatten)]
    pub args: UnpmArgs,

    #[command(flatten)]
    pub dlx_args: DlxArgs,
}

pub fn unpx(_: &mut Command) -> Result<()> {
    let cli = Unpx::parse();
    let ctx = &AppContext::new(
        cli.args.config,
        cli.args.pm,
        cli.args.runner,
        cli.args.dry_run,
    )?;

    dlx(ctx, &cli.dlx_args)
}
