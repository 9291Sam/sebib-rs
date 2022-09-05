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

pub mod seblog {
    extern crate chrono;

    enum MessageCategory {
        Fatal,
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }

    impl std::fmt::Display for MessageCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
            use self::MessageCategory::*;
            match *self {
                Fatal => write!(f, "Fatal"),
                Error => write!(f, "Error"),
                Warn  => write!(f, " Warn"),
                Info  => write!(f, " Info"),
                Debug => write!(f, "Debug"),
                Trace => write!(f, "Trace"),
            }
        }
    }

    struct Message {
        message: String,
        calling_file_location: String,
        message_category: MessageCategory,
    }
    pub struct LogHandle {
        handle: std::sync::mpsc::Sender<Message>,
    }

    impl LogHandle {

        #[track_caller]
        pub fn fatal(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Fatal,
                }
            ).expect("Failed to send message to sebib logger");
        }

        #[track_caller]
        pub fn error(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Error,
                }
            ).expect("Failed to send message to sebib logger");
        }

        #[track_caller]
        pub fn warn(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Warn,
                }
            ).expect("Failed to send message to sebib logger");
        }

        #[track_caller]
        pub fn info(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Info,
                }
            ).expect("Failed to send message to sebib logger");
        }

        #[track_caller]
        pub fn debug(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Debug,
                }
            ).expect("Failed to send message to sebib logger");
        }

        #[track_caller]
        pub fn trace(&self, message: &str) {
            self.handle.send(
                Message {
                    message: message.to_string(),
                    calling_file_location: str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                    message_category: MessageCategory::Trace,
                }
            ).expect("Failed to send message to sebib logger");
        }
    }

    pub fn init() -> LogHandle {

        let (tx, rx) = std::sync::mpsc::channel::<Message>();

        std::thread::Builder::new().name("SEBIB_LOGGER_THREAD".to_string()).spawn(move || -> ! {
            fn get_current_timestamp() -> String {
                let first_formatted_time: String = chrono::Local::now().format("%I:%M:%S:%6f %p").to_string();
                String::from(std::format!("{}:{}", &first_formatted_time[0..12],  &first_formatted_time[12..])) // Adding a colon in the middile of the ms / us digits
            }

            loop {
                use MessageCategory::*;
                match rx.recv() {
                    Ok(message) => match message.message_category { 
                        Fatal | Error => eprintln!("[{}] [{}] {}: {}", get_current_timestamp(), message.calling_file_location, message.message_category,  message.message),
                        _             =>  println!("[{}] [{}] {}: {}", get_current_timestamp(), message.calling_file_location, message.message_category,  message.message),
                    }, // Ok

                    Err(_) => {
                        use std::io::Write;
                        std::io::stdout().flush().expect("Failed to flush stdout");
                        panic!("SEBIB sender has disconnected")
                    } // Err
                } // rx.recv()
            } // loop
        }).expect("SEBIB thread failed to be made");


        LogHandle { handle: tx }
    }
} // mod seblog