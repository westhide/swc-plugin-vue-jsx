#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! regex_set {
    ($regexps:expr) => {{
        static SET: once_cell::sync::OnceCell<regex::RegexSet> = once_cell::sync::OnceCell::new();
        SET.get_or_init(|| regex::RegexSet::new($regexps).unwrap())
    }};
}
