use crate::cmd_tree::CmdNode;
use crate::matchers::ExactMatcher;

pub struct CmdBuilder<'a>(&'a mut CmdNode);

impl CmdBuilder {
    pub fn new(root: CmdNode) -> Self {
        Self(root)
    }

    pub fn new_exact(literal: &str) -> Self {
        let node = CmdNode {
            pattern: Box::new(ExactMatcher::new(String::from(literal))),
            children: vec![]
        };

        Self(node)
    }

    pub fn exact(&mut self, literal: &str) -> Self {
        let node = CmdNode {
            pattern: Box::new(ExactMatcher::new(String::from(literal))),
            children: vec![]
        };

        self.0.children.push(node);
        Self(self.0.children.last_mut().unwrap())
    }
}
