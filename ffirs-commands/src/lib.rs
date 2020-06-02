use anyhow::Result;
use serenity::model::prelude::Message;
use std::env::Args;

pub mod command_tree;
pub mod fragment;
pub mod matchers;

// #[command]
// #[aliases = "ping,p"]
// #[syntax = "<first: User> <chan: Channel> <second: User>"]
fn ping(message: Message, args: Args) -> Result<()> {
    Ok(())
}

pub fn parse(message: Message) {
    let content = message.content;
}
