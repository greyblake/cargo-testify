use notifiers::Notify;
use report::{Outcome, Report};
use std::process::Command;

pub struct Osascript;

impl Osascript {
    pub fn new() -> Self {
        Self { }
    }
}

impl Notify for Osascript {
    fn notify(&self, report: Report) {
        unimplemented!()
    //    let detail = report.detail.as_ref();
    //    let mut args = match report.outcome {
    //        Outcome::TestsPassed => vec!["Test passed", "--icon=face-angel"],
    //        Outcome::TestsFailed => vec!["Test failed", "--icon=face-angry"],
    //        Outcome::CompileError => vec!["Error", "--icon=face-angry"]
    //    };
    //    match detail {
    //        Some(msg) => args.push(msg),
    //        None => ()
    //    };
    //    Command::new("notify-send")
    //        .args(args)
    //        .output()
    //        .expect("failed to execute `notify-send` shell command");
    }
}
