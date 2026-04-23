use super::{
    CssStringQuote,
    source_cursor::{SourceCursor, is_newline_byte},
};
use biome_css_syntax::{TextRange, TextSize};
use biome_unicode_table::{Dispatch::*, is_css_non_ascii, lookup_byte};
use std::char::REPLACEMENT_CHARACTER;

/// Classification result for the first non-trivia content inside `url(...)`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum UrlBodyStartScan {
    /// Keep lexing in regular mode so the parser can build an SCSS function.
    InterpolatedFunction,
    /// Commit a raw URL literal starting at the recorded non-trivia position.
    RawValue(PendingUrlRawValueScan),
    /// Fall back to regular tokenization.
    Other,
}

/// Byte range for a raw URL literal discovered by speculative scanning.
///
/// The lexer can cache this after consuming leading trivia so it does not need
/// to rescan when it later reaches the actual raw-value start.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct PendingUrlRawValueScan {
    /// Absolute byte position where the non-trivia raw value starts.
    pub(crate) start: usize,
    /// Absolute byte position immediately before the closing `)` or EOF.
    pub(crate) end: usize,
    /// Whether the scan stopped on a closing `)` instead of EOF.
    pub(crate) terminated: bool,
}

/// Result of scanning a string body from the current lexer position.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct StringBodyScan {
    /// Where scanning stopped.
    pub(crate) stop: StringBodyScanStop,
    /// Ranges of invalid escape sequences encountered before the stop
    /// boundary. The caller decides when to emit diagnostics for them.
    pub(crate) invalid_escape_ranges: Vec<TextRange>,
}

/// Next boundary encountered while scanning a string body.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum StringBodyScanStop {
    /// An unescaped `#{` starts an interpolation part.
    Interpolation { position: usize },
    /// The matching quote closes the outer string.
    ClosingQuote { position: usize },
    /// A newline terminates the string as malformed input.
    Newline { position: usize, len: usize },
    /// The file ended before a closing quote was found.
    Eof { position: usize },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InterpolatedIdentifierFragment {
    Identifier,
    Interpolation,
}

/// Shared CSS-aware scanner with low-level source navigation plus CSS-specific
/// scan primitives.
///
/// This layer exposes reusable scan primitives for the real lexer and for
/// non-committing lookahead. It intentionally does not own diagnostics, token
/// kinds, or parser state.
#[derive(Debug, Copy, Clone)]
pub(crate) struct CssScanCursor<'src> {
    cursor: SourceCursor<'src>,
    is_scss: bool,
    line_comments_enabled: bool,
}

/// Result metadata from scanning one identifier sequence with the shared
/// scanner.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct IdentifierSequenceScan {
    /// Number of lowercase ASCII bytes written into the provided buffer before
    /// scanning encountered non-ASCII content or the buffer filled up.
    pub(crate) count: usize,
    /// Whether every consumed identifier fragment decoded to ASCII.
    pub(crate) only_ascii_used: bool,
    /// Whether the most recently consumed fragment was written into the
    /// keyword buffer.
    pub(crate) last_was_buffered: bool,
    /// Absolute byte position immediately after the last consumed fragment.
    pub(crate) position: usize,
    /// First byte that stopped the identifier scan, if any.
    pub(crate) stop_byte: Option<u8>,
}

impl<'src> CssScanCursor<'src> {
    /// Creates a shared scan cursor at `position` with a stable lexical
    /// environment.
    pub(crate) const fn new(
        cursor: SourceCursor<'src>,
        is_scss: bool,
        line_comments_enabled: bool,
    ) -> Self {
        Self {
            cursor,
            is_scss,
            line_comments_enabled,
        }
    }

    /// Returns the current absolute byte position.
    pub(crate) fn position(self) -> usize {
        self.cursor.position()
    }

    /// Returns the byte at the current scan position, if any.
    fn current_byte(&self) -> Option<u8> {
        self.cursor.current_byte()
    }

    /// Returns the byte immediately after the current scan position, if any.
    fn peek_byte(&self) -> Option<u8> {
        self.cursor.peek_byte()
    }

