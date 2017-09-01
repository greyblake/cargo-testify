use notifiers::Notify;
use report::{Outcome, Report};
use std::process::Command;

/// Shows notification messages using `notify-send` command.
/// To run this command in Linux, one requires to install `libnotify-bin` system package.
pub struct NotifySend;

impl NotifySend {
    pub fn new() -> Self {
        Self { }
    }
}

impl Notify for NotifySend {
    fn notify(&self, report: Report) {
        let detail = report.detail.as_ref();
        let mut args = match report.outcome {
            Outcome::TestsPassed => vec!["Test passed", "--icon=face-angel"],
            Outcome::TestsFailed => vec!["Test failed", "--icon=face-angry"],
            Outcome::CompileError => vec!["Error", "--icon=face-angry"]
        };
        match detail {
            Some(msg) => args.push(msg),
            None => ()
        };
        Command::new("notify-send")
            .args(args)
            .output()
            .expect("failed to execute `notify-send` shell command");
    }
}
