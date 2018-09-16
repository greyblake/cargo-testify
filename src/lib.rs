extern crate glob;
extern crate notify;
extern crate notify_rust;
extern crate regex;
#[macro_use]
extern crate structopt;

mod reactor;
mod report;
mod report_builder;

use glob::Pattern;
use reactor::{Config as ReactorConfig, Reactor};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "cargo", bin_name = "cargo")]
enum CargoOpt {
    #[structopt(name = "testify")]
    Testify(Args),
}

#[derive(StructOpt)]
#[structopt(name = "testify")]
struct Args {
    #[structopt(
        long = "include",
        short = "i",
        help = "Only run tests when a file matches the pattern(s)",
        value_name = "PATTERN",
        default_value = "src/**/*.rs,tests/**/*.rs,Cargo.toml,Cargo.lock,build.rs",
        raw(use_delimiter = "true"),
    )]
    patterns: Vec<Pattern>,

    #[structopt(
        long = "delay",
        short = "d",
        help = "Wait at least for provided delay (ms) before rerunning tests",
        value_name = "MILLISECONDS",
        default_value = "300",
        parse(try_from_str = "duration_from_str"),
    )]
    ignore_duration: Duration,

    #[structopt(
        help = "Arguments to pass to `cargo test`",
        value_name = "ARGS",
    )]
    cargo_test_args: Vec<String>,
}

pub fn run() {
    let CargoOpt::Testify(Args {
        ignore_duration,
        patterns,
        cargo_test_args,
    }) = CargoOpt::from_args();

    let project_dir = detect_project_dir();

    let rconfig = ReactorConfig {
        project_dir,
        ignore_duration,
        patterns,
        cargo_test_args,
    };

    Reactor::new(rconfig).start()
}

/// Convert a str containing milliseconds to a duration
fn duration_from_str(s: &str) -> Result<Duration, ::std::num::ParseIntError> {
    s.parse().map(Duration::from_millis)
}

/// Search for Cargo.toml file starting from the current directory,
/// going with every step to parent directory. If directory with
/// Cargo.toml is found return it, otherwise print error message and
/// terminate the process.
fn detect_project_dir() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut optional_dir = Some(current_dir.as_path());

    while let Some(dir) = optional_dir {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.is_file() {
            return dir.to_path_buf();
        }
        optional_dir = dir.parent();
    }

    eprintln!(
        "Error: could not find `Cargo.toml` in {:?} or any parent directory.",
        current_dir
    );
    std::process::exit(1);
}