    /// Returns the byte at `position + offset`, if any.
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.cursor.byte_at(offset)
    }

    /// Advances the shared scan cursor by a known byte count.
    fn advance(&mut self, amount: usize) {
        self.cursor.advance(amount);
    }

    /// Advances over one ASCII byte or one full UTF-8 code point.
    fn advance_byte_or_char(&mut self, current: u8) {
        self.cursor.advance_byte_or_char(current);
    }

    /// Returns the character that starts at the current byte position.
    ///
    /// ## Safety
    /// Must be called at a valid non-EOF UTF-8 char boundary.
    fn current_char_at_boundary(&self) -> char {
        self.cursor.current_char()
    }

    /// Returns the character that starts at `position + offset`.
    ///
    /// ## Safety
    /// The target byte position must be a valid non-EOF UTF-8 char boundary.
    pub(crate) fn char_at_boundary(&self, offset: usize) -> char {
        self.cursor.char_at(offset)
    }

    /// Returns true when a backslash escape starting at `offset` is valid.
    fn is_valid_escape_at(&self, offset: usize) -> bool {
        self.cursor.is_valid_escape_at(offset)
    }

    /// Returns the total byte length of the backing source.
    fn source_len(&self) -> usize {
        self.cursor.source().len()
    }

    /// Consumes a CSS hex escape sequence that starts at the current position
    /// and returns the decoded code point.
    pub(crate) fn consume_escape_sequence(&mut self) -> char {
        debug_assert!(
            self.current_byte()
                .is_some_and(|byte| byte.is_ascii_hexdigit()),
            "hex escape scanning must start on a hex digit"
        );

        let mut offset = 0usize;
        let mut hex = 0u32;

        while offset < 6 {
            let Some(byte) = self.byte_at(offset) else {
                break;
            };

            let Some(digit) = (byte as char).to_digit(16) else {
                break;
            };

            hex = hex * 16 + digit;
            offset += 1;
        }

        if self
            .byte_at(offset)
            .is_some_and(|byte| matches!(byte, b'\t' | b' ' | b'\n' | b'\r' | 0x0C))
        {
            offset += 1;
        }

        self.advance(offset);
        decode_escaped_code_point(hex)
    }

    /// Returns true when `position + offset` can start a CSS identifier.
    pub(crate) fn is_ident_start_at(self, offset: usize) -> bool {
        let Some(current) = self.byte_at(offset) else {
            return false;
        };

        match lookup_byte(current) {
            MIN => {
                let Some(next) = self.byte_at(offset + 1) else {
                    return false;
                };

                match lookup_byte(next) {
                    MIN => {
                        let Some(next) = self.byte_at(offset + 2) else {
                            return false;
                        };

                        match lookup_byte(next) {
                            IDT | MIN | DIG | ZER => true,
                            UNI => is_css_non_ascii(self.char_at_boundary(offset + 2)),
                            BSL => self.is_valid_escape_at(offset + 3),
                            MUL => true,
                            _ => false,
                        }
                    }
                    IDT => true,
                    UNI => is_css_non_ascii(self.char_at_boundary(offset + 1)),
                    BSL => self.is_valid_escape_at(offset + 2),
                    _ => false,
                }
            }
            IDT => true,
            UNI => is_css_non_ascii(self.char_at_boundary(offset)),
            BSL => self.is_valid_escape_at(offset + 1),
            _ => false,
        }
    }

    /// Returns true when the current position can start a CSS identifier.
    pub(crate) fn is_ident_start(self) -> bool {
        self.is_ident_start_at(0)
    }

    fn consume_ident_part(&mut self, allow_slash: bool) -> Option<char> {
        if allow_slash && self.current_byte() == Some(b'/') {
            self.advance(1);
            return Some('/');
        }

        let current = self.current_byte()?;

        match lookup_byte(current) {
            IDT | MIN | DIG | ZER => {
                self.advance(1);
                Some(current as char)
            }
            UNI => {
                let chr = self.current_char_at_boundary();
                if is_css_non_ascii(chr) {
                    self.advance(chr.len_utf8());
                    Some(chr)
                } else {
                    None
                }
            }
            BSL if self.is_valid_escape_at(1) => {
                self.advance(1);

                match self.current_byte() {
                    Some(byte) if byte.is_ascii_hexdigit() => Some(self.consume_escape_sequence()),
                    Some(_) => {
                        let chr = self.current_char_at_boundary();
                        self.advance(chr.len_utf8());
                        Some(chr)
                    }
                    None => None,
                }
            }
            _ => None,
        }
    }

    /// Consumes standard CSS identifier fragments until the current position
    /// no longer matches the identifier grammar.
    ///
    /// This is the shared hot-path entrypoint for lexer identifier scanning:
    /// it advances one scan cursor across the whole token while optionally
    /// lowercasing ASCII content into `buf` for keyword matching.
    pub(crate) fn consume_ident_sequence(&mut self, buf: &mut [u8]) -> IdentifierSequenceScan {
        self.scan_ident_sequence(Some(buf), false)
    }

    /// Consumes identifier fragments, also accepting `/` for slash-enabled
    /// utility names such as Tailwind `w/2`.
    pub(crate) fn consume_ident_sequence_with_slash(
        &mut self,
        buf: &mut [u8],
    ) -> IdentifierSequenceScan {
        self.scan_ident_sequence(Some(buf), true)
    }

    /// Advances through one standard CSS identifier sequence without
    /// collecting lexer keyword-buffer bookkeeping.
    pub(crate) fn advance_ident_sequence(&mut self) {
        let _ = self.scan_ident_sequence(None, false);
    }

    fn scan_ident_sequence(
        &mut self,
        mut buf: Option<&mut [u8]>,
        allow_slash: bool,
    ) -> IdentifierSequenceScan {
        let mut idx = 0;
        let mut only_ascii_used = true;
        let mut last_was_buffered = false;

        while self.current_byte().is_some() {
            let start = self.position();

            let Some(part) = self.consume_ident_part(allow_slash) else {
                break;
            };

            if only_ascii_used && !part.is_ascii() {
                only_ascii_used = false;
            }

            // Once a non-ASCII fragment appears, the lexer can no longer use
            // the lowercase ASCII buffer for keyword matching, but the scan
            // still needs to advance through the whole identifier.
            last_was_buffered = false;
            if only_ascii_used
                && let Some(buf) = buf.as_deref_mut()
                && let Some(slot) = buf.get_mut(idx)
            {
                *slot = part.to_ascii_lowercase() as u8;
                idx += 1;
                // The lexer's Tailwind `-*` fixup may later need to know
                // whether this last fragment actually entered the buffer.
                last_was_buffered = true;
            }

            debug_assert!(
                self.position() > start,
                "identifier-part consumption must advance the scan cursor",
            );
        }

        IdentifierSequenceScan {
            count: idx,
            only_ascii_used,
            last_was_buffered,
            position: self.position(),
            stop_byte: self.current_byte(),
        }
    }

    /// Consumes a string escape after `\` and returns whether it produced an
    /// invalid hex escape that should later surface as a diagnostic.
    fn consume_string_escape(&mut self, quote: CssStringQuote) -> bool {
        debug_assert!(self.current_byte() == Some(b'\\'));

        self.advance(1);

        match self.current_byte() {
            Some(byte) if is_newline_byte(byte) => {
                let len = if byte == b'\r' && self.peek_byte() == Some(b'\n') {
                    2
                } else {
                    1
                };
                self.advance(len);
                false
            }
            Some(byte) if byte == quote.as_byte() => {
                self.advance(1);
                false
            }
            Some(byte) if byte.is_ascii_hexdigit() => {
                self.consume_escape_sequence() == REPLACEMENT_CHARACTER
            }
            Some(byte) if byte.is_ascii() => {
                self.advance(1);
                false
            }
            Some(_) => {
                let chr = self.current_char_at_boundary();
                self.advance(chr.len_utf8());
                false
            }
            None => false,
        }
    }

    /// Consumes a quoted string, including nested escapes, until the matching
    /// closing quote or EOF.
    fn consume_string(&mut self, quote: CssStringQuote) {
        let quote_byte = quote.as_byte();
        self.advance(1);

        while let Some(current) = self.current_byte() {
            match current {
                b'\\' => {
                    self.consume_string_escape(quote);
                }
                _ if current == quote_byte => {
                    self.advance(1);
                    return;
                }
                _ => self.advance_byte_or_char(current),
            }
        }
    }

    /// Consumes a CSS block comment, including SCSS nested block comments when
    /// SCSS mode is enabled.
    fn consume_block_comment(&mut self) {
        debug_assert!(self.current_byte() == Some(b'/') && self.peek_byte() == Some(b'*'));
        self.advance(2);
        let mut depth = 1usize;

        while let Some(current) = self.current_byte() {
            match current {
                b'/' if self.is_scss && self.peek_byte() == Some(b'*') => {
                    self.advance(2);
                    depth += 1;
                }
                b'*' if self.peek_byte() == Some(b'/') => {
                    self.advance(2);
                    depth -= 1;

                    if !self.is_scss || depth == 0 {
                        return;
                    }
                }
                _ => self.advance_byte_or_char(current),
            }
        }
    }

    /// Consumes an SCSS/CSS line comment and returns whether it terminated on a
    /// newline before EOF.
    fn consume_line_comment(&mut self) -> bool {
        debug_assert!(self.current_byte() == Some(b'/') && self.peek_byte() == Some(b'/'));
        self.advance(2);

        while let Some(current) = self.current_byte() {
            match current {
                b'\n' | b'\r' => return true,
                _ => self.advance_byte_or_char(current),
            }
        }

        false
    }

    /// Returns true when the current position starts a line comment under the
    /// active lexical configuration.
    fn is_at_line_comment(self) -> bool {
        self.line_comments_enabled
            && self.current_byte() == Some(b'/')
            && self.peek_byte() == Some(b'/')
    }

    /// Skips whitespace and block comments while leaving line comments for
    /// callers that need custom `//` handling.
    fn skip_whitespace_and_block_comments(&mut self) {
        loop {
            let start = self.position();

            while let Some(byte) = self.current_byte() {
                if matches!(byte, b'\t' | b' ' | b'\n' | b'\r' | 0x0C) {
                    self.advance(1);
                } else {
                    break;
                }
            }

            if self.current_byte() == Some(b'/') && self.peek_byte() == Some(b'*') {
                self.consume_block_comment();
            }

            if self.position() == start {
                return;
            }
        }
    }

    /// Skips URL-leading trivia while preserving protocol-relative raw URLs
    /// such as `url(//cdn.example.com/app.css)`.
    fn skip_url_body_trivia(&mut self) {
        loop {
            self.skip_whitespace_and_block_comments();

            if !self.is_at_line_comment() {
                return;
            }

            let mut after_comment = *self;
            // `url(//cdn.example.com/app.css)` is a valid protocol-relative raw
            // URL, so only treat `//...` as trivia when the comment actually
            // terminates before the URL body continues.
            if !after_comment.consume_line_comment() {
                return;
            }

            *self = after_comment;
        }
    }

    /// Returns true when `position + offset` starts an SCSS interpolation.
    pub(crate) fn is_at_scss_interpolation_at(self, offset: usize) -> bool {
        self.byte_at(offset) == Some(b'#') && self.byte_at(offset + 1) == Some(b'{')
    }

    /// Returns true when the current position starts an SCSS interpolation.
    pub(crate) fn is_at_scss_interpolation(self) -> bool {
        self.is_at_scss_interpolation_at(0)
    }

    /// Consumes a full SCSS interpolation body, including nested
    /// interpolations, strings, and comments.
    fn consume_scss_interpolation(&mut self) -> bool {
        if !self.is_at_scss_interpolation() {
            return false;
        }

        self.advance(2);
        let mut brace_depth = 1usize;

        while let Some(current) = self.current_byte() {
            match current {
                b'\'' => self.consume_string(CssStringQuote::Single),
                b'"' => self.consume_string(CssStringQuote::Double),
                b'/' if self.peek_byte() == Some(b'*') => self.consume_block_comment(),
                b'/' if self.line_comments_enabled && self.peek_byte() == Some(b'/') => {
                    self.consume_line_comment();
                }
                b'#' if self.peek_byte() == Some(b'{') => {
                    self.advance(2);
                    brace_depth += 1;
                }
                b'{' => {
                    self.advance(1);
                    brace_depth += 1;
                }
                b'}' => {
                    self.advance(1);
                    brace_depth -= 1;

                    if brace_depth == 0 {
                        return true;
                    }
                }
                _ => self.advance_byte_or_char(current),
            }
        }

        false
    }

    /// Scans a plain string body until a closing quote, newline, or EOF.
    pub(crate) fn scan_plain_string_body(self, quote: CssStringQuote) -> StringBodyScan {
        ScssStringScanner { cursor: self }.scan_plain_body(quote)
    }

    /// Scans an interpolation-aware string body until a closing quote,
    /// interpolation boundary, newline, or EOF.
    pub(crate) fn scan_interpolated_string_body(self, quote: CssStringQuote) -> StringBodyScan {
        ScssStringScanner { cursor: self }.scan_interpolated_body(quote)
    }

    /// Returns true when this cursor begins an interpolation-containing
    /// identifier immediately followed by `(`.
    pub(crate) fn is_at_scss_interpolated_function(self) -> bool {
        ScssInterpolatedIdentifierScanner::new(self).is_at_function()
    }

    /// Classifies the first non-trivia content in a URL body as raw URL text,
    /// interpolated function syntax, or ordinary tokenization.
    pub(crate) fn scan_url_body_start(
        self,
        scss_exclusive_syntax_allowed: bool,
    ) -> UrlBodyStartScan {
        UrlBodyScanner::new(self).scan_url_body_start(scss_exclusive_syntax_allowed)
    }

    /// Scans a raw URL literal starting at the current position, if one can
    /// begin here.
    pub(crate) fn scan_url_raw_value(self) -> Option<PendingUrlRawValueScan> {
        UrlBodyScanner::new(self).scan_url_raw_value()
    }
}

