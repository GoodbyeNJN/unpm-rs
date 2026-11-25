use std::process::{Command, ExitStatus};

use anyhow::{Context, Result, bail};
use color_print::{cformat, cprintln};

pub fn spawn_command<C: Into<Command>>(command: C, dry_run: bool) -> Result<()> {
    let mut command = command.into();

    if dry_run {
        cprintln!("<green>[dry-run]</> {}", format_command(&command));

        return Ok(());
    }

    let status = command
        .status()
        .with_context(|| spawn_failed_message(&command))?;
    check_status(&command, &status)?;

    Ok(())
}

pub fn spawn_command_silent<C: Into<Command>>(command: C) -> Result<(ExitStatus, String, String)> {
    let mut command = command.into();

    let output = command
        .output()
        .with_context(|| spawn_failed_message(&command))?;
    let status = output.status;
    check_status(&command, &status)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok((status, stdout, stderr))
}

fn spawn_failed_message(command: &Command) -> String {
    cformat!(
        "failed to spawn command: `<yellow>{}</>`",
        format_command(&command)
    )
}

fn check_status(command: &Command, status: &ExitStatus) -> Result<()> {
    if !status.success() {
        let code = status
            .code()
            .map(|code| code.to_string())
            .unwrap_or("unknown".to_owned());
        bail!(cformat!(
            "command `<yellow>{}</>` exited with status code <red>{code}</>",
            format_command(&command)
        ));
    }

    Ok(())
}

fn format_command(command: &Command) -> String {
    let mut str = command.get_program().to_string_lossy().to_string();
    for arg in command.get_args() {
        str.push(' ');
        str.push_str(&arg.to_string_lossy());
    }

    str
}
