extern crate notify;
extern crate regex;
#[macro_use] extern crate error_chain;

use notify::{RecommendedWatcher, Watcher};
use regex::Regex;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::Instant;

mod errors;

mod outcome;
use outcome::Outcome;

mod notifiers;
use notifiers::{Notify, NotifySend};

mod config;
use config::{Config, ConfigBuilder};

//mod reactor;

// TODO: implement filter for files like .git, /target, etc..
pub fn run() {
    let project_dir = detect_project_dir();
    let notifier = obtain_notifier();

    let config = ConfigBuilder::new()
        .project_dir(project_dir)
        .notifier(notifier)
        .build()
        .unwrap();

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx).expect("Failed to obtain a watcher");

    watcher.watch(config.project_dir).expect("Failed to start watcher");

    let mut last_run_at = Instant::now();
    run_tests();

    loop {
        match rx.recv() {
            Ok(_event) => {
                if Instant::now() - last_run_at > config.ignore_duration {
                    run_tests();
                    last_run_at = Instant::now();
                }
            },
            Err(err) => {
                eprintln!("Unexpected error occurred:");
                eprintln!("  {:?}", err);
                std::process::exit(1);
            }
        }
    }
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
        if cargo_toml.is_file() { return dir.to_path_buf(); }
        optional_dir = dir.parent();
    }

    eprintln!("Error: could not find `Cargo.toml` in {:?} or any parent directory.", current_dir);
    std::process::exit(1);
}

pub fn run_tests() {
    let result = Command::new("cargo")
        .args(&["test"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match result {
        Ok(mut child) => {
            let stdout = child.stdout.take().unwrap();
            let stdout_buf_reader = BufReader::new(stdout);
            let stdout_buffer = Arc::new(Mutex::new(String::new()));
            let stdout_buffer_clone = stdout_buffer.clone();
            std::thread::spawn(move || {
                for line in stdout_buf_reader.lines() {
                    let line = line.unwrap();
                    let mut buffer = stdout_buffer_clone.lock().unwrap();
                    buffer.push_str(&line);
                    buffer.push('\n');
                    println!("{}", line);
                }
            });

            let stderr = child.stderr.take().unwrap();
            let stderr_buf_reader = BufReader::new(stderr);
            let stderr_buffer = Arc::new(Mutex::new(String::new()));
            let stderr_buffer_clone = stderr_buffer.clone();
            std::thread::spawn(move || {
                for line in stderr_buf_reader.lines() {
                    let line = line.unwrap();
                    let mut buffer = stderr_buffer_clone.lock().unwrap();
                    buffer.push_str(&line);
                    buffer.push('\n');
                    eprintln!("{}", line);
                }
            });

            let exit_status = child.wait().expect("failed to wait on child");
            let stdout_output = stdout_buffer.lock().unwrap().clone();
            let stderr_output = stderr_buffer.lock().unwrap().clone();

            let outcome = detect_outcome(exit_status.success(), &stdout_output, &stderr_output);
            let notifier = obtain_notifier();
            notifier.notify(outcome);
        }
        Err(err) => {
            println!("Failed to spawn `cargo test`");
            println!("{:?}", err);
            std::process::exit(1);
        }
    }
}

pub fn detect_outcome<'a>(process_success: bool, stdout: &'a str, stderr: &'a str) -> Outcome<'a> {
    let result_re = Regex::new(r"\d{1,} passed.*filtered out").unwrap();
    let error_re = Regex::new(r"error(:|\[).*").unwrap();

    if process_success {
        let message = result_re.find(stdout).unwrap().as_str();
        Outcome::TestsPassed(message)
    } else {
        match result_re.find(stdout) {
            Some(matched) => {
                Outcome::TestsFailed(matched.as_str())
            },
            None => {
                let message = error_re.find(stderr).unwrap().as_str();
                Outcome::CompileError(message)
            }
        }
    }
}

fn obtain_notifier() -> Box<Notify> {
    Box::new(NotifySend::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fail() {
        assert_eq!(1, 1);
    }
}
