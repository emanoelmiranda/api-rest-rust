pub struct Logger;

impl Logger {
    pub fn log<T: std::fmt::Debug>(&self, message: T) {
        println!("{:?}", message)
    }
}