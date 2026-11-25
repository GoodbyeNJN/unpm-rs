use crate::{ScriptRunner, ScriptRunnerOperation};

use super::Adapter;

impl Adapter {
    pub fn for_node(op: ScriptRunnerOperation) -> Option<Self> {
        use ScriptRunnerOperation::*;

        let new = |a: &[&str]| {
            Adapter::new()
                .set_program(ScriptRunner::Node.to_string())
                .set_program_args(a.to_owned())
        };

        let adapter = match op {
            Run => new(&["--run"]).set_separate(true),
        };

        Some(adapter)
    }
}
