use clap::CommandFactory;
use clap_complete::CompleteEnv;
use unpm::{Unpm, unpm};

fn main() {
    CompleteEnv::with_factory(Unpm::command).complete();

    env_logger::init();

    let mut cmd = Unpm::command();
    unpm(&mut cmd).unwrap_or_else(|err| {
        err.exit(&cmd);
    });
}
