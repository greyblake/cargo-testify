error_chain! {
    errors {
        ProjectDirMissing { description("project directory is missing") }
        NotifierMissing { description("notifier is missing") }
    }
}
