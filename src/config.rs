use std::time::Duration;

pub struct Config {
    pub ignore_duration: Duration
}

pub struct ConfigBuilder {
    ignore_duration: Duration
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            ignore_duration: Duration::from_millis(300)
        }
    }

    pub fn build(&self) -> Config {
        Config {
            ignore_duration: self.ignore_duration
        }
    }
}

