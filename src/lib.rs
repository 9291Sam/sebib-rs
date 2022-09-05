
mod seblog;
#[cfg(test)]
mod tests {
    use super::*;

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
}
