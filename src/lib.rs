//! A super simple `#[no_std]`-compatible character cursor implementation geared towards
//! lexers/tokenizers. The implementation is inspired by the one used in `rustc` and should be
//! performant enough to handle pretty much anything you could throw at it.
//!
//! # Basic use
//! The following examples showcases the basic features of `simple_cursor`. Please refer to
//! [`Cursor`] for more info.
//!
//! ```rust
//! use simple_cursor::Cursor;
//!
//! // Create the input string and the cursor.
//! let input = "123 foobar竜<!>";
//! let mut cursor = Cursor::new(input);
//!
//! // "123"
//! let number_start = cursor.byte_pos();
//! cursor.skip_while(|c| c.is_ascii_digit());
//! let number_end = cursor.byte_pos();
//!
//! // Some(' ')
//! let whitespace = cursor.bump();
//!
//! // "foobar"
//! let ident_start = cursor.byte_pos();
//! cursor.skip_while(|c| c.is_ascii_alphabetic());
//! let ident_end = cursor.byte_pos();
//!
//! // "竜<!>"
//! let rest_start = ident_end;
//! let rest_end = input.len();
//!
//! assert_eq!("123", &input[number_start..number_end]);
//! assert_eq!(Some(' '), whitespace);
//! assert_eq!("foobar", &input[ident_start..ident_end]);
//! assert_eq!("竜<!>", &input[rest_start..rest_end]);
//! ```

#![no_std]

use core::str::Chars;

/// Abstraction over a character iterator.
pub struct Cursor<'a> {
    /// Raw charactor iterator.
    chars: Chars<'a>,
    /// Current byte position of the cursor.
    byte_pos: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new [`Cursor`] from an input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            byte_pos: 0,
        }
    }

    /// Immutable reference to the internal character iterator.
    pub fn chars(&self) -> &Chars<'a> {
        &self.chars
    }

    /// The current byte position of the cursor into the input string.
    pub fn byte_pos(&self) -> usize {
        self.byte_pos
    }

    /// Peeks the next character without advancing the cursor.
    pub fn peek(&self) -> Option<char> {
        // Cloning a [`Chars`] iterator is cheap.
        self.chars.clone().next()
    }

    /// Peeks the next two characters without advancing the cursor.
    pub fn peek_two(&self) -> (Option<char>, Option<char>) {
        // Cloning a [`Chars`] iterator is cheap.
        let mut cloned = self.chars.clone();
        (cloned.next(), cloned.next())
    }

    /// Bumps the cursor and returns the next character.
    pub fn bump(&mut self) -> Option<char> {
        // Bump the character iterator
        let c = self.chars.next();
        // Bump the byte position
        self.byte_pos += c.map(char::len_utf8).unwrap_or_default();
        c
    }

    /// Bumps the cursor and returns the next two characters.
    pub fn bump_two(&mut self) -> (Option<char>, Option<char>) {
        // Bump the character iterator
        let (c1, c2) = (self.chars.next(), self.chars.next());

        // Bump the byte position
        self.byte_pos += c1.map(char::len_utf8).unwrap_or_default();
        self.byte_pos += c2.map(char::len_utf8).unwrap_or_default();

        (c1, c2)
    }

    /// Bumps the cursor while `predicate` is true for the current character.
    ///
    /// Notably, this method will **not** consume the first non-matching character. This is in
    /// contrast with methods like [`Iterator::take_while`]. This behavior is achieved by peeking
    /// the next character to see if it matches before consuming it.
    pub fn skip_while(&mut self, predicate: fn(char) -> bool) {
        // Record the remaining input bytes before skipping
        let start_length = self.chars.as_str().len();
        // Skip while predicate matches (without taking the first non-matching)
        while matches!(self.peek(), Some(c) if predicate(c)) {
            // Notice how this doesn't call [`Cursor::next`] directly.
            // This way we can batch the byte_pos update.
            self.chars.next();
        }
        // Record the remaining input bytes after skipping
        let final_length = self.chars.as_str().len();
        // Bump the byte_pos by how many bytes were skipped
        self.byte_pos += start_length - final_length;
    }
}

#[cfg(test)]
mod tests {
    use super::Cursor;

    #[test]
    fn peek() {
        let input = "s";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.peek(), Some('s'));
        assert_eq!(cursor.byte_pos, 0);
        assert_eq!(cursor.chars.as_str(), input);

        let input = "";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.peek(), None);
        assert_eq!(cursor.byte_pos, 0);
        assert_eq!(cursor.chars.as_str(), input);
    }

    #[test]
    fn peek_two() {
        let input = "ab";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.peek_two(), (Some('a'), Some('b')));
        assert_eq!(cursor.byte_pos, 0);
        assert_eq!(cursor.chars.as_str(), input);

        let input = "a";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.peek_two(), (Some('a'), None));
        assert_eq!(cursor.byte_pos, 0);
        assert_eq!(cursor.chars.as_str(), input);

        let input = "";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.peek_two(), (None, None));
        assert_eq!(cursor.byte_pos, 0);
        assert_eq!(cursor.chars.as_str(), input);
    }

    #[test]
    fn bump() {
        let input = "a";
        let mut cursor = Cursor::new(input);
        assert_eq!(cursor.bump(), Some('a'));
        assert_eq!(cursor.byte_pos, 1);
        assert_eq!(cursor.chars.as_str(), "");
    }

    #[test]
    fn bump_two() {
        let input = "abc";
        let mut cursor = Cursor::new(input);
        assert_eq!(cursor.bump_two(), (Some('a'), Some('b')));
        assert_eq!(cursor.byte_pos, 2);
        assert_eq!(cursor.chars.as_str(), "c");

        assert_eq!(cursor.bump_two(), (Some('c'), None));
        assert_eq!(cursor.byte_pos, 3);
        assert_eq!(cursor.chars.as_str(), "");

        assert_eq!(cursor.bump_two(), (None, None));
        assert_eq!(cursor.byte_pos, 3);
        assert_eq!(cursor.chars.as_str(), "");
    }

    #[test]
    fn skip_while() {
        let input = "aaaab";
        let mut cursor = Cursor::new(input);
        cursor.skip_while(|c| c == 'a');
        assert_eq!(cursor.byte_pos, 4);
        assert_eq!(cursor.chars.as_str(), "b");
    }
}
