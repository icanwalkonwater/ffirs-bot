use crate::cmd_tree::CmdNode;
use crate::cmd_tree_builder::CmdTreeBuilderBranched;
use crate::error::{CmdError, CmdResult};
use crate::matchers::{
    ExactMatcher, FragMatcher, SignedMatcher, UnsignedMatcher, UserMentionMatcher,
};

pub(self) struct CmdCreatorBranchIter {
    original: String,
    current_position: usize,
}

impl CmdCreatorBranchIter {
    pub fn new(original: String) -> Self {
        Self {
            original,
            current_position: 0,
        }
    }

    #[inline]
    fn make_matcher(ty: &str) -> CmdResult<Box<dyn FragMatcher>> {
        match ty {
            "Unsigned" => Ok(Box::new(UnsignedMatcher)),
            "Signed" => Ok(Box::new(SignedMatcher)),
            "UserMention" => Ok(Box::new(UserMentionMatcher::default())),
            _ => Err(CmdError::CreatorUnknownMatcher { ty: ty.into() }),
        }
    }
}

impl Iterator for CmdCreatorBranchIter {
    type Item = CmdResult<(Box<dyn FragMatcher>, Option<String>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position >= self.original.len() - 1 {
            None
        } else {
            let remaining = &self.original[self.current_position..];

            // If not typed, its an Exact
            if remaining.chars().next().unwrap() != '<' {
                let lit = remaining
                    .chars()
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>();

                self.current_position += lit.len() + 1; // Also skip the whitespace

                Some(Ok((Box::new(ExactMatcher::new(lit)), None)))
            } else {
                // We have a typed part: <name: Type>
                let segment = remaining
                    .chars()
                    .skip(1)
                    .take_while(|&c| c != '>')
                    .collect::<String>();

                self.current_position += segment.len() + 3;

                // Look for separator
                if let Some(sep) = segment.find(':') {
                    let (name, ty) = segment.split_at(sep);
                    let ty = &ty[1..];

                    let matcher = Self::make_matcher(ty.trim());
                    match matcher {
                        Ok(matcher) => Some(Ok((matcher, Some(name.into())))),
                        Err(err) => Some(Err(err)),
                    }
                } else {
                    // No separator, just assume its an Exact named
                    let name = segment.trim();
                    Some(Ok((
                        Box::new(ExactMatcher::new(name.into())),
                        Some(name.into()),
                    )))
                }
            }
        }
    }
}

pub struct CmdCreator;

impl CmdCreator {
    pub fn create_cmd_branch(format: &str) -> CmdResult<CmdNode> {
        let mut builder = CmdTreeBuilderBranched::new();
        let mut iter = CmdCreatorBranchIter::new(format.into());

        for res in iter {
            let (matcher, name) = res?;

            if let Some(name) = name {
                builder.raw_node(CmdNode::new_named(matcher, name));
            } else {
                builder.raw_matcher_boxed(matcher);
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd_creator::CmdCreatorBranchIter;
    use crate::matchers::{ExactMatcher, FragMatcher, SignedMatcher, UserMentionMatcher};
    use std::any::TypeId;

    fn downcast_pattern<T: FragMatcher>(node: &dyn FragMatcher) -> &T {
        node.as_any().downcast_ref().unwrap()
    }

    #[test]
    pub fn test_creator_iter_simple() {
        let mut iter = CmdCreatorBranchIter::new("root add sub".into());

        let mut next = iter.next();
        assert!(next.is_some());
        assert_eq!(
            downcast_pattern::<ExactMatcher>(next.unwrap().unwrap().0.as_ref()),
            &ExactMatcher::new("root".into())
        );

        next = iter.next();
        assert!(next.is_some());
        assert_eq!(
            downcast_pattern::<ExactMatcher>(next.unwrap().unwrap().0.as_ref()),
            &ExactMatcher::new("add".into())
        );

        next = iter.next();
        assert!(next.is_some());
        assert_eq!(
            downcast_pattern::<ExactMatcher>(next.unwrap().unwrap().0.as_ref()),
            &ExactMatcher::new("sub".into())
        );

        assert!(iter.next().is_none());
    }

    #[test]
    pub fn test_creator_iter_complex() {
        let mut iter = CmdCreatorBranchIter::new("root <a: Signed> <b: UserMention>".into());

        let next = iter.next();
        assert!(next.is_some());
        assert_eq!(
            downcast_pattern::<ExactMatcher>(next.unwrap().unwrap().0.as_ref()),
            &ExactMatcher::new("root".into())
        );

        let next = iter.next();
        assert!(next.is_some());
        let next = next.unwrap().unwrap();
        assert_eq!(next.0.type_id(), TypeId::of::<SignedMatcher>());
        assert_eq!(&next.1.unwrap(), "a");

        let next = iter.next();
        assert!(next.is_some());
        let next = next.unwrap().unwrap();
        assert_eq!(next.0.type_id(), TypeId::of::<UserMentionMatcher>());
        assert_eq!(&next.1.unwrap(), "b");

        assert!(iter.next().is_none());
    }

    #[test]
    pub fn test_creator_iter_panic() {
        let mut iter = CmdCreatorBranchIter::new("root <a: Garbage>".into());

        let next = iter.next();
        assert!(next.is_some());
        assert_eq!(
            downcast_pattern::<ExactMatcher>(next.unwrap().unwrap().0.as_ref()),
            &ExactMatcher::new("root".into())
        );

        let next = iter.next();
        assert!(next.is_some());
        assert!(next.unwrap().is_err());
    }
}
