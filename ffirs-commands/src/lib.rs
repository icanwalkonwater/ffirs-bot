use std::env::Args;

use serenity::model::prelude::Message;

use error::CmdResult;

pub mod cmd_creator;
pub mod cmd_tree_builder_ext;
pub mod cmd_tree_builder;
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
fn ping(_message: Message, _args: Args) -> CmdResult<()> {
    Ok(())
}

pub fn parse(message: Message) {
    let _content = message.content;
}
