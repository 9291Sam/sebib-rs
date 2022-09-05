pub trait TrimNewlineExt {
    fn trim_newline<'a>(&'a self) -> &'a str;
}

impl TrimNewlineExt for String {
    fn trim_newline<'a>(&'a self) -> &'a str {
        const NEWLINES: &[char; 2] = &['\n', '\r'];
        self.trim_end_matches(NEWLINES)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_newline_test() {
        let result = String::from("asdf\n\n\r\n").trim_newline().to_string();
        assert_eq!(result, "asdf");
    }
}