use clap::CommandFactory;
use clap_complete::CompleteEnv;
use unpm::{Unpm, unpm};

fn main() {
    env_logger::init();
    CompleteEnv::with_factory(Unpm::command).complete();

    let mut cmd = Unpm::command();
    unpm(&mut cmd).unwrap_or_else(|err| {
        err.exit(&cmd);
    });
}
