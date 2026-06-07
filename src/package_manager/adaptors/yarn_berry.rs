use crate::package_manager::CommandAdaptor;
use crate::{PackageManager, PackageManagerOperation};

impl CommandAdaptor {
    pub fn for_yarn_berry(op: PackageManagerOperation) -> Option<Self> {
        use PackageManagerOperation::*;

        let new = |a: &[&str]| {
            CommandAdaptor::new()
                .set_program(PackageManager::Yarn.to_string())
                .set_program_args(a.to_owned())
        };

        let adaptor = match op {
            Pm => new(&[]),

            Install => new(&["install"]),
            FrozenInstall => new(&["install", "--immutable"]),

            Add => new(&["add"]),
            GlobalAdd => return None,

            Remove => new(&["remove"]),
            GlobalRemove => return None,

            List => new(&["info", "--name-only"]),
            GlobalList => return None,

            Update => new(&["up"]),
            GlobalUpdate => return None,
            InteractiveUpdate => new(&["upgrade-interactive"]),
            GlobalInteractiveUpdate => return None,

            Dlx => new(&["dlx"]),
            Exec => new(&["exec"]),
            Run => new(&["run"]),
        };

        Some(adaptor)
    }
}
