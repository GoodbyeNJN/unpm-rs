use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(
        "failed to execute command `{}`: {source}",
        color_print::cformat!("<yellow>{}</>", .command)
    )]
    Spawn {
        command: String,
        source: std::io::Error,
    },

    #[error(
        "command `{}` exited with status {}",
        color_print::cformat!("<yellow>{}</>", .command),
        color_print::cformat!("<red>{}</>", .code)
    )]
    Exit { command: String, code: String },
}