/// Scans the body of a quoted string without committing lexer state.
///
/// This helper centralizes the shared stop conditions for plain CSS strings and
/// SCSS strings that may suspend on interpolation boundaries.
#[derive(Debug, Copy, Clone)]
struct ScssStringScanner<'src> {
    cursor: CssScanCursor<'src>,
}

impl<'src> ScssStringScanner<'src> {
    /// Scans a plain string body until the string closes, hits a newline, or
    /// reaches EOF, while collecting invalid escape diagnostics as data.
    fn scan_plain_body(self, quote: CssStringQuote) -> StringBodyScan {
        self.scan_body(quote, false)
    }

    /// Scans an interpolation-aware string body until the string closes, hits
    /// interpolation, hits a newline, or reaches EOF.
    fn scan_interpolated_body(self, quote: CssStringQuote) -> StringBodyScan {
        self.scan_body(quote, true)
    }

    fn scan_body(mut self, quote: CssStringQuote, stop_at_interpolation: bool) -> StringBodyScan {
        let mut invalid_escape_ranges = Vec::new();
        let quote_byte = quote.as_byte();

        while let Some(byte) = self.cursor.current_byte() {
            let position = self.cursor.position();

            if byte == quote_byte {
                return StringBodyScan {
                    stop: StringBodyScanStop::ClosingQuote { position },
                    invalid_escape_ranges,
                };
            }

            if stop_at_interpolation && self.cursor.is_at_scss_interpolation() {
                return StringBodyScan {
                    stop: StringBodyScanStop::Interpolation { position },
                    invalid_escape_ranges,
                };
            }

            match lookup_byte(byte) {
                BSL => {
                    let escape_start = self.cursor.position();
                    if self.cursor.consume_string_escape(quote) {
                        invalid_escape_ranges.push(TextRange::new(
                            TextSize::from(escape_start as u32),
                            TextSize::from(self.cursor.position() as u32),
                        ));
                    }
                }
                WHS if is_newline_byte(byte) => {
                    let len = if byte == b'\r' && self.cursor.peek_byte() == Some(b'\n') {
                        2
                    } else {
                        1
                    };

                    return StringBodyScan {
                        stop: StringBodyScanStop::Newline { position, len },
                        invalid_escape_ranges,
                    };
                }
                _ => self.cursor.advance_byte_or_char(byte),
            }
        }

        StringBodyScan {
            stop: StringBodyScanStop::Eof {
                position: self.cursor.source_len(),
            },
            invalid_escape_ranges,
        }
    }
}

