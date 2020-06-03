use regex::Regex;
use serenity::model::id::UserId;
use std::any::TypeId;

/// Trait used to recognize arguments and map them to a real object.
pub trait FragMatcher {
    /// Check if the given token can be mapped into the output type.
    /// If this returns true, the associated mapper must not fail.
    fn matches(&self, frag: &str) -> bool;

    /// `TypeId` contains in the fragment, the mapper associated with this type will be used.
    fn fragment_type_id(&self) -> TypeId;
}

/// Matches a string literal perfectly.
pub struct ExactMatcher {
    literal: String,
}

impl ExactMatcher {
    pub fn new(literal: String) -> Self {
        Self { literal }
    }
}

impl FragMatcher for ExactMatcher {
    fn matches(&self, frag: &str) -> bool {
        self.literal == frag
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<()>()
    }
}

/// Matches an unsigned number.
pub struct UnsignedMatcher;

impl FragMatcher for UnsignedMatcher {
    fn matches(&self, frag: &str) -> bool {
        !frag.is_empty() && frag.chars().all(|c| c.is_numeric())
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<u64>()
    }
}

/// Matches a signed number.
pub struct SignedMatcher;

impl FragMatcher for SignedMatcher {
    fn matches(&self, frag: &str) -> bool {
        if frag.is_empty() {
            false
        } else {
            let first = frag.chars().next().unwrap();
            (first.is_numeric() || first == '-') && frag.chars().skip(1).all(|c| c.is_numeric())
        }
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<i64>()
    }
}

/// Matches a User mention (`<@123456789>`).
/// Supports nicks.
pub struct UserIdMatcher {
    regex: Regex,
}

impl Default for UserIdMatcher {
    fn default() -> Self {
        Self {
            regex: Regex::new("<@!?[0-9]+>").expect("Failed to compile user regex"),
        }
    }
}

impl FragMatcher for UserIdMatcher {
    fn matches(&self, frag: &str) -> bool {
        self.regex.is_match(frag)
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<UserId>()
    }
}

#[cfg(test)]
mod tests {
    use crate::matchers::{ExactMatcher, FragMatcher, UserIdMatcher};

    #[test]
    pub fn test_matcher_exact() {
        let matcher = ExactMatcher::new(String::from("a word"));

        assert!(matcher.matches("a word"));
        assert!(!matcher.matches("a word "));
        assert!(!matcher.matches("nope"));
    }

    #[test]
    pub fn test_matcher_exact2() {
        let matcher = ExactMatcher::new(String::from("12hey"));

        assert!(matcher.matches("12hey"));
        assert!(!matcher.matches("12"));
        assert!(!matcher.matches("hey"));
        assert!(!matcher.matches("12 hey"));
    }

    #[test]
    pub fn test_matcher_user_id() {
        let matcher = UserIdMatcher::default();

        assert!(matcher.matches("<@123>"));
        assert!(matcher.matches("<@!123>"));

        assert!(!matcher.matches("<@abc>"));
        assert!(!matcher.matches("<@123"));
        assert!(!matcher.matches("<123>"));
        assert!(!matcher.matches("123"));
    }
}
