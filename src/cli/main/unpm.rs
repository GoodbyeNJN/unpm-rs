use clap::Parser;

use crate::cli::MainArgs;
use crate::cli::sub::{
    Subcommands, add, complete, dlx, exec, install, list, pm, remove, run, update,
};
use crate::{Context, Error, Result};

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
    pub main_args: MainArgs,

    #[command(subcommand)]
    pub command: Option<Subcommands>,
}

impl Unpm {
    pub fn try_parse_for_complete() -> Result<Self> {
        // When completing, example command line looks like:
        // unpm -- unpm <SUBCOMMAND> <TAB>
        // Should skip the first "unpm" and "--"
        let args = std::env::args_os().skip(2);
        Self::try_parse_from(args).map_err(|err| Error::Raw(err.into()))
    }

    pub fn extends_ctx(&self, ctx: &mut Context) {
        use Subcommands::*;

        let Some(command) = &self.command else {
            return;
        };

        match command {
            Pm(args) => ctx.global = args.global,
            Install(_) => {}
            Add(args) => ctx.global = args.global,
            Remove(args) => ctx.global = args.global,
            List(args) => ctx.global = args.global,
            Update(args) => ctx.global = args.global,
            Dlx(_) => {}
            Exec(_) => {}
            Run(_) => {}
            Complete(_) => {}
        }
    }
}

pub fn unpm(cmd: &mut clap::Command) -> Result<()> {
    use Subcommands::*;

    let cli = Unpm::parse();
    let ctx = &mut Context::new(&cli.main_args)?;
    cli.extends_ctx(ctx);

    let Some(command) = &cli.command else {
        return cmd
            .print_long_help()
            .map_err(|err| Error::Raw(Box::new(err)));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::main::MainArgs;
    use crate::cli::sub::{AddArgs, ListArgs, RemoveArgs, Subcommands, UpdateArgs};

    fn main_args() -> MainArgs {
        MainArgs {
            config: None,
            pm: None,
            runner: None,
            dry_run: false,
        }
    }

    #[test]
    fn extends_ctx_sets_global_for_add_remove_list_update() {
        let mut ctx = Context::new(&main_args()).expect("expected context to build");

        let mut unpm = Unpm {
            main_args: main_args(),
            command: Some(Subcommands::Add(AddArgs {
                global: true,
                ..Default::default()
            })),
        };
        unpm.extends_ctx(&mut ctx);
        assert!(ctx.global);

        let mut ctx = Context::new(&main_args()).expect("expected context to build");
        unpm.command = Some(Subcommands::Remove(RemoveArgs {
            global: true,
            ..Default::default()
        }));
        unpm.extends_ctx(&mut ctx);
        assert!(ctx.global);

        let mut ctx = Context::new(&main_args()).expect("expected context to build");
        unpm.command = Some(Subcommands::List(ListArgs {
            global: true,
            ..Default::default()
        }));
        unpm.extends_ctx(&mut ctx);
        assert!(ctx.global);

        let mut ctx = Context::new(&main_args()).expect("expected context to build");
        unpm.command = Some(Subcommands::Update(UpdateArgs {
            global: true,
            ..Default::default()
        }));
        unpm.extends_ctx(&mut ctx);
        assert!(ctx.global);
    }

    #[test]
    fn extends_ctx_does_not_set_global_for_dlx() {
        let mut ctx = Context::new(&main_args()).expect("expected context to build");
        let unpm = Unpm {
            main_args: main_args(),
            command: Some(Subcommands::Dlx(Default::default())),
        };

        unpm.extends_ctx(&mut ctx);
        assert!(!ctx.global);
    }
}
