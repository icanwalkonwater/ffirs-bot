use crate::matchers::FragmentMatcher;
use std::ops::Deref;

pub type CommandTree = CommandNode;

pub struct CommandNode {
    pub pattern: Box<dyn FragmentMatcher>,
    pub children: Vec<CommandNode>,
}

impl Deref for CommandNode {
    type Target = [CommandNode];

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl CommandNode {
    pub fn filter(&self, fragment: &str) -> Vec<&CommandNode> {
        self.children
            .iter()
            .filter(|node| node.pattern.matches(fragment))
            .collect()
    }
}
