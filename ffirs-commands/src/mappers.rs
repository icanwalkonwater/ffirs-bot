use serenity::model::id::UserId;
use std::any::Any;
use std::str::FromStr;

pub type MapperOutput = Box<dyn Any>;

/// Used to map matched fragments to there output type.
/// The mapper must never `panic!` or fail because it was checked by its corresponding matcher.
pub trait FragmentMapper {
    fn map(&self, fragment: &str) -> MapperOutput;
}

/// A mapper that does nothing.
pub struct NoopMapper;

impl FragmentMapper for NoopMapper {
    fn map(&self, _: &str) -> MapperOutput {
        Box::new(())
    }
}

/// A mapper that can be used for any type that implements `FromStr`.
#[derive(Default)]
pub struct FromStrMapper<F: FromStr> {
    _phantom_data: std::marker::PhantomData<F>,
}

impl<F: FromStr + 'static> FragmentMapper for FromStrMapper<F> {
    fn map(&self, fragment: &str) -> MapperOutput {
        match F::from_str(fragment) {
            Ok(value) => Box::new(value),
            _ => unreachable!(),
        }
    }
}

/// Extracts the `UserId` from the fragment.
/// Support nicks.
pub struct UserMapper;

impl FragmentMapper for UserMapper {
    fn map(&self, fragment: &str) -> MapperOutput {
        let start = if fragment.chars().nth(2).unwrap() == '!' {
            3
        } else {
            2
        };

        let id_str = &fragment[start..fragment.len() - 1];
        Box::new(UserId(id_str.parse::<u64>().unwrap()))
    }
}

/// Return the fragment as is.
pub struct ExactMapper;

impl FragmentMapper for ExactMapper {
    fn map(&self, fragment: &str) -> MapperOutput {
        Box::new(fragment.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::mappers::{FragmentMapper, FromStrMapper, NoopMapper, UserMapper};
    use serenity::model::id::UserId;

    #[test]
    pub fn test_mapper_noop() {
        let mapper = NoopMapper;
        assert_eq!(mapper.map("garbage").downcast().unwrap(), Box::new(()));
    }

    #[test]
    pub fn test_mapper_from_str_i32() {
        let mapper = FromStrMapper::<i32>::default();

        assert_eq!(mapper.map("0").downcast().unwrap(), Box::new(0_i32));
        assert_eq!(mapper.map("12").downcast().unwrap(), Box::new(12_i32));
        assert_eq!(mapper.map("042").downcast().unwrap(), Box::new(42_i32));
    }

    #[test]
    pub fn test_mapper_from_str_bool() {
        let mapper = FromStrMapper::<bool>::default();

        assert_eq!(mapper.map("true").downcast().unwrap(), Box::new(true));
        assert_eq!(mapper.map("false").downcast().unwrap(), Box::new(false));
    }

    #[test]
    pub fn test_mapper_user_id() {
        let mapper = UserMapper;

        assert_eq!(
            mapper.map("<@123456>").downcast().unwrap(),
            Box::new(UserId(123456))
        );
        assert_eq!(
            mapper.map("<@!123456>").downcast().unwrap(),
            Box::new(UserId(123456))
        );
    }
}
