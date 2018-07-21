#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate aitios_asset;
extern crate aitios_geom;
extern crate aitios_scene;

mod app;
mod err;
mod filter;
mod launcher;
mod litter;

use err::summarize_error;
use failure::Error;
use launcher::Launcher;
use std::process::exit;

fn main() {
    if let Err(err) = Launcher::launch() {
        exit_with_error(err);
    }
}

/// Prints error messages and exits with a non-zero exit code.
fn exit_with_error(error: Error) -> ! {
    fail_for_humans(error);
    exit(1)
}

fn fail_for_humans(error: Error) {
    eprintln!("{}", summarize_error(error));
}
