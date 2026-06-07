use crate::{
    CommandError, PackageManager, PackageManagerError, PackageManagerOperation, Result,
    ScriptRunner, ScriptRunnerError, ScriptRunnerOperation, bail,
};

mod bun;
mod node;
mod npm;
mod pnpm;
mod yarn;
mod yarn_berry;

#[derive(Clone, Debug, Default)]
pub struct CommandAdaptor {
    program: String,
    program_args: Vec<String>,
    extra_args: Vec<String>,
    separate: bool,
}

impl CommandAdaptor {
    pub fn new() -> Self {
        Self {
            separate: false,
            ..Default::default()
        }
    }

    pub fn for_pm(pm: PackageManager, op: PackageManagerOperation) -> Result<Self> {
        use PackageManager::*;

        match pm {
            Npm => Self::for_npm(op),
            Pnpm => Self::for_pnpm(op),
            Yarn => Self::for_yarn(op),
            YarnBerry => Self::for_yarn_berry(op),
            Bun => Self::for_bun(op),
        }
        .ok_or_else(|| {
            PackageManagerError::Unsupported {
                package_manager: pm,
                operation: op,
                option: None,
            }
            .into()
        })
    }

    pub fn for_runner(runner: ScriptRunner, op: ScriptRunnerOperation) -> Result<Self> {
        use ScriptRunner::*;

        match runner {
            Node => Self::for_node(op),
        }
        .ok_or_else(|| {
            ScriptRunnerError::Unsupported {
                script_runner: ScriptRunner::Node,
                operation: ScriptRunnerOperation::Run,
                option: None,
            }
            .into()
        })
    }

    pub fn set_program<S>(mut self, v: S) -> Self
    where
        S: Into<String>,
    {
        self.program = v.into();

        self
    }

    pub fn set_program_args<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.program_args = v.into_iter().map(Into::into).collect();

        self
    }

    pub fn set_extra_args<T, S>(mut self, v: T) -> Self
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.extra_args = v.into_iter().map(Into::into).collect();

        self
    }

    pub fn set_separate(mut self, v: bool) -> Self {
        self.separate = v;

        self
    }

    pub fn run(&self, dry: bool) -> Result<()> {
        if dry {
            color_print::cprintln!("<green>[dry-run]</> {}", self);

            return Ok(());
        }

        let status = self
            .to_command()
            .status()
            .map_err(|source| CommandError::Spawn {
                command: self.to_string(),
                source,
            })?;
        self.check_status(&status)?;

        Ok(())
    }

    pub fn silent_run(&self) -> Result<(std::process::ExitStatus, String, String)> {
        let output = self
            .to_command()
            .output()
            .map_err(|source| CommandError::Spawn {
                command: self.to_string(),
                source,
            })?;
        let status = output.status;
        self.check_status(&status)?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok((status, stdout, stderr))
    }

    fn check_status(&self, status: &std::process::ExitStatus) -> Result<()> {
        if !status.success() {
            bail!(CommandError::Exit {
                command: self.to_string(),
                code: status
                    .code()
                    .map(|code| code.to_string())
                    .unwrap_or("unknown".to_owned()),
            });
        }

        Ok(())
    }

    fn to_command(&self) -> std::process::Command {
        let mut cmd = std::process::Command::new(&self.program);
        cmd.args(self.combine_args());

        cmd
    }

    fn combine_args(&self) -> Vec<String> {
        let mut args = self.program_args.to_owned();
        let mut iter = self.extra_args.iter().cloned();

        if self.separate {
            if let Some(first) = iter.next() {
                args.push(first);
            }

            if let Some(second) = iter.next() {
                args.push("--".to_owned());
                args.push(second);

                args.extend(iter);
            }
        } else {
            args.extend(iter);
        }

        args
    }
}

