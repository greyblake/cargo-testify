use std::time::Duration;
use std::path::PathBuf;

use errors::*;

pub struct Config<'a> {
    pub ignore_duration: Duration,
    pub project_dir: PathBuf,
    pub cargo_test_args: Vec<&'a str>
}

pub struct ConfigBuilder<'a> {
    ignore_duration: Duration,
    project_dir: Option<PathBuf>,
    cargo_test_args: Vec<&'a str>
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        Self {
            ignore_duration: Duration::from_millis(300),
            project_dir: None,
            cargo_test_args: vec![]
        }
    }

    pub fn project_dir(mut self, dir: PathBuf) -> Self {
        self.project_dir = Some(dir);
        self
    }

    pub fn cargo_test_args(mut self, args: Vec<&'a str>) -> Self {
        self.cargo_test_args = args;
        self
    }

    pub fn build(self) -> Result<Config<'a>> {
        let project_dir = self.project_dir.ok_or(ErrorKind::ProjectDirMissing)?;

        let config = Config {
            ignore_duration: self.ignore_duration,
            cargo_test_args: self.cargo_test_args,
            project_dir: project_dir
        };
        Ok(config)
    }
}
