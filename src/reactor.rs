use regex::Regex;
use notify::{RecommendedWatcher, Watcher};

use std::process::{Command, Stdio};
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::process;
use std::sync::mpsc::channel;

use config::Config;
use outcome::Outcome;
use notifiers::{Notify, NotifySend};
use outcome_identifier::OutcomeIdentifier;

pub struct Reactor {
    config: Config,
    last_run_at: Instant,
    outcome_identifier: OutcomeIdentifier
}

impl Reactor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            last_run_at: Instant::now(),
            outcome_identifier: OutcomeIdentifier::new()
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx).expect("Failed to obtain a watcher");
        watcher.watch(&self.config.project_dir).expect("Failed to start watcher");

        self.last_run_at = Instant::now();
        self.run_tests();

        loop {
            match rx.recv() {
                Ok(_event) => {
                    if Instant::now() - self.last_run_at > self.config.ignore_duration {
                        self.run_tests();
                        self.last_run_at = Instant::now();
                    }
                },
                Err(err) => {
                    eprintln!("Unexpected error occurred:");
                    eprintln!("  {:?}", err);
                    process::exit(1);
                }
            }
        }
    }

    pub fn run_tests(&self) {
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
                thread::spawn(move || {
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
                thread::spawn(move || {
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

                let outcome = self.outcome_identifier.identify(exit_status.success(), &stdout_output, &stderr_output);
                self.config.notifier.notify(outcome);
            }
            Err(err) => {
                eprintln!("Failed to spawn `cargo test`");
                eprintln!("{:?}", err);
                process::exit(1);
            }
        }
    }
}
