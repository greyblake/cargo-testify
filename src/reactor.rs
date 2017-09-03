use notify::{RecommendedWatcher, Watcher, Event};
use notify_rust::Notification;

use std::process::{Command, Stdio};
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::process;
use std::sync::mpsc::channel;

use config::Config;
use report_builder::ReportBuilder;
use report::Outcome;

pub struct Reactor {
    config: Config,
    last_run_at: Instant,
    report_builder: ReportBuilder
}

impl Reactor {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            last_run_at: Instant::now(),
            report_builder: ReportBuilder::new()
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
                Ok(event) => {
                    if self.should_react(event) {
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

    fn should_react(&self, event: Event) -> bool {
        // ignore event if tests just finished very recently
        if Instant::now() - self.last_run_at < self.config.ignore_duration {
            return false;
        }

        match event.path {
            Some(path) => {
                // React if the changed file is not withing project/target directory
                !path.starts_with(self.config.target_dir.as_path())
            },
            None => false
        }
    }

    /// Spawn `cargo test` and catch stdout and stderr, then build report and call notifier.
    /// TODO: Number of things can and have to be improved here:
    ///   * Preserve color output of `cargo test`
    ///   * Is it possible intercept stdout and stderr in one thread using futures?
    fn run_tests(&self) {
        let result = Command::new("cargo")
            .args(&["test"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match result {
            Ok(mut child) => {
                // Catch stdout
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

                // Catch stderr
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

                let exit_status = child.wait().expect("failed to wait for child process `cargo test`");
                let stdout_output = stdout_buffer.lock().unwrap().clone();
                let stderr_output = stderr_buffer.lock().unwrap().clone();

                let report = self.report_builder.identify(exit_status.success(), &stdout_output, &stderr_output);
                let icon = match report.outcome {
                    Outcome::TestsPassed => "face-angel",
                    Outcome::TestsFailed | Outcome::CompileError => "face-angry"
                };
                let mut notification = Notification::new()
                    .summary(report.title())
                    .icon(icon)
                    .finalize();
                if let Some(detail) = report.detail {
                    notification.body(&detail);
                }
                notification
                    .show()
                    .expect("unable to send notification");
            }
            Err(err) => {
                eprintln!("Failed to spawn `cargo test`");
                eprintln!("{:?}", err);
                process::exit(1);
            }
        }
    }
}
