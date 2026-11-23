use std::ffi::OsStr;

use clap_complete::ArgValueCompleter;
use clap_complete::CompletionCandidate;
use log::{debug, error};

use crate::cli::Unpm;
use crate::{Context, Result};

pub fn wrap_completer<F>(completer: F) -> ArgValueCompleter
where
    F: Fn(&Context, &OsStr) -> Result<Vec<CompletionCandidate>> + Send + Sync + 'static,
{
    ArgValueCompleter::new(move |current: &OsStr| {
        let ctx = (|| -> Result<_> {
            let cli = Unpm::try_parse_for_complete()?;
            let mut ctx = Context::new(&cli.main_args)?;
            cli.extends_ctx(&mut ctx);
            ctx.interactive = false;

            Ok(ctx)
        })();

        match ctx {
            Ok(ctx) => match completer(&ctx, current) {
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
            },
            Err(err) => {
                error!("failed to initialize completion context: {err:?}");
                vec![]
            }
        }
    })
}
