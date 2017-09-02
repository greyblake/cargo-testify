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
        let mut args = vec![
            report.title(),
            get_icon_arg(report.outcome)
        ];
        match report.detail.as_ref() {
            Some(msg) => args.push(msg),
            None => ()
        };
        Command::new("notify-send")
            .args(args)
            .output()
            .expect("failed to execute `notify-send` shell command");
    }
}

fn get_icon_arg(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::TestsPassed => "--icon=face-angel",
        Outcome::TestsFailed | Outcome::CompileError => "--icon=face-angry"
    }
}
