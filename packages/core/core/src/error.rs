use std::fmt::Display;




#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message:&str) -> Self {
        Self {message:message.to_owned()}
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Error {}