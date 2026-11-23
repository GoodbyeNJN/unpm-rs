use clap::CommandFactory;
use clap_complete::CompleteEnv;
use unpm::{Unpx, unpx};

fn main() {
    env_logger::init();
    CompleteEnv::with_factory(Unpx::command).complete();

    let mut cmd = Unpx::command();
    unpx(&mut cmd).unwrap_or_else(|err| {
        err.exit(&cmd);
    });
}
