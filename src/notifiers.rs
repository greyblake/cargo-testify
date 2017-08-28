use outcome::Outcome;
use std::process::Command;

pub trait Notify {
    fn notify(&self, outcome: Outcome);
}

pub struct NotifySend;

impl NotifySend {
    pub fn new() -> Self {
        Self { }
    }
}

impl Notify for NotifySend {
    fn notify(&self, outcome: Outcome) {
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
}
