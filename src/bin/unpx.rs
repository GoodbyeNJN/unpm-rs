use clap::CommandFactory;
use clap_complete::CompleteEnv;
use unpm::{Unpx, unpx};

fn main() {
    CompleteEnv::with_factory(Unpx::command).complete();

    env_logger::init();

    let mut cmd = Unpx::command();
    unpx(&mut cmd).unwrap_or_else(|err| {
        err.exit(&cmd);
    });
}
