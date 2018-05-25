error_chain! {
    errors {
        ProjectDirMissing { description("project directory is missing") }
    }

    foreign_links {
        PatternError(::glob::PatternError);
    }
}
