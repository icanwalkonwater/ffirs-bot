use crate::cmd_tree::CmdNode;
use crate::error::{CmdError, CmdResult};
use crate::matchers::{FragMatcher};

pub type CmdTreeBuilder = CmdTreeBuilderBranched;

pub struct CmdTreeBuilderBranched {
    stack: Vec<CmdNode>,
}

impl CmdTreeBuilderBranched {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn from_consumer(
        consumer: fn(&mut CmdTreeBuilderBranched) -> CmdResult<()>,
    ) -> CmdResult<CmdTreeBuilderBranched> {
        let mut builder = Self::new();
        consumer(&mut builder)?;
        Ok(builder)
    }

    pub fn flat(
        &mut self,
        consumer: fn(&mut CmdTreeBuilderParallel) -> CmdResult<()>,
    ) -> CmdResult<&mut Self> {
        if self.stack.is_empty() {
            return Err(CmdError::EmptyCmdBuilder);
        }

        let mut parallel_builder = CmdTreeBuilderParallel {
            root: self.stack.last_mut().unwrap(),
            branches: Vec::new(),
        };
        consumer(&mut parallel_builder)?;
        parallel_builder.build()?;

        Ok(self)
    }

    #[inline]
    pub fn raw_matcher<M: FragMatcher>(&mut self, matcher: M) -> &mut Self {
        let node = CmdNode::new(matcher);
        self.stack.push(node);
        self
    }

    #[inline]
    pub fn raw_matcher_boxed(&mut self, matcher: Box<dyn FragMatcher>) -> &mut Self {
        let node = CmdNode::new_raw(matcher);
        self.stack.push(node);
        self
    }

    #[inline]
    pub fn raw_node(&mut self, node: CmdNode) -> &mut Self {
        self.stack.push(node);
        self
    }

    pub fn build(self) -> CmdResult<CmdNode> {
        let mut stack = self.stack;

        if stack.is_empty() {
            return Err(CmdError::EmptyCmdBuilder);
        }

        // Chain every node contained in the stack
        let mut root = stack.remove(0);
        stack.into_iter().fold(&mut root, |acc, node| {
            acc.children.push(node);
            // The node we just pushed becomes the acc
            acc.children.last_mut().unwrap()
        });

        Ok(root)
    }
}

pub struct CmdTreeBuilderParallel<'a> {
    root: &'a mut CmdNode,
    branches: Vec<CmdNode>,
}

impl CmdTreeBuilderParallel<'_> {
    pub fn branch(&mut self, consumer: fn(&mut CmdTreeBuilderBranched)) -> CmdResult<&mut Self> {
        let mut builder = CmdTreeBuilderBranched::new();
        consumer(&mut builder);
        self.branches.push(builder.build()?);

        Ok(self)
    }

    fn build(self) -> CmdResult<()> {
        let branches = self.branches;

        if branches.is_empty() {
            return Err(CmdError::EmptyCmdBuilder);
        }

        self.root.children.extend(branches.into_iter());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::{ExactMatcher, FragMatcher, SignedMatcher};
    use std::any::TypeId;
    use crate::cmd_tree_builder_ext::CmdTreeBuilderExt;
    use crate::cmd_tree_builder::CmdTreeBuilder;

    fn downcast_pattern<T: FragMatcher>(node: &dyn FragMatcher) -> &T {
        node.as_any().downcast_ref().unwrap()
    }

    #[test]
    pub fn test_builder_simple_branch() {
        let root = CmdTreeBuilder::from_consumer(|builder| {
            builder.exact("root").signed().signed();
            Ok(())
        })
        .unwrap()
        .build()
        .unwrap();

        assert_eq!(
            downcast_pattern::<ExactMatcher>(root.matcher.as_ref()),
            &ExactMatcher::new("root".into())
        );
        assert_eq!(root.children.len(), 1);
        assert_eq!(
            root.children[0].matcher.type_id(),
            TypeId::of::<SignedMatcher>()
        );
        assert_eq!(root.children[0].children.len(), 1);
        assert!(root.children[0].children[0].children.is_empty());
    }

    #[test]
    pub fn test_builder_parallel() {
        let root = CmdTreeBuilder::from_consumer(|builder| {
            builder.exact("root").flat(|level| {
                level
                    .branch(|branch| {
                        branch.exact("add").signed().signed();
                    })?
                    .branch(|branch| {
                        branch.exact("neg").signed();
                    })?;
                Ok(())
            })?;
            Ok(())
        })
        .unwrap()
        .build()
        .unwrap();

        assert_eq!(root.children.len(), 2);

        let branch_add = &root.children[0];
        assert_eq!(
            downcast_pattern::<ExactMatcher>(branch_add.matcher.as_ref()),
            &ExactMatcher::new("add".into())
        );
        assert_eq!(branch_add.children.len(), 1);
        assert_eq!(branch_add.children[0].children.len(), 1);
        assert!(branch_add.children[0].children[0].children.is_empty());

        let branch_sub = &root.children[1];
        assert_eq!(
            downcast_pattern::<ExactMatcher>(branch_sub.matcher.as_ref()),
            &ExactMatcher::new("neg".into())
        );
        assert_eq!(branch_sub.children.len(), 1);
        assert!(branch_sub.children[0].children.is_empty());
    }
}
