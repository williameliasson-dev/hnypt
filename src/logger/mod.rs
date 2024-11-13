pub struct Logger {}

impl Logger {
    pub fn info(msg: &str) {
        println!("INFO: {}", msg)
    }
    pub fn warn(msg: &str) {
        println!("WARN: {}", msg)
    }

    pub fn error(msg: &str) {
        println!("ERROR: {}", msg)
    }
}
