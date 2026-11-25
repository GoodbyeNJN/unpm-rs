use crate::{PackageManager, PackageManagerOperation};

use super::Adapter;

impl Adapter {
    pub fn for_bun(op: PackageManagerOperation) -> Option<Self> {
        use PackageManagerOperation::*;

        let new = |a: &[&str]| {
            Adapter::new()
                .set_program(PackageManager::Bun.to_string())
                .set_program_args(a.to_owned())
        };

        let adapter = match op {
            Pm => new(&[]),

            Install => new(&["install"]),
            FrozenInstall => new(&["install", "--frozen-lockfile"]),

            Add => new(&["add"]),
            GlobalAdd => new(&["add", "--global"]),

            Remove => new(&["remove"]),
            GlobalRemove => new(&["remove", "--global"]),

            List => new(&["list"]),
            GlobalList => new(&["pm", "-g", "list"]),

            Update => new(&["update"]),
            GlobalUpdate => new(&["update", "--global"]),
            InteractiveUpdate => new(&["update", "--interactive"]),
            GlobalInteractiveUpdate => new(&["update", "--interactive", "--global"]),

            Dlx => new(&["x"]),
            Exec => new(&["x"]),
            Run => new(&["run"]),
        };

        Some(adapter)
    }
}