impl std::fmt::Display for CommandAdaptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = if self.program_args.is_empty() {
            self.program.to_owned()
        } else {
            format!("{} {}", self.program, self.combine_args().join(" "))
        };

        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::CommandAdaptor;
    use super::PackageManager::*;
    use super::PackageManagerOperation::*;

    #[test]
    fn add_operation_fixtures() {
        let args = ["typescript".to_owned(), "-D".to_owned()];

        let cases = [
            (Npm, Some(("npm", vec!["add", "typescript", "-D"]))),
            (Pnpm, Some(("pnpm", vec!["add", "typescript", "-D"]))),
            (Yarn, Some(("yarn", vec!["add", "typescript", "-D"]))),
            (YarnBerry, Some(("yarn", vec!["add", "typescript", "-D"]))),
            (Bun, Some(("bun", vec!["add", "typescript", "-D"]))),
        ];

        for (manager, expected) in cases {
            let actual =
                CommandAdaptor::for_pm(manager, Add).map(|adaptor| adaptor.set_extra_args(&args));
            let actual = actual.expect("expected Some for supported operation");
            match expected {
                Some((program, args)) => {
                    assert_eq!(actual.program, program, "add fixture for {manager:?}");
                    assert_eq!(actual.combine_args(), args, "add fixture for {manager:?}");
                }
                None => {
                    panic!("expected Some for supported operation for {manager:?}");
                }
            }
        }
    }

    #[test]
    fn run_operation_fixtures() {
        let args = ["arg0".to_owned(), "arg1-0 arg1-1".to_owned()];

        let cases = [
            (
                Npm,
                Some(("npm", vec!["run", "arg0", "--", "arg1-0 arg1-1"])),
            ),
            (Pnpm, Some(("pnpm", vec!["run", "arg0", "arg1-0 arg1-1"]))),
            (Yarn, Some(("yarn", vec!["run", "arg0", "arg1-0 arg1-1"]))),
            (
                YarnBerry,
                Some(("yarn", vec!["run", "arg0", "arg1-0 arg1-1"])),
            ),
            (Bun, Some(("bun", vec!["run", "arg0", "arg1-0 arg1-1"]))),
        ];

        for (manager, expected) in cases {
            let actual =
                CommandAdaptor::for_pm(manager, Run).map(|adaptor| adaptor.set_extra_args(&args));
            match expected {
                Some((program, args)) => {
                    let actual = actual.expect("expected Some for supported operation");
                    assert_eq!(actual.program, program, "run fixture for {manager:?}");
                    assert_eq!(actual.combine_args(), args, "run fixture for {manager:?}");
                }
                None => {
                    panic!("expected Some for supported operation for {manager:?}");
                }
            }
        }
    }

    #[test]
    fn dlx_operation_fixtures() {
        let args = ["eslint".to_owned(), "--fix".to_owned()];

        let cases = [
            (Npm, Some(("npx", vec!["eslint", "--fix"]))),
            (Pnpm, Some(("pnpm", vec!["dlx", "eslint", "--fix"]))),
            (Yarn, None),
            (YarnBerry, Some(("yarn", vec!["dlx", "eslint", "--fix"]))),
            (Bun, Some(("bun", vec!["x", "eslint", "--fix"]))),
        ];

        for (manager, expected) in cases {
            let actual =
                CommandAdaptor::for_pm(manager, Dlx).map(|adaptor| adaptor.set_extra_args(&args));
            match expected {
                Some((program, args)) => {
                    let actual = actual.expect("expected Some for supported operation");
                    assert_eq!(actual.program, program, "dlx fixture for {manager:?}");
                    assert_eq!(actual.combine_args(), args, "dlx fixture for {manager:?}");
                }
                None => {
                    panic!("expected Some for supported operation for {manager:?}");
                }
            }
        }
    }

    #[test]
    fn returns_none_for_unsupported_operations() {
        let unsupported = [
            (Npm, InteractiveUpdate),
            (Npm, GlobalInteractiveUpdate),
            (Yarn, Dlx),
            (YarnBerry, GlobalAdd),
            (YarnBerry, GlobalRemove),
            (YarnBerry, GlobalList),
            (YarnBerry, GlobalUpdate),
            (YarnBerry, GlobalInteractiveUpdate),
        ];

        for (manager, operation) in unsupported {
            let actual = CommandAdaptor::for_pm(manager, operation)
                .map(|adaptor| adaptor.set_extra_args(&["foo".to_owned()]));
            assert!(
                actual.is_err(),
                "expected None for unsupported mapping: {manager:?} {operation:?}"
            );
        }
    }
}
