use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid command arguments: {0}")]
    InvalidArguments(String),
    #[error("Invalid command length: {0}")]
    InvalidLength(isize),
    #[error("Command is not complete")]
    NotComplete,
}
