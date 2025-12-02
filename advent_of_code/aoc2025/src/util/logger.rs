use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init_logger(log: bool) {
    LOGGER.set(Logger::new(log)).expect("Failed to initialize logger")
}

pub fn logger() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
}

#[derive(Debug)]
pub struct Logger {
    log: bool,
}

impl Logger {
    pub fn new(log: bool) -> Self {
        Logger { log }
    }
    pub fn log(&self, message: &str) {
        if self.log {
            println!("{}", message);
        }
    }
}
