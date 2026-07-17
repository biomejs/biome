/// Shared source-only navigation primitive for CSS cursor helpers.
#[derive(Debug, Copy, Clone)]
pub(crate) struct SourceCursor<'src> {
    source: &'src str,
    position: usize,
}

impl<'src> SourceCursor<'src> {
    /// Creates a source cursor at the given byte position.
    pub(crate) const fn new(source: &'src str, position: usize) -> Self {
        Self { source, position }
    }

    /// Returns the backing source text.
    pub(crate) const fn source(self) -> &'src str {
        self.source
    }

    /// Returns the current absolute byte position.
    pub(crate) const fn position(self) -> usize {
        self.position
    }

    /// Sets the current absolute byte position.
    pub(crate) fn set_position(&mut self, position: usize) {
        debug_assert!(position <= self.source.len());
        self.position = position;
    }

    /// Returns the byte at the current position, if any.
    pub(crate) fn current_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.position).copied()
    }

    /// Returns the byte immediately after the current position, if any.
    pub(crate) fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Returns the byte at `position + offset`, if any.
    pub(crate) fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.as_bytes().get(self.position + offset).copied()
    }

    /// Advances the cursor by a known byte count.
    pub(crate) fn advance(&mut self, amount: usize) {
        self.position += amount;
    }

    /// Advances over one ASCII byte or one full UTF-8 code point.
    pub(crate) fn advance_byte_or_char(&mut self, current: u8) {
        if current.is_ascii() {
            self.advance(1);
        } else {
            self.advance(self.current_char().len_utf8());
        }
    }

    /// Returns the character that starts at the current byte position.
    ///
    /// ## Safety
    /// Must be called at a valid non-EOF UTF-8 char boundary.
    pub(crate) fn current_char(&self) -> char {
        self.char_at(0)
    }

    /// Returns the character that starts at `position + offset`.
    ///
    /// ## Safety
    /// The target byte position must be a valid non-EOF UTF-8 char boundary.
    pub(crate) fn char_at(&self, offset: usize) -> char {
        let position = self.position + offset;

        debug_assert!(position < self.source.len());
        debug_assert!(self.source.is_char_boundary(position));

        // SAFETY: The lexer guarantees valid UTF-8 input, and callers only
        // reach this helper after proving `position` is a non-EOF UTF-8
        // boundary.
        let string = unsafe {
            std::str::from_utf8_unchecked(self.source.as_bytes().get_unchecked(position..))
        };

        if let Some(chr) = string.chars().next() {
            chr
        } else {
            // SAFETY: The preconditions above guarantee a character exists
            // here.
            unsafe { core::hint::unreachable_unchecked() }
        }
    }

    /// Returns true when a backslash escape starting at `offset` is valid.
    pub(crate) fn is_valid_escape_at(&self, offset: usize) -> bool {
        self.byte_at(offset)
            .is_some_and(|byte| !is_newline_byte(byte))
    }
}

pub(super) const fn is_newline_byte(byte: u8) -> bool {
    matches!(byte, b'\n' | b'\r' | 0x0C)
}

#[cfg(test)]
mod tests {
    use super::SourceCursor;

    #[test]
    fn char_access_uses_utf8_boundaries() {
        let cursor = SourceCursor::new("aé", 0);

        assert_eq!(cursor.current_char(), 'a');
        assert_eq!(cursor.char_at(1), 'é');
    }

    #[test]
    fn advances_over_non_ascii_code_points() {
        let mut cursor = SourceCursor::new("éx", 0);

        cursor.advance_byte_or_char(b'\xC3');
        assert_eq!(cursor.position(), 2);
        assert_eq!(cursor.current_char(), 'x');
    }

    #[test]
    fn set_position_restores_position_and_byte() {
        let mut cursor = SourceCursor::new("abc", 0);

        cursor.advance(1);
        cursor.advance(1);

        assert_eq!(cursor.position(), 2);
        assert_eq!(cursor.current_byte(), Some(b'c'));

        cursor.set_position(1);

        assert_eq!(cursor.position(), 1);
        assert_eq!(cursor.current_byte(), Some(b'b'));
    }

    #[test]
    fn form_feed_is_not_a_valid_escape_target() {
        let cursor = SourceCursor::new("\\\u{000C}", 0);

        assert!(!cursor.is_valid_escape_at(1));
    }
}
