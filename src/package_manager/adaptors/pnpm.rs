use crate::{PackageManager, PackageManagerOperation};

use super::Adapter;

impl Adapter {
    pub fn for_pnpm(op: PackageManagerOperation) -> Option<Self> {
        use PackageManagerOperation::*;

        let new = |a: &[&str]| {
            Adapter::new()
                .set_program(PackageManager::Pnpm.to_string())
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
            GlobalList => new(&["list", "--global"]),

            Update => new(&["update"]),
            GlobalUpdate => new(&["update", "--global"]),
            InteractiveUpdate => new(&["update", "--interactive"]),
            GlobalInteractiveUpdate => new(&["update", "--interactive", "--global"]),

            Dlx => new(&["dlx"]),
            Exec => new(&["exec"]),
            Run => new(&["run"]),
        };

        Some(adapter)
    }

    pub fn for_pnpm_global_list_json() -> Self {
        Adapter::new()
            .set_program(PackageManager::Pnpm.to_string())
            .set_program_args(vec!["list", "--global", "--json"])
    }
}
