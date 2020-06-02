use std::marker::PhantomData;

pub struct FragmentIterator {
    original: String,
    current_position: usize,
}

impl FragmentIterator {
    pub fn new(original: String) -> Self {
        Self {
            original,
            current_position: 0,
        }
    }

    pub(self) fn trim_self(&mut self) {
        self.current_position =
            self.original.len() - self.original[self.current_position..].trim_start().len();
    }

    pub(self) fn find_end_of_word(fragment: &str) -> usize {
        fragment.chars().take_while(|c| !c.is_whitespace()).count()
    }

    /// Take a string slice that begin with a type of quote.
    /// Returns the position of its corresponding quote or None.
    /// A quote can actually be any valid char.
    pub(self) fn find_end_of_quote(fragment: &str) -> Option<usize> {
        let quote = fragment.chars().next().unwrap();

        // TODO: Escape characters ?
        let quoted_length = fragment.chars().skip(1).take_while(|&c| c != quote).count();

        let supposed_end = quoted_length + 1;
        if fragment.chars().nth(supposed_end)? == quote {
            Some(supposed_end)
        } else {
            None
        }
    }
}

impl Iterator for FragmentIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_self();
        let remaining = &self.original[self.current_position..];

        if self.current_position < self.original.len() {
            let first_char = remaining.chars().next().unwrap();

            // Quotes
            let (fragment, end) = if first_char == '\'' || first_char == '"' {
                let end = Self::find_end_of_quote(remaining)?;
                (&remaining[1..end], end)
            } else {
                let end = Self::find_end_of_word(remaining);
                (&remaining[..end], end)
            };

            self.current_position += end;

            Some(fragment.to_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fragment::FragmentIterator;

    #[test]
    pub fn test_fragment_iterator() {
        let mut iterator = FragmentIterator::new(String::from("  hey ho 'bl bl' "));

        assert_eq!(iterator.next(), Some(String::from("hey")));
        assert_eq!(iterator.next(), Some(String::from("ho")));
        assert_eq!(iterator.next(), Some(String::from("bl bl")));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    pub fn test_trim_self_normal() {
        let mut it = FragmentIterator::new(String::from("  hey"));
        it.trim_self();
        assert_eq!(it.current_position, 2);
    }

    #[test]
    pub fn test_trim_self_nothing() {
        let mut it = FragmentIterator::new(String::from("hey"));
        it.trim_self();
        assert_eq!(it.current_position, 0);
    }

    #[test]
    pub fn test_end_of_word_normal() {
        let normal = "hey ho";
        assert_eq!(FragmentIterator::find_end_of_word(normal), 3);
    }

    #[test]
    pub fn test_end_of_word_eof() {
        let eof = "hey";
        assert_eq!(FragmentIterator::find_end_of_word(eof), 3);
    }

    #[test]
    pub fn test_end_of_word_whitespace() {
        let whitespace = "hey  ";
        assert_eq!(FragmentIterator::find_end_of_word(whitespace), 3);
    }

    #[test]
    pub fn test_find_ending_quote_valid() {
        let valid = "'this is quoted'";
        assert_eq!(FragmentIterator::find_end_of_quote(valid), Some(15));
    }

    #[test]
    pub fn test_find_ending_quote_invalid() {
        let invalid = "'nope";
        assert_eq!(FragmentIterator::find_end_of_quote(invalid), None);
    }

    #[test]
    pub fn test_find_ending_quote_multiple() {
        let multiple = "'hey' 'ho'";
        assert_eq!(FragmentIterator::find_end_of_quote(multiple), Some(4));
    }
}
