use crate::matchers::FragMatcher;
use std::fmt::Debug;

pub type CmdTree = CmdNode;

#[derive(Debug)]
pub struct CmdNode {
    pub pattern: Box<dyn FragMatcher>,
    pub children: Vec<CmdNode>,
}

impl CmdNode {
    pub fn filter(&self, frag: &str) -> Vec<&CmdNode> {
        self.children
            .iter()
            .filter(|node| node.pattern.matches(frag))
            .collect()
    }
}