/// Scans identifier-like sequences that may mix ordinary identifier fragments
/// with SCSS interpolation parts.
///
/// The main consumer is function-head classification for forms such as
/// `foo#{1 + 1}(bar)` and `#{name}(bar)`.
#[derive(Debug, Copy, Clone)]
struct ScssInterpolatedIdentifierScanner<'src> {
    cursor: CssScanCursor<'src>,
}

impl<'src> ScssInterpolatedIdentifierScanner<'src> {
    /// Creates a speculative scanner over the provided shared scan cursor.
    const fn new(cursor: CssScanCursor<'src>) -> Self {
        Self { cursor }
    }

    /// Returns true when the current position starts an interpolation-containing
    /// identifier immediately followed by `(`.
    fn is_at_function(mut self) -> bool {
        matches!(
            self.scan_interpolated_identifier(),
            Some(InterpolatedIdentifierFragment::Interpolation)
        ) && self.cursor.current_byte() == Some(b'(')
    }

    /// Consumes an interpolation-capable identifier and reports whether any of
    /// its fragments came from interpolation.
    fn scan_interpolated_identifier(&mut self) -> Option<InterpolatedIdentifierFragment> {
        let first = self.scan_identifier_fragment()?;
        let mut saw_interpolation = matches!(first, InterpolatedIdentifierFragment::Interpolation);

        loop {
            if self.is_at_identifier_hyphen() {
                self.cursor.advance(1);
            } else if !self.is_at_identifier_fragment() {
                break;
            }

            let fragment_start = self.cursor.position();
            let fragment = self.scan_identifier_fragment()?;
            debug_assert!(
                self.cursor.position() > fragment_start,
                "interpolated-identifier fragment scanning must advance the scan cursor",
            );
            saw_interpolation |= matches!(fragment, InterpolatedIdentifierFragment::Interpolation);
        }

        if saw_interpolation {
            Some(InterpolatedIdentifierFragment::Interpolation)
        } else {
            Some(InterpolatedIdentifierFragment::Identifier)
        }
    }

