use std::ffi::OsStr;

use anyhow::Result;
use clap::Args;
use clap_complete::CompletionCandidate;

use crate::{
    Adapter, AppContext, PackageManager, PackageManagerOperation, package_manager, process,
    registry,
};

use super::super::unpm_completer;

/// Add packages to the project or globally
#[derive(Clone, Debug, Default, Args)]
#[command(
    visible_alias = "a",
    after_help = color_print::cstr!("\
<strong,underline>Note:</>
  Unknown options and arguments are forwarded directly to the package manager
  Use '--' only when forwarding options that conflict with unpm's own options

<strong,underline>Examples:</>
  <underline>Underlined</> text represents forwarded options and arguments

  <dim>$</> unpm add -d <underline>lodash</>
  <dim>$</> unpm --dry-run add <underline>lodash</>
  <dim>$</> unpm add <underline>--dry-run lodash</>
  <dim>$</> unpm add -o <underline>--os linux lodash</>
  <dim>$</> unpm add --dev -- <underline>--dev lodash</>\
"),
)]
pub struct AddArgs {
    /// Add to global packages
    #[arg(short, long, conflicts_with_all = ["dev", "optional", "peer"])]
    pub global: bool,

    /// Add as development dependencies
    #[arg(short, long, conflicts_with_all = ["global", "optional", "peer"])]
    pub dev: bool,

    /// Add as optional dependencies
    #[arg(short, long, conflicts_with_all = ["global", "dev", "peer"])]
    pub optional: bool,

    /// Add as peer dependencies
    #[arg(short, long, conflicts_with_all = ["global", "dev", "optional"])]
    pub peer: bool,

    /// Use exact version
    #[arg(short, long)]
    pub exact: bool,

    /// Extra options and packages forwarded directly to the package manager
    #[arg(
        value_name = "FORWARD_OPTION | PACKAGE",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        add = unpm_completer(),
    )]
    pub forward_args: Vec<String>,
}

pub fn add(ctx: &AppContext, args: &AddArgs) -> Result<()> {
    use PackageManager::*;
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
    let op = if args.global { GlobalAdd } else { Add };
    let mut resolved_args = vec![];

    if args.dev {
        match pm {
            Npm | Pnpm => resolved_args.push("--save-dev".to_owned()),
            Yarn | YarnBerry | Bun => resolved_args.push("--dev".to_owned()),
        }
    } else if args.optional {
        match pm {
            Npm | Pnpm => resolved_args.push("--save-optional".to_owned()),
            Yarn | YarnBerry | Bun => resolved_args.push("--optional".to_owned()),
        }
    } else if args.peer {
        match pm {
            Npm | Pnpm => resolved_args.push("--save-peer".to_owned()),
            Yarn | YarnBerry | Bun => resolved_args.push("--peer".to_owned()),
        }
    }
    if args.exact {
        match pm {
            Npm | Pnpm => resolved_args.push("--save-exact".to_owned()),
            Yarn | YarnBerry | Bun => resolved_args.push("--exact".to_owned()),
        }
    }

    resolved_args.extend(args.forward_args.to_owned());

    let adapter = Adapter::for_package_manager(pm, op)?.set_extra_args(&resolved_args);
    process::spawn_command(adapter, ctx.dry_run)
}

pub fn add_completer(
    ctx: &AppContext,
    _: &AddArgs,
    current: &OsStr,
) -> Result<Vec<CompletionCandidate>> {
    let pkgs = registry::search_npm_registry(&ctx.registry, current.to_str())?;

    let max_version_len = pkgs
        .iter()
        .map(|pkg| pkg.version.len())
        .max()
        .unwrap_or_default();
    let candidates: Vec<_> = pkgs
        .into_iter()
        .map(|pkg| {
            let version = format!("v{}", pkg.version);
            let help_text = format!(
                "[{:>max_version_len$} | {}] {}",
                version, pkg.updated, pkg.description,
            );
            CompletionCandidate::new(pkg.name).help(Some(help_text.into()))
        })
        .collect();

    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use clap::{Parser, error::ErrorKind};

    use crate::Unpm;

    use super::super::Subcommand;
    use super::*;

    fn parse_add(args: &[&str]) -> AddArgs {
        let cli = Unpm::try_parse_from(args).expect("failed to parse test cli arguments");
        let Subcommand::Add(args) = cli.command.expect("expected command") else {
            panic!("expected add command");
        };

        args
    }

    #[test]
    fn add_collects_unknown_options_without_double_dash() {
        let args = parse_add(&["unpm", "add", "--dry-run", "lodash"]);

        assert_eq!(args.forward_args, vec!["--dry-run", "lodash"]);
        assert!(!args.dev);
        assert!(!args.exact);
    }

    #[test]
    fn add_keeps_known_flags_before_passthrough_args() {
        let args = parse_add(&[
            "unpm",
            "add",
            "--dev",
            "--audit-level",
            "moderate",
            "lodash",
        ]);

        assert!(args.dev);
        assert_eq!(
            args.forward_args,
            vec!["--audit-level", "moderate", "lodash"]
        );
    }

    #[test]
    fn add_still_supports_double_dash_for_conflicting_flags() {
        let args = parse_add(&["unpm", "add", "--", "--dev", "lodash"]);

        assert!(!args.dev);
        assert_eq!(args.forward_args, vec!["--dev", "lodash"]);
    }

    #[test]
    fn add_requires_at_least_one_forwarded_arg() {
        let err = Unpm::try_parse_from(["unpm", "add", "-d"]).expect_err("expected parse error");

        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

        let rendered = err.to_string();
        assert!(rendered.contains("FORWARD_OPTION | PACKAGE"));
        assert!(rendered.contains("Usage:"));
    }
}
