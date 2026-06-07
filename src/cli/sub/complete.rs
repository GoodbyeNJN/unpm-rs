use clap::{Args, CommandFactory};
use clap_complete::shells::Shell;

use crate::Context;
use crate::cli::main::{Unpm, Unpx};

/// Generate shell completions
#[derive(Clone, Debug, Args)]
#[command(
    after_help = color_print::cstr!("\
<strong,underline>Examples:</>
  <dim>$</> unpm complete bash >> ~/.bashrc                                <dim># For Bash</>
  <dim>$</> unpm complete elvish >> ~/.elvish/rc.elv                       <dim># For Elvish</>
  <dim>$</> unpm complete fish >> ~/.config/fish/completions/unpm.fish     <dim># For Fish</>
  <dim>$</> unpm complete powershell >> $PROFILE                           <dim># For PowerShell</>
  <dim>$</> unpm complete zsh >> ~/.zshrc                                  <dim># For Zsh</>
  <dim>$</> unpm complete zsh --unpm my-unpm --unpx my-unpx >> ~/.zshrc    <dim># With custom binary names</>
\
"),
)]
pub struct CompleteArgs {
    /// The shell to generate completions for
    pub shell: Shell,

    /// Override generated binary name for unpm
    #[arg(long, value_name = "NAME")]
    pub unpm: Option<String>,

    /// Override generated binary name for unpx
    #[arg(long, value_name = "NAME")]
    pub unpx: Option<String>,
}

impl Default for CompleteArgs {
    fn default() -> Self {
        Self {
            shell: Shell::Bash,
            unpm: None,
            unpx: None,
        }
    }
}

pub fn complete(_: &Context, args: &CompleteArgs) -> String {
    use Shell::*;

    let unpm_default = Unpm::command().get_name().to_owned();
    let unpx_default = Unpx::command().get_name().to_owned();

    let unpm = args
        .unpm
        .to_owned()
        .or_else(|| std::env::args().next())
        .unwrap_or_else(|| unpm_default.to_owned());
    let unpx = args
        .unpx
        .to_owned()
        .or_else(|| {
            unpm.strip_suffix(&unpm_default)
                .map(|prefix| format!("{prefix}{}", unpx_default))
        })
        .unwrap_or_else(|| unpx_default.to_owned());

    match args.shell {
        Bash => {
            format!(
                "source <(COMPLETE={} {});\nsource <(COMPLETE={} {});",
                args.shell, unpm, args.shell, unpx
            )
        }
        Elvish => {
            format!(
                "eval (E:COMPLETE={} {} | slurp);\neval (E:COMPLETE={} {} | slurp);",
                args.shell, unpm, args.shell, unpx
            )
        }
        Fish => {
            format!(
                "COMPLETE={} {} | source;\nCOMPLETE={} {} | source;",
                args.shell, unpm, args.shell, unpx
            )
        }
        PowerShell => {
            format!(
                "$env:COMPLETE = \"{}\"; {} | Out-String | Invoke-Expression;\n$env:COMPLETE = \"{}\"; {} | Out-String | Invoke-Expression;\nRemove-Item Env:\\COMPLETE;",
                args.shell, unpm, args.shell, unpx
            )
        }
        Zsh => {
            format!(
                "source <(COMPLETE={} {});\nsource <(COMPLETE={} {});",
                args.shell, unpm, args.shell, unpx
            )
        }
        _ => {
            unreachable!("unsupported shell");
        }
    }
}
