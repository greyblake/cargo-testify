extern crate notify;
extern crate regex;

use notify::{RecommendedWatcher, Watcher};
use regex::Regex;


use std::sync::mpsc::channel;
use std::time::{Instant, Duration};
use std::process::{Command, Stdio};
use std::io::Read;

enum Outcome<'a> {
    TestsPassed(&'a str),
    TestsFailed(&'a str),
    CompileError(&'a str)
}


fn main() {
    let project_dir = std::env::current_dir().expect("Failed to get current directory");
    let src_dir = project_dir.join("src");

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx).expect("Failed to obtain a watcher");

    watcher.watch(src_dir).expect("Failed to start watcher");

    let ignore_duration = Duration::from_millis(300);

    let mut last_run_at = Instant::now();
    run_tests();

    loop {
        match rx.recv() {
            Ok(_event) => {
                if Instant::now() - last_run_at > ignore_duration {
                    run_tests();
                    last_run_at = Instant::now();
                }
            },
            Err(err) => {
                println!("Unexpected error occured:");
                println!("  {:?}", err);
                std::process::exit(1);
            }
        }
    }
}

fn run_tests() {
    let result = Command::new("cargo")
        .args(&["test"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match result {
        Ok(mut child) => {
            let exit_status = child.wait().expect("failed to wait on child");

            let mut stdout = String::new();
            child.stdout.unwrap().read_to_string(&mut stdout);

            let mut stderr = String::new();
            child.stderr.unwrap().read_to_string(&mut stderr);

            eprint!("{}", stderr);
            print!("{}", stdout);

            let outcome = detect_outcome(exit_status.success(), &stdout, &stderr);
            notify(outcome);
        },
        Err(err) => {
            println!("Failed to spawn `cargo test`");
            println!("{:?}", err);
            std::process::exit(1);
        }
    }
}

fn notify(outcome: Outcome) {
    let args = match outcome {
        Outcome::TestsPassed(msg) => vec!["Test passed", msg, "--icon=face-angel"],
        Outcome::TestsFailed(msg) => vec!["Test failed", msg, "--icon=face-angry"],
        Outcome::CompileError(msg) => vec!["Error", msg, "--icon=face-angry"]
    };
    Command::new("notify-send")
        .args(args)
        .output()
        .expect("failed to execute `notify-send` shell command");
}

fn detect_outcome<'a>(process_success: bool, stdout: &'a str, stderr: &'a str) -> Outcome<'a> {
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


#[cfg(test)]
mod tests {
    #[test]
    fn test_fail() {
        assert_eq!(1, 1);
    }
}
