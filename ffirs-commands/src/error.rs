use thiserror::Error;

pub type CommandResult<T> = std::result::Result<T, CommandError>;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Command parsing error at [{start}..{end}]: {message}")]
    ParsingError {
        message: String,
        start: usize,
        end: usize,
    },
    #[error("Command not found: {name}")]
    NotFound { name: String },
    #[error("Missing permission, required level: {level}")]
    MissingPermission { level: u32 },
}
