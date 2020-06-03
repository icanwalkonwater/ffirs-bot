use crate::error::CmdError::ParsingError;
use crate::error::CmdResult;

pub struct FragmentIter {
    original: String,
    current_position: usize,
}

impl FragmentIter {
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

impl Iterator for FragmentIter {
    type Item = CmdResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_self();
        let remaining = &self.original[self.current_position..];

        if self.current_position < self.original.len() {
            let first_char = remaining.chars().next().unwrap();

            // Quotes
            let (fragment, end) = if first_char == '\'' || first_char == '"' {
                let end = Self::find_end_of_quote(remaining);

                if let None = end {
                    let start = self.current_position;
                    self.current_position = self.original.len();
                    return Some(Err(ParsingError {
                        message: String::from("Can't find closing quote."),
                        start,
                        end: self.original.len() - 1,
                    }));
                }

                (&remaining[1..end.unwrap()], end.unwrap() + 1)
            } else {
                let end = Self::find_end_of_word(remaining);
                (&remaining[..end], end)
            };

            self.current_position += end;

            Some(Ok(fragment.to_owned()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::CmdError;
    use crate::fragment_iter::FragmentIter;

    #[test]
    pub fn test_fragment_iterator() {
        let mut iterator = FragmentIter::new(String::from("  hey ho 'bl bl' "));

        assert_eq!(iterator.next().unwrap().unwrap(), String::from("hey"));
        assert_eq!(iterator.next().unwrap().unwrap(), String::from("ho"));
        assert_eq!(iterator.next().unwrap().unwrap(), String::from("bl bl"));
        assert!(iterator.next().is_none());
    }

    #[test]
    pub fn test_fragment_collect() {
        let iterator = FragmentIter::new(String::from("part one     and two 'three hey'"));
        let frags = iterator.collect::<Vec<_>>();
        let err = frags.iter().find(|res| res.is_err());
        assert!(err.is_none());
        let frags = frags
            .into_iter()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();

        assert_eq!(frags.len(), 5);
        assert_eq!(&frags, &["part", "one", "and", "two", "three hey"]);
    }

    #[test]
    pub fn test_fragment_collect_err() {
        let iterator = FragmentIter::new(String::from("part one     and two 'three hey"));
        let frags = iterator.collect::<Vec<_>>();
        let err = frags.iter().find(|res| res.is_err());

        assert!(err.is_some());
        let err = err.unwrap().as_ref().unwrap_err();
        match err {
            CmdError::ParsingError { start, end, .. } => {
                assert_eq!(*start, 21);
                assert_eq!(*end, 30);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    pub fn test_trim_self_normal() {
        let mut it = FragmentIter::new(String::from("  hey"));
        it.trim_self();
        assert_eq!(it.current_position, 2);
    }

    #[test]
    pub fn test_trim_self_nothing() {
        let mut it = FragmentIter::new(String::from("hey"));
        it.trim_self();
        assert_eq!(it.current_position, 0);
    }

    #[test]
    pub fn test_end_of_word_normal() {
        let normal = "hey ho";
        assert_eq!(FragmentIter::find_end_of_word(normal), 3);
    }

    #[test]
    pub fn test_end_of_word_eof() {
        let eof = "hey";
        assert_eq!(FragmentIter::find_end_of_word(eof), 3);
    }

    #[test]
    pub fn test_end_of_word_whitespace() {
        let whitespace = "hey  ";
        assert_eq!(FragmentIter::find_end_of_word(whitespace), 3);
    }

    #[test]
    pub fn test_find_ending_quote_valid() {
        let valid = "'this is quoted'";
        assert_eq!(FragmentIter::find_end_of_quote(valid), Some(15));
    }

    #[test]
    pub fn test_find_ending_quote_invalid() {
        let invalid = "'nope";
        assert_eq!(FragmentIter::find_end_of_quote(invalid), None);
    }

    #[test]
    pub fn test_find_ending_quote_multiple() {
        let multiple = "'hey' 'ho'";
        assert_eq!(FragmentIter::find_end_of_quote(multiple), Some(4));
    }

    #[test]
    pub fn test_find_ending_quote_spaced() {
        let not_trimmed = "'hey'  ";
        assert_eq!(FragmentIter::find_end_of_quote(not_trimmed), Some(4));
    }

    #[test]
    pub fn test_find_ending_quote_invalid2() {
        let invalid = "'nope hey";
        assert_eq!(FragmentIter::find_end_of_quote(invalid), None);
    }
}
