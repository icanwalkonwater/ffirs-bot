use crate::cmd_manager::CmdManager;
use crate::cmd_tree::{CmdNode, CmdTree};
use crate::error::{CmdError, CmdResult};
use crate::fragment_iter::FragmentIter;

pub type CmdPath<'a> = Vec<&'a CmdNode>;

impl CmdManager {
    pub fn try_find_cmd_path(cmd_root: &CmdTree, raw: String) -> CmdResult<CmdPath> {
        let frags = FragmentIter::new(raw).collect::<Vec<_>>();

        let err = frags.iter().enumerate().find(|(_, res)| res.is_err());
        if let Some((i, _)) = err {
            let err = frags.into_iter().nth(i).unwrap();
            return Err(err.unwrap_err());
        }

        let frags = frags
            .into_iter()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();

        Self::walk_command_tree(cmd_root, &frags).ok_or(CmdError::NoPathFound)
    }

    pub(self) fn walk_command_tree<'a>(node: &'a CmdNode, frags: &[String]) -> Option<CmdPath<'a>> {
        if let Some(frag) = frags.first() {
            if node.pattern.matches(frag) {
                if node.children.is_empty() {
                    // We're at the end of the chain
                    Some(vec![node])
                } else {
                    // The node is good and has children, recurse and get the first matching path
                    let child_res = node
                        .children
                        .iter()
                        .find_map(|child| Self::walk_command_tree(child, &frags[1..]));

                    if let Some(mut chain) = child_res {
                        // There is a match down the recursion, append to the chain and return up
                        chain.push(node);
                        Some(chain)
                    } else {
                        // No match found in the children
                        None
                    }
                }
            } else {
                // The node didn't match the fragment
                None
            }
        } else {
            // No fragment left, can't match this node
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd_manager::CmdManager;
    use crate::cmd_tree::CmdNode;
    use crate::matchers::{ExactMatcher, FragMatcher, SignedMatcher, UnsignedMatcher};
    use std::any::{Any, TypeId};

    fn make_tree() -> CmdNode {
        CmdNode {
            pattern: Box::new(ExactMatcher::new(String::from("root"))),
            children: vec![
                CmdNode {
                    pattern: Box::new(ExactMatcher::new(String::from("add"))),
                    children: vec![CmdNode {
                        pattern: Box::new(SignedMatcher),
                        children: vec![CmdNode {
                            pattern: Box::new(UnsignedMatcher),
                            children: vec![],
                        }],
                    }],
                },
                CmdNode {
                    pattern: Box::new(ExactMatcher::new(String::from("add"))),
                    children: vec![CmdNode {
                        pattern: Box::new(ExactMatcher::new(String::from("infty"))),
                        children: vec![CmdNode {
                            pattern: Box::new(SignedMatcher),
                            children: vec![],
                        }],
                    }],
                },
                CmdNode {
                    pattern: Box::new(ExactMatcher::new(String::from("sub"))),
                    children: vec![],
                },
            ],
        }
    }

    #[test]
    pub fn test_tree_simple() {
        let cmd_root = make_tree();

        let frags = vec!["root".to_owned(), "sub".to_owned()];
        let path = CmdManager::walk_command_tree(&cmd_root, &frags).unwrap();

        assert_eq!(path.len(), 2);
        assert_eq!(path[0].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher
        assert_eq!(path[1].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher

        assert_eq!(
            path[0]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("sub"))
        );
        assert_eq!(
            path[1]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("root"))
        );
    }

    #[test]
    pub fn test_simple_frag_leftover() {
        let cmd_root = make_tree();

        let frags = vec!["root".to_owned(), "sub".to_owned(), "garbage".to_owned()];
        let path = CmdManager::walk_command_tree(&cmd_root, &frags);

        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 2)
    }

    #[test]
    pub fn test_tree_complex() {
        let cmd_root = make_tree();

        let frags = vec![
            "root".to_owned(),
            "add".to_owned(),
            "-12".to_owned(),
            "42".to_owned(),
        ];
        let path = CmdManager::walk_command_tree(&cmd_root, &frags).unwrap();

        assert_eq!(path.len(), 4);
        assert_eq!(path[0].pattern.fragment_type_id(), TypeId::of::<u64>()); // Unsigned matcher
        assert_eq!(path[1].pattern.fragment_type_id(), TypeId::of::<i64>()); // Signed matcher
        assert_eq!(path[2].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher
        assert_eq!(path[3].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher

        assert_eq!(
            path[0]
                .pattern
                .as_any()
                .downcast_ref::<UnsignedMatcher>()
                .unwrap(),
            &UnsignedMatcher
        );
        assert_eq!(
            path[1]
                .pattern
                .as_any()
                .downcast_ref::<SignedMatcher>()
                .unwrap(),
            &SignedMatcher
        );
        assert_eq!(
            path[2]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("add"))
        );
        assert_eq!(
            path[3]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("root"))
        );
    }

    #[test]
    pub fn test_tree_complex2() {
        let cmd_root = make_tree();

        let frags = vec![
            "root".to_owned(),
            "add".to_owned(),
            "infty".to_owned(),
            "42".to_owned(),
        ];
        let path = CmdManager::walk_command_tree(&cmd_root, &frags).unwrap();

        assert_eq!(path.len(), 4);
        assert_eq!(path[0].pattern.fragment_type_id(), TypeId::of::<i64>()); // Signed matcher
        assert_eq!(path[1].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher
        assert_eq!(path[2].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher
        assert_eq!(path[3].pattern.fragment_type_id(), TypeId::of::<()>()); // Exact matcher

        assert_eq!(
            path[0]
                .pattern
                .as_any()
                .downcast_ref::<SignedMatcher>()
                .unwrap(),
            &SignedMatcher
        );
        assert_eq!(
            path[1]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("infty"))
        );
        assert_eq!(
            path[2]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("add"))
        );
        assert_eq!(
            path[3]
                .pattern
                .as_any()
                .downcast_ref::<ExactMatcher>()
                .unwrap(),
            &ExactMatcher::new(String::from("root"))
        );
    }

    #[test]
    pub fn test_tree_invalid() {
        let cmd_root = make_tree();

        let frags = vec!["garbage".to_owned()];
        assert!(CmdManager::walk_command_tree(&cmd_root, &frags).is_none());

        let frags = vec!["root".to_owned(), "add".to_owned()];
        assert!(CmdManager::walk_command_tree(&cmd_root, &frags).is_none());

        let frags = vec![
            "root".to_owned(),
            "add".to_owned(),
            "-12".to_owned(),
            "-12".to_owned(),
        ];
        assert!(CmdManager::walk_command_tree(&cmd_root, &frags).is_none());
    }
}