    /// Consumes one identifier fragment, either a normal identifier piece or a
    /// full SCSS interpolation.
    fn scan_identifier_fragment(&mut self) -> Option<InterpolatedIdentifierFragment> {
        if self.cursor.is_at_scss_interpolation() {
            self.cursor
                .consume_scss_interpolation()
                .then_some(InterpolatedIdentifierFragment::Interpolation)
        } else if self.cursor.is_ident_start() {
            self.cursor.advance_ident_sequence();
            Some(InterpolatedIdentifierFragment::Identifier)
        } else {
            None
        }
    }

    /// Returns true when `position + offset` can continue an
    /// interpolation-capable identifier.
    fn is_at_identifier_fragment_at(&self, offset: usize) -> bool {
        self.cursor.is_at_scss_interpolation_at(offset) || self.cursor.is_ident_start_at(offset)
    }

    /// Returns true when the current position can continue an
    /// interpolation-capable identifier.
    fn is_at_identifier_fragment(&self) -> bool {
        self.is_at_identifier_fragment_at(0)
    }

    /// Returns true when the current `-` should stay attached to the
    /// interpolated identifier instead of being left for surrounding syntax.
    fn is_at_identifier_hyphen(&self) -> bool {
        if self.cursor.current_byte() != Some(b'-') {
            return false;
        }

        // Only treat `-` as part of the interpolated identifier when another
        // identifier fragment follows it. This keeps trailing `-` boundaries
        // available for surrounding syntax such as operators or line breaks.
        self.is_at_identifier_fragment_at(1)
    }
}

