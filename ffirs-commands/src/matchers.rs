use regex::Regex;
use serenity::model::id::UserId;
use std::any::{Any, TypeId};
use std::fmt::Debug;

/// Trait used to recognize arguments and map them to a real object.
pub trait FragMatcher: Debug + Any {
    /// Check if the given token can be mapped into the output type.
    /// If this returns true, the associated mapper must not fail.
    fn matches(&self, frag: &str) -> bool;

    /// `TypeId` contains in the fragment, the mapper associated with this type will be used.
    fn fragment_type_id(&self) -> TypeId;
}

/// Matches a string literal perfectly.
#[derive(Debug, Clone)]
pub struct ExactMatcher {
    literal: String,
}

impl ExactMatcher {
    pub fn new(literal: String) -> Self {
        Self { literal }
    }
}

impl PartialEq for ExactMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
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

impl PartialEq for UserIdMatcher {
    fn eq(&self, _: &Self) -> bool {
        true
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
    use crate::matchers::{
        ExactMatcher, FragMatcher, SignedMatcher, UnsignedMatcher, UserIdMatcher,
    };

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

    #[test]
    pub fn test_matcher_signed() {
        let matcher = SignedMatcher;

        assert!(matcher.matches("12"));
        assert!(matcher.matches("-12"));
        assert!(!matcher.matches("-12a"));
        assert!(!matcher.matches("a"));
        assert!(!matcher.matches("a12"));
    }

    #[test]
    pub fn test_matcher_unsigned() {
        let matcher = UnsignedMatcher;

        assert!(matcher.matches("12"));
        assert!(!matcher.matches("-12"));
        assert!(!matcher.matches("-12a"));
        assert!(!matcher.matches("a"));
        assert!(!matcher.matches("a12"));
    }
}
