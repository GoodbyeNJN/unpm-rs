use crate::package_manager::CommandAdaptor;
use crate::{PackageManager, PackageManagerOperation};

impl CommandAdaptor {
    pub fn for_yarn(op: PackageManagerOperation) -> Option<Self> {
        use PackageManagerOperation::*;

        let new = |a: &[&str]| {
            CommandAdaptor::new()
                .set_program(PackageManager::Yarn.to_string())
                .set_program_args(a.to_owned())
        };

        let adaptor = match op {
            Pm => new(&[]),

            Install => new(&["install"]),
            FrozenInstall => new(&["install", "--frozen-lockfile"]),

            Add => new(&["add"]),
            GlobalAdd => new(&["global", "add"]),

            Remove => new(&["remove"]),
            GlobalRemove => new(&["global", "remove"]),

            List => new(&["list"]),
            GlobalList => new(&["global", "list"]),

            Update => new(&["upgrade"]),
            GlobalUpdate => new(&["global", "upgrade"]),
            InteractiveUpdate => new(&["upgrade-interactive"]),
            GlobalInteractiveUpdate => new(&["global", "upgrade-interactive"]),

            Dlx => return None,
            Exec => new(&["exec"]).set_separate(true),
            Run => new(&["run"]),
        };

        Some(adaptor)
    }
}
