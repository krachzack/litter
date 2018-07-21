#[macro_use]
extern crate clap;
extern crate failure;

mod app;
mod err;
mod launcher;

use err::summarize_error;
use failure::Error;
use launcher::Launcher;
use std::process::exit;

fn main() {
    if let Err(err) = Launcher::run() {
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
