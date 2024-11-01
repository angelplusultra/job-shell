use std::{error::Error, fmt};


#[derive(Debug)]
pub struct CustomError {
    pub details: String,
}

// Implement a constructor to make creating the error easier
impl CustomError {
    fn new(msg: &str) -> CustomError {
        CustomError {
            details: msg.to_string(),
        }
    }
}

// Implement the Display trait for user-friendly error messages
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// Implement the Error trait so it can be used as a standard error
impl Error for CustomError {}