/// Classifies the first non-trivia token in a URL body.
///
/// CSS `url(...)` bodies are normally raw literals, but SCSS can place
/// interpolation-containing function calls there, such as
/// `url(foo#{1 + 1}(bar))`. This scanner decides whether the real lexer should
/// emit a raw URL literal or stay on regular tokens so the parser can build the
/// function call.
#[derive(Debug, Copy, Clone)]
struct UrlBodyScanner<'src> {
    cursor: CssScanCursor<'src>,
}

impl<'src> UrlBodyScanner<'src> {
    /// Creates a speculative URL-body scanner over the shared scan cursor.
    const fn new(cursor: CssScanCursor<'src>) -> Self {
        Self { cursor }
    }

    /// Consumes a raw-URL escape after `\`, preserving UTF-8 boundaries for
    /// escaped non-ASCII characters.
    fn consume_url_escape(&mut self) {
        debug_assert!(self.cursor.current_byte() == Some(b'\\'));
        self.cursor.advance(1);

        match self.cursor.current_byte() {
            Some(byte) if byte.is_ascii() => self.cursor.advance(1),
            // URL raw values are otherwise byte-oriented, but escaped
            // non-ASCII characters must still advance on a UTF-8 boundary so
            // the next loop iteration does not slice in the middle of a code
            // point.
            Some(current) => self.cursor.advance_byte_or_char(current),
            None => {}
        }
    }

