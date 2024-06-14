use colored::Colorize;

pub struct Error {
    pub message: String,
    pub pos: String,
}

impl Error {
    pub fn new(message: &str, pos: String) -> Error {
        Error {
            message: message.to_string(),
            pos: pos.to_string(),
        }
    }

    pub fn print(&self) {
        let error_message = format!(
            "{}{}{}{}",
            "[ERROR at position ".red(),
            self.pos.red(),
            "] > ".red(),
            self.message.red()
        );
        eprintln!("{}", error_message);
    }
}
