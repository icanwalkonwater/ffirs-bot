use crate::matchers::FragMatcher;

pub type CmdTree<'a> = CmdNode<'a>;

pub struct CmdNode<'a> {
    pub pattern: &'a dyn FragMatcher,
    pub children: Vec<CmdNode<'a>>,
}

impl CmdNode<'_> {
    pub fn filter(&self, frag: &str) -> Vec<&CmdNode> {
        self.children
            .iter()
            .filter(|node| node.pattern.matches(frag))
            .collect()
    }
}
