pub trait TrimNewlineExt {
    fn trim_newline<'a>(&'a self) -> &'a str;
}

impl TrimNewlineExt for String {
    fn trim_newline<'a>(&'a self) -> &'a str {
        const NEWLINES: &[char; 2] = &['\n', '\r'];
        self.trim_end_matches(NEWLINES)
    }
}

