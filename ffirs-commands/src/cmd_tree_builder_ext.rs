use crate::matchers::{ExactMatcher, SignedMatcher, UnsignedMatcher, UserMentionMatcher};
use crate::error::CmdResult;
use crate::cmd_tree_builder::CmdTreeBuilderBranched;

/// Helpers to easily add specific matchers.
pub trait CmdTreeBuilderExt {
    fn exact(&mut self, literal: &str) -> &mut Self;
    fn signed(&mut self) -> &mut Self;
    fn unsigned(&mut self) -> &mut Self;
    fn user_mention(&mut self) -> &mut Self;
}

impl CmdTreeBuilderExt for CmdTreeBuilderBranched {
    fn exact(&mut self, literal: &str) -> &mut Self {
        self.raw_matcher(ExactMatcher::new(literal.into()))
    }

    fn signed(&mut self) -> &mut Self {
        self.raw_matcher(SignedMatcher)
    }

    fn unsigned(&mut self) -> &mut Self {
        self.raw_matcher(UnsignedMatcher)
    }

    fn user_mention(&mut self) -> &mut Self {
        self.raw_matcher(UserMentionMatcher::default())
    }
}

fn parse_command_build() {

}

// root add <n1: i64> <mention: UserId>
fn cmd_branch_from_string(pattern: &str) -> CmdResult<CmdTreeBuilderBranched> {
    let mut builder = CmdTreeBuilderBranched::new();

    Ok(builder)
}
