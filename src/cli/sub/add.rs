use std::ffi::OsStr;

use clap::Args;
use clap_complete::CompletionCandidate;

use crate::cli::completion::wrap_completer;
use crate::{Context, PackageManager, PackageManagerOperation, Result};

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
        add = wrap_completer(complete)
    )]
    pub forward_args: Vec<String>,
}

pub fn add(ctx: &Context, args: &AddArgs) -> Result<()> {
    use PackageManager::*;
    use PackageManagerOperation::*;

    let pm = PackageManager::resolve(ctx)?;
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

    pm.run_operation(ctx, op, &resolved_args)
}

fn complete(ctx: &Context, current: &OsStr) -> Option<Vec<CompletionCandidate>> {
    let pkgs = PackageManager::search_npm_registry()
        .ctx(ctx)
        .maybe_query(current.to_str())
        .call()?;

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

    Some(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Unpm;
    use crate::cli::sub::Subcommands;
    use clap::{Parser, error::ErrorKind};

    fn parse_add(args: &[&str]) -> AddArgs {
        let cli = Unpm::try_parse_from(args).expect("failed to parse test cli arguments");
        let Subcommands::Add(args) = cli.command.expect("expected command") else {
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
