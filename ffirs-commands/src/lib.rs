use std::env::Args;

use serenity::model::prelude::Message;

use error::CmdResult;

pub mod cmd_manager;
pub mod cmd_tree;
pub mod cmd_walker;
pub mod error;
pub mod fragment_iter;
pub mod mappers;
pub mod matchers;
pub mod type_map;

// #[command]
// #[aliases = "ping,p"]
// #[syntax = "<first: User> <chan: Channel> <second: User>"]
fn ping(message: Message, args: Args) -> CmdResult<()> {
    Ok(())
}

pub fn parse(message: Message) {
    let content = message.content;
}
