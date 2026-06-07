use crate::package_manager::CommandAdaptor;
use crate::{PackageManager, PackageManagerOperation};

impl CommandAdaptor {
    pub fn for_npm(op: PackageManagerOperation) -> Option<Self> {
        use PackageManagerOperation::*;

        let new = |a: &[&str]| {
            CommandAdaptor::new()
                .set_program(PackageManager::Npm.to_string())
                .set_program_args(a.to_owned())
        };

        let adaptor = match op {
            Pm => new(&[]),

            Install => new(&["install"]),
            FrozenInstall => new(&["ci"]),

            Add => new(&["add"]),
            GlobalAdd => new(&["add", "--global"]),

            Remove => new(&["remove"]),
            GlobalRemove => new(&["remove", "--global"]),

            List => new(&["list"]),
            GlobalList => new(&["list", "--global"]),

            Update => new(&["update"]),
            GlobalUpdate => new(&["update", "--global"]),
            InteractiveUpdate => return None,
            GlobalInteractiveUpdate => return None,

            Dlx => new(&[]).set_program("npx"),
            Exec => new(&["exec"]),
            Run => new(&["run"]).set_separate(true),
        };

        Some(adaptor)
    }
}
