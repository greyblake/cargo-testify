pub struct Report {
    pub outcome: Outcome,
    pub detail: Option<String>
}

/// This enum represents an outcome of attempt to run tests.
/// It's passed to a notifier in order to display a message to a user.
pub enum Outcome {
    /// Tests have passed successfully
    TestsPassed,

    /// Tests failed
    TestsFailed,

    /// Compilation error detected
    CompileError
}

