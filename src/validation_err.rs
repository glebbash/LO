use std::error::Error;

#[derive(Debug, Clone)]
pub struct ValidationError {
    message: String,
}

impl Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        ValidationError { message }
    }
}
