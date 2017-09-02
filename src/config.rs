use std::time::Duration;
use std::path::PathBuf;
use notifiers::Notify;

use errors::*;

pub struct Config {
    pub ignore_duration: Duration,
    pub project_dir: PathBuf,
    pub target_dir: PathBuf,
    pub notifier: Box<Notify>
}

pub struct ConfigBuilder {
    ignore_duration: Duration,
    project_dir: Option<PathBuf>,
    notifier: Option<Box<Notify>>
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            ignore_duration: Duration::from_millis(300),
            project_dir: None,
            notifier: None
        }
    }

    pub fn project_dir(mut self, dir: PathBuf) -> Self {
        self.project_dir = Some(dir);
        self
    }

    pub fn notifier(mut self, notifier: Box<Notify>) -> Self {
        self.notifier = Some(notifier);
        self
    }

    pub fn build(self) -> Result<Config> {
        let notifier = self.notifier.ok_or(ErrorKind::NotifierMissing)?;
        let project_dir = self.project_dir.ok_or(ErrorKind::ProjectDirMissing)?;
        let target_dir = project_dir.join("target");

        let config = Config {
            ignore_duration: self.ignore_duration,
            project_dir: project_dir,
            target_dir: target_dir,
            notifier: notifier
        };
        Ok(config)
    }
}
