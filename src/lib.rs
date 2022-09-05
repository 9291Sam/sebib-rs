mod seblog;
mod string;
#[cfg(test)]
mod tests {
    use crate::seblog;
    use crate::string::TrimNewlineExt;

    #[test]
    fn test_seblog() {
        // Rember to run with `cargo test -- --nocapture`
        let handle = seblog::init();

        handle.fatal("Fatal Error");
        handle.error("Recoverable Error");
        handle.warn("warning");
        handle.info("info");
        handle.debug("Debug message");
        handle.trace("Tracing!");
        std::thread::sleep(std::time::Duration::from_micros(50));
    }

    #[test]
    fn trim_newline_test() {
        let result = String::from("asdf\n\n\r\n").trim_newline().to_string();
        assert_eq!(result, "asdf");
    }
}