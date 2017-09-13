use std::time::Duration;
use std::path::PathBuf;

use errors::*;

pub struct Config {
    pub ignore_duration: Duration,
    pub project_dir: PathBuf,
}

pub struct ConfigBuilder {
    ignore_duration: Duration,
    project_dir: Option<PathBuf>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            ignore_duration: Duration::from_millis(300),
            project_dir: None,
        }
    }

    pub fn project_dir(mut self, dir: PathBuf) -> Self {
        self.project_dir = Some(dir);
        self
    }

    pub fn build(self) -> Result<Config> {
        let project_dir = self.project_dir.ok_or(ErrorKind::ProjectDirMissing)?;

        let config = Config {
            ignore_duration: self.ignore_duration,
            project_dir: project_dir
        };
        Ok(config)
    }
}
