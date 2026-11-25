use std::process;

use clap::CommandFactory;
use clap_complete::CompleteEnv;
use color_print::cprintln;
use unpm::{Unpm, unpm};

fn main() {
    env_logger::init();
    CompleteEnv::with_factory(Unpm::command).complete();

    let mut cmd = Unpm::command();
    if let Err(err) = unpm(&mut cmd) {
        cprintln!("<red,bold>error:</> {}", err);

        let cause = err.chain().skip(1).next();
        if let Some(cause) = cause {
            cprintln!("  <red,bold>cause:</> {cause}");
        }

        process::exit(1);
    }
}