    /// Scans a raw URL literal from the current position until `)` or EOF.
    fn scan_url_raw_value(&mut self) -> Option<PendingUrlRawValueScan> {
        let current = self.cursor.current_byte()?;
        let dispatch = lookup_byte(current);

        // Reuse the lexer's "can a raw URL literal start here?" classification
        // before switching into byte-oriented URL scanning.
        if !matches!(
            dispatch,
            IDT | DOL | UNI | PRD | SLH | ZER | DIG | TLD | HAS
        ) {
            return None;
        }

        let start = self.cursor.position();

        while let Some(current) = self.cursor.current_byte() {
            match lookup_byte(current) {
                PNC => {
                    return Some(PendingUrlRawValueScan {
                        start,
                        end: self.cursor.position(),
                        terminated: true,
                    });
                }
                BSL if self.cursor.is_valid_escape_at(1) => self.consume_url_escape(),
                _ => self.cursor.advance_byte_or_char(current),
            }
        }

        Some(PendingUrlRawValueScan {
            start,
            end: self.cursor.position(),
            terminated: false,
        })
    }

    /// Classifies the first non-trivia URL-body content for the real lexer.
    fn scan_url_body_start(&mut self, scss_exclusive_syntax_allowed: bool) -> UrlBodyStartScan {
        self.cursor.skip_url_body_trivia();

        if scss_exclusive_syntax_allowed && self.cursor.is_at_scss_interpolated_function() {
            UrlBodyStartScan::InterpolatedFunction
        } else {
            self.scan_url_raw_value()
                .map_or(UrlBodyStartScan::Other, UrlBodyStartScan::RawValue)
        }
    }
}

/// Decodes a CSS hex escape sequence into a Unicode scalar value or the
/// replacement character when the escape is invalid.
#[inline]
fn decode_escaped_code_point(hex: u32) -> char {
    match hex {
        0 => REPLACEMENT_CHARACTER,
        55_296..=57_343 => REPLACEMENT_CHARACTER,
        1_114_112.. => REPLACEMENT_CHARACTER,
        _ => char::from_u32(hex).unwrap_or(REPLACEMENT_CHARACTER),
    }
}
