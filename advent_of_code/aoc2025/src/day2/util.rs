
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
