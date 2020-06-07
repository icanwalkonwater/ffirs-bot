use thiserror::Error;

pub type CmdResult<T> = std::result::Result<T, CmdError>;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("Command parsing error at [{start}..{end}]: {message}")]
    ParsingError {
        message: String,
        start: usize,
        end: usize,
    },
    #[error("Command not found: {name}")]
    NotFound { name: String },
    #[error("No matching path found")]
    NoPathFound,
    #[error("Missing permission, required level: {level}")]
    MissingPerm { level: u32 },

    #[error("Can't build a command without fragments !")]
    EmptyCmdBuilder,
    #[error("Unknown matcher type ecoutered: {ty}")]
    CreatorUnknownMatcher { ty: String },
}
