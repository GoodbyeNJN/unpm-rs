use clap::Parser;

use crate::cli::MainArgs;
use crate::cli::sub::{DlxArgs, dlx};
use crate::{Context, Error, Result};

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
    pub main_args: MainArgs,

    #[command(flatten)]
    pub dlx_args: DlxArgs,
}

impl Unpx {
    pub fn try_parse_for_complete() -> Result<Self> {
        // When completing, example command line looks like:
        // unpx -- unpx <TAB>
        // Should skip the first "unpx" and "--"
        let args = std::env::args_os().skip(2);
        Self::try_parse_from(args).map_err(|err| Error::Raw(err.into()))
    }

    pub fn extends_ctx(&self, _: &mut Context) {}
}

pub fn unpx(_: &mut clap::Command) -> Result<()> {
    let cli = Unpx::parse();
    let ctx = &mut Context::new(&cli.main_args)?;
    cli.extends_ctx(ctx);

    dlx(ctx, &cli.dlx_args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PackageManager;
    use clap::Parser;

    #[test]
    fn unpx_parses_forwarded_arguments_like_dlx() {
        let cli = Unpx::try_parse_from(["unpx", "tsx", "--no-cache", "hello.ts"])
            .expect("expected unpx arguments to parse");

        assert_eq!(cli.main_args.pm, None);
        assert!(cli.dlx_args.global);
        assert_eq!(
            cli.dlx_args.forward_args,
            vec!["tsx", "--no-cache", "hello.ts"]
        );
    }

    #[test]
    fn unpx_supports_global_flags_and_no_global_switch() {
        let cli = Unpx::try_parse_from([
            "unpx",
            "--pm",
            "npm",
            "--dry-run",
            "-G",
            "tsx",
            "--no-cache",
        ])
        .expect("expected unpx arguments to parse");

        assert_eq!(cli.main_args.pm, Some(PackageManager::Npm));
        assert!(cli.main_args.dry_run);
        assert!(!cli.dlx_args.global);
        assert_eq!(cli.dlx_args.forward_args, vec!["tsx", "--no-cache"]);
    }
}
