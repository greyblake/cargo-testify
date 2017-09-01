use regex::Regex;
use report::{Report, Outcome};

/// Determines what is result of running tests, based on the following information:
/// * Did process finish successfully?
/// * Stdout
/// * Stderr
///
/// The structure just keeps compiled regular expressions, so they can be used
/// every time `identify` function is called.
pub struct ReportBuilder {
    result_re: Regex,
    error_re: Regex
}

impl ReportBuilder {
    pub fn new() -> Self {
        // Unwrap here is always safe, because the regexps are valid
        Self {
            result_re: Regex::new(r"\d{1,} passed.*filtered out").unwrap(),
            error_re: Regex::new(r"error(:|\[).*").unwrap()
        }
    }

    pub fn identify(&self, process_success: bool, stdout: &str, stderr: &str) -> Report {
        if process_success {
            let detail  = self.result_re.find(stdout).map(|m| m.as_str().to_string() );
            Report { outcome: Outcome::TestsPassed, detail: detail }
        } else {
            match self.result_re.find(stdout) {
                Some(matched) => {
                    Report { outcome: Outcome::TestsFailed, detail: Some(matched.as_str().to_string()) }
                },
                None => {
                    let detail = self.error_re.find(stderr).map(|m| m.as_str().to_string() );
                    Report { outcome: Outcome::CompileError, detail: detail }
                }
            }
        }
    }
}
