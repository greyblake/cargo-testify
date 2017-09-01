/// This enum represents an outcome of attempt to run tests.
/// It's passed to a notifier in order to display a message to a user.
/// Every invariant contains a text message, that is shown in addition.
pub enum Outcome<'a> {
    /// Tests have passed successfully
    TestsPassed(&'a str),

    /// Tests failed
    TestsFailed(&'a str),

    /// Compilation error detected
    CompileError(&'a str)
}
