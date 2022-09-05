<<<<<<< HEAD:src/seblog.rs
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
        self.send(message,
            str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
            MessageCategory::Fatal)
    }

    #[track_caller]
    pub fn error(&self, message: &str) {
        self.send(message,
            str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
            MessageCategory::Error)
    }

    #[track_caller]
    pub fn warn(&self, message: &str) {
        self.send(message,
            str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
            MessageCategory::Warn)
    }

    #[track_caller]
    pub fn info(&self, message: &str) {
        self.send(message,
            str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
            MessageCategory::Info)
    }

    #[track_caller]
    pub fn debug(&self, message: &str) {
        self.send(message,
            str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
            MessageCategory::Debug)
    }

    #[track_caller]
    pub fn trace(&self, message: &str) {
        self.send(message,
                str::replace(&std::panic::Location::caller().to_string(), "\\", "/"),
                MessageCategory::Trace)
    }

    fn send(&self, message: &str, calling_file_location: String, message_category: MessageCategory) {
        self.handle.send(
            Message {
                message: message.to_string(),
                calling_file_location: calling_file_location,
                message_category: message_category,
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
    }).expect("SEBIB thread failed to initalize");

    LogHandle { handle: tx }
}
=======
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
>>>>>>> 949326bbd8f9aca0bbd36650dce645bb607fb48b:seblog.rs
