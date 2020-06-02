use regex::Regex;
use serenity::model::id::UserId;
use std::any::{Any, TypeId};

/// Trait used to recognize arguments and map them to a real object.
pub trait FragmentMatcher {
    /// Check if the given token can be mapped into the output type.
    /// If this returns true, the associated mapper must not fail.
    fn matches(&self, fragment: &str) -> bool;

    /// `TypeId` contains in the fragment, the mapper associated with this type will be used.
    fn fragment_type_id(&self) -> TypeId;
}

/// Matches a string literal perfectly.
/// Maps to nothing.
pub struct ExactMatcher {
    literal: String,
}

impl FragmentMatcher for ExactMatcher {
    fn matches(&self, fragment: &str) -> bool {
        self.literal == fragment
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<()>()
    }
}

/// Matches a User mention (`<@123456789>`).
/// Maps its user ID.
pub struct UserMatcher {
    regex: Regex,
}

impl Default for UserMatcher {
    fn default() -> Self {
        Self {
            regex: Regex::new("<@!?[0-9]+>").expect("Failed to compile user regex"),
        }
    }
}

impl FragmentMatcher for UserMatcher {
    fn matches(&self, fragment: &str) -> bool {
        self.regex.is_match(fragment)
    }

    fn fragment_type_id(&self) -> TypeId {
        TypeId::of::<UserId>()
    }
}
