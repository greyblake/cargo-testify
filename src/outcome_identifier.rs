use regex::Regex;
use outcome::Outcome;

/// Determines what is result of running tests, based on the following information:
/// * Did process finish successfully?
/// * Stdout
/// * Stderr
///
/// The structure just keeps compiled regular expressions, so they can be used
/// every time `identify` function is called.
pub struct OutcomeIdentifier {
    result_re: Regex,
    error_re: Regex
}

impl OutcomeIdentifier {
    pub fn new() -> Self {
        // Unwrap here is always safe, because the regexps are valid
        Self {
            result_re: Regex::new(r"\d{1,} passed.*filtered out").unwrap(),
            error_re: Regex::new(r"error(:|\[).*").unwrap()
        }
    }

    // TODO:
    // Get rid of unwraps, instead return Outcome without text message.
    pub fn identify<'a>(&self, process_success: bool, stdout: &'a str, stderr: &'a str) -> Outcome<'a> {
        if process_success {
            let message = self.result_re.find(stdout).unwrap().as_str();
            Outcome::TestsPassed(message)
        } else {
            match self.result_re.find(stdout) {
                Some(matched) => {
                    Outcome::TestsFailed(matched.as_str())
                },
                None => {
                    let message = self.error_re.find(stderr).unwrap().as_str();
                    Outcome::CompileError(message)
                }
            }
        }
    }
}
