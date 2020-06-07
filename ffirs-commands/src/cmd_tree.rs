use crate::matchers::FragMatcher;
use std::fmt::Debug;

pub type CmdTree = CmdNode;

#[derive(Debug)]
pub struct CmdNode {
    pub matcher: Box<dyn FragMatcher>,
    pub children: Vec<CmdNode>,
    pub name: Option<String>,
}

impl CmdNode {
    pub fn new<M: FragMatcher>(matcher: M) -> Self {
        Self {
            matcher: Box::new(matcher),
            children: Vec::new(),
            name: None
        }
    }

    pub fn new_raw(matcher: Box<dyn FragMatcher>) -> Self {
        Self {
            matcher,
            children: Vec::new(),
            name: None,
        }
    }

    pub fn new_named<N: Into<String>>(matcher: Box<dyn FragMatcher>, name: N) -> Self {
        Self {
            matcher,
            children: Vec::new(),
            name: Some(name.into()),
        }
    }
}
