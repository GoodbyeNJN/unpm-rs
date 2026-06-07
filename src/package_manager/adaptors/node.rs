use crate::package_manager::CommandAdaptor;
use crate::{ScriptRunner, ScriptRunnerOperation};

impl CommandAdaptor {
    pub fn for_node(op: ScriptRunnerOperation) -> Option<Self> {
        use ScriptRunnerOperation::*;

        let new = |a: &[&str]| {
            CommandAdaptor::new()
                .set_program(ScriptRunner::Node.to_string())
                .set_program_args(a.to_owned())
        };

        let adaptor = match op {
            Run => new(&["--run"]).set_separate(true),
        };

        Some(adaptor)
    }
}
