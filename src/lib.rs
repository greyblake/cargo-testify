extern crate clap;
extern crate glob;
extern crate notify;
extern crate notify_rust;
extern crate regex;
#[macro_use]
extern crate error_chain;

use clap::{App, Arg, SubCommand};

mod config;
mod errors;
mod reactor;
mod report;
mod report_builder;
use config::ConfigBuilder;
use reactor::Reactor;

pub fn run() {
    let matches = App::new("cargo")
        .bin_name("cargo")
        .help_message("")
        .version_message("")
        .subcommand(
            SubCommand::with_name("testify")
            .version("0.2.0")
            .author("Sergey Potapov <blake131313@gmail.com>")
            .about("Automatically runs tests for Rust project and notifies about the result.\nSource code: https://github.com/greyblake/cargo-testify")
            .arg(Arg::with_name("includes")
                 .short("i")
                 .long("include")
                 .takes_value(true)
                 .help("Comma separated list of include pattern in addition to the predefined default patterns"))
            .arg(Arg::with_name("cargo_test_args")
                 .multiple(true)
                 .last(true))
        )
        .get_matches();

    let cargo_test_args = if let Some(matches) = matches.subcommand_matches("testify") {
        matches
            .values_of("cargo_test_args")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or(vec![])
    } else {
        vec![]
    };

    let include_patterns = matches
        .subcommand_matches("testify")
        .and_then(|m| m.value_of("includes"))
        .map(|vals| vals.split(',').collect::<Vec<_>>())
        .unwrap_or(vec![]);

    let project_dir = detect_project_dir();
    let config = ConfigBuilder::new()
        .project_dir(project_dir)
        .include_patterns(&include_patterns)
        .cargo_test_args(cargo_test_args)
        .build()
        .unwrap();

    Reactor::new(config).start()
}

/// Search for Cargo.toml file starting from the current directory,
/// going with every step to parent directory. If directory with
/// Cargo.toml is found return it, otherwise print error message and
/// terminate the process.
fn detect_project_dir() -> std::path::PathBuf {
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
