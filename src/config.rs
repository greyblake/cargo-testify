use std::path::PathBuf;
use std::time::Duration;

use glob::Pattern;

use errors::*;

pub struct Config<'a> {
    pub ignore_duration: Duration,
    pub project_dir: PathBuf,
    pub cargo_test_args: Vec<&'a str>,
    pub patterns: Vec<Pattern>,
}

pub struct ConfigBuilder<'a> {
    ignore_duration: Duration,
    project_dir: Option<PathBuf>,
    cargo_test_args: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        let default_patterns = vec![
            "src/**/*.rs",
            "tests/**/*.rs",
            "Cargo.toml",
            "Cargo.lock",
            "build.rs",
        ];

        Self {
            ignore_duration: Duration::from_millis(300),
            project_dir: None,
            cargo_test_args: vec![],
            patterns: default_patterns,
        }
    }

    pub fn project_dir(mut self, dir: PathBuf) -> Self {
        self.project_dir = Some(dir);
        self
    }

    pub fn include_patterns(mut self, pattern: &[&'a str]) -> Self {
        self.patterns.extend_from_slice(pattern);
        self
    }

    pub fn cargo_test_args(mut self, args: Vec<&'a str>) -> Self {
        self.cargo_test_args = args;
        self
    }

    pub fn build(self) -> Result<Config<'a>> {
        let project_dir = self.project_dir.ok_or(ErrorKind::ProjectDirMissing)?;
        let patterns: Vec<Pattern> = self
            .patterns
            .iter()
            .map(|p| Pattern::new(p).map_err(|e| e.into()))
            .collect::<Result<_>>()?;

        let config = Config {
            ignore_duration: self.ignore_duration,
            cargo_test_args: self.cargo_test_args,
            project_dir: project_dir,
            patterns: patterns,
        };
        Ok(config)
    }
}
