pub mod bun;
pub mod node;
pub mod npm;
pub mod pnpm;
pub mod yarn;
pub mod yarn_berry;

use std::process::Command;

use anyhow::{Result, anyhow};

use super::{PackageManager, PackageManagerOperation, ScriptRunner, ScriptRunnerOperation, error};

#[derive(Clone, Debug, Default)]
pub struct Adapter {
    program: String,
    program_args: Vec<String>,
    extra_args: Vec<String>,
    separate: bool,
}

impl Adapter {
    pub fn new() -> Self {
        Self {
            separate: false,
            ..Default::default()
        }
    }

    pub fn for_package_manager(pm: PackageManager, op: PackageManagerOperation) -> Result<Self> {
        use PackageManager::*;

        match pm {
            Npm => Self::for_npm(op),
            Pnpm => Self::for_pnpm(op),
            Yarn => Self::for_yarn(op),
            YarnBerry => Self::for_yarn_berry(op),
            Bun => Self::for_bun(op),
        }
        .ok_or_else(|| {
            anyhow!(error::unsupported_package_manager_operation(
                &pm, &op, &None
            ))
        })
    }

    pub fn for_script_runner(runner: ScriptRunner, op: ScriptRunnerOperation) -> Result<Self> {
        use ScriptRunner::*;

        match runner {
            Node => Self::for_node(op),
        }
        .ok_or_else(|| {
            anyhow!(error::unsupported_script_runner_operation(
                &runner, &op, &None
            ))
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

impl From<Adapter> for Command {
    fn from(adapter: Adapter) -> Self {
        let mut cmd = Command::new(&adapter.program);
        cmd.args(adapter.combine_args());

        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::Adapter;
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
            let actual = Adapter::for_package_manager(manager, Add)
                .map(|adapter| adapter.set_extra_args(&args));
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
            let actual = Adapter::for_package_manager(manager, Run)
                .map(|adapter| adapter.set_extra_args(&args));
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
            let actual = Adapter::for_package_manager(manager, Dlx)
                .map(|adapter| adapter.set_extra_args(&args));
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
            let actual = Adapter::for_package_manager(manager, operation)
                .map(|adapter| adapter.set_extra_args(&["foo".to_owned()]));
            assert!(
                actual.is_err(),
                "expected None for unsupported mapping: {manager:?} {operation:?}"
            );
        }
    }
}
