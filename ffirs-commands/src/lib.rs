use anyhow::Result;
use serenity::model::prelude::Message;
use std::env::Args;
use thiserror::Error;

pub mod command_tree;
pub mod fragment;
pub mod matchers;

pub type CommandResult<T> = std::result::Result<T, CommandError>;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Command parsing error at [{start}..{end}]: {message}")]
    ParsingError {
        message: String,
        start: usize,
        end: usize,
    }
}

// #[command]
// #[aliases = "ping,p"]
// #[syntax = "<first: User> <chan: Channel> <second: User>"]
fn ping(message: Message, args: Args) -> Result<()> {
    Ok(())
}

pub fn parse(message: Message) {
    let content = message.content;
}
