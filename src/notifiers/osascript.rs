use notifiers::Notify;
use report::Report;
use std::process::Command;

pub struct Osascript;

impl Osascript {
    pub fn new() -> Self {
        Self { }
    }
}

impl Notify for Osascript {
    fn notify(&self, report: Report) {
        let title = report.title();
        let text = report.detail.unwrap_or("".to_owned());

        let cmd = format!("'display notification \"{}\" with title \"{}\"'", text, title);
        let args = vec!["-e", &cmd];

        Command::new("osascript")
            .args(args)
            .output()
            .expect("failed to execute `osascript` shell command");
    }
}
