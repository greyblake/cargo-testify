pub enum Outcome<'a> {
    TestsPassed(&'a str),
    TestsFailed(&'a str),
    CompileError(&'a str)
}
