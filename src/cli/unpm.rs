use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Command, Parser};
use clap_complete::{ArgValueCompleter, CompletionCandidate};
use log::{debug, error};

use crate::{AppContext, PackageManager, ScriptRunner};

use super::{
    Subcommand, add, add_completer, complete, dlx, exec, exec_completer, install, list, pm, remove,
    remove_completer, run, run_completer, update, update_completer,
};

#[derive(Clone, Debug, Args)]
pub struct UnpmArgs {
    /// Path to the configuration file
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// The package manager to use
    #[arg(long, value_name = "PACKAGE_MANAGER")]
    pub pm: Option<PackageManager>,

    /// The script runner to use
    #[arg(long, value_name = "SCRIPT_RUNNER")]
    pub runner: Option<ScriptRunner>,

    /// Print the command without executing it
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Parser)]
#[command(
    name = "unpm",
    about = color_print::cstr!("<strong,underline>UN</>ified <strong,underline>P</>ackage <strong,underline>M</>anager for Node.js"),
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
pub struct Unpm {
    #[command(flatten)]
    pub args: UnpmArgs,

    #[command(subcommand)]
    pub command: Option<Subcommand>,
}

impl Unpm {
    pub fn try_parse_for_complete() -> Result<Self> {
        // When completing, example command line looks like:
        // unpm -- unpm <SUBCOMMAND> <TAB>
        // Should skip the first "unpm" and "--"
        let args = env::args_os().skip(2);

        Ok(Self::try_parse_from(args)?)
    }
}

pub fn unpm(cmd: &mut Command) -> Result<()> {
    use Subcommand::*;

    let cli = Unpm::parse();
    let ctx = &AppContext::new(
        cli.args.config,
        cli.args.pm,
        cli.args.runner,
        cli.args.dry_run,
    )?;

    let Some(command) = &cli.command else {
        cmd.print_long_help()?;

        return Ok(());
    };

    match command {
        Pm(args) => pm(ctx, args),
        Install(args) => install(ctx, args),
        Add(args) => add(ctx, args),
        Remove(args) => remove(ctx, args),
        List(args) => list(ctx, args),
        Update(args) => update(ctx, args),
        Dlx(args) => dlx(ctx, args),
        Exec(args) => exec(ctx, args),
        Run(args) => run(ctx, args),
        Complete(args) => {
            println!("{}", complete(ctx, args));
            Ok(())
        }
    }
}

pub fn unpm_completer() -> ArgValueCompleter {
    use Subcommand::*;

    ArgValueCompleter::new(move |current: &OsStr| {
        let result = (|| -> Result<Vec<CompletionCandidate>> {
            // When completing, example command line looks like:
            // unpm -- unpm <SUBCOMMAND> <TAB>
            // Should skip the first "unpm" and "--"
            let args = env::args_os().skip(2);
            let cli = Unpm::try_parse_from(args)?;
            let ctx = &AppContext::new(
                cli.args.config,
                cli.args.pm,
                cli.args.runner,
                cli.args.dry_run,
            )?;

            let Some(command) = &cli.command else {
                return Ok(vec![]);
            };

            match command {
                Add(args) => add_completer(ctx, args, current),
                Remove(args) => remove_completer(ctx, args, current),
                Update(args) => update_completer(ctx, args, current),
                Exec(args) => exec_completer(ctx, args, current),
                Run(args) => run_completer(ctx, args, current),
                _ => Ok(vec![]),
            }
        })();

        match result {
            Ok(candidates) => {
                debug!(
                    "completion provider returned {} candidates",
                    candidates.len()
                );
                candidates
            }
            Err(err) => {
                error!("completion provider failed: {err}");
                vec![]
            }
        }
    })
}
