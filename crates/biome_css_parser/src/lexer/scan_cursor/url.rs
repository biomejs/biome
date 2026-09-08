use super::CssScanCursor;
use crate::lexer::{CssLexContext, UrlRawValueScan};
use biome_css_syntax::is_css_whitespace_byte;
use biome_unicode_table::{Dispatch::*, lookup_byte};

/// Result of scanning the body of `url(images/#{$name}.png)`.
enum ScssRawUrlBodyScan {
    /// Reached `)` and records whether the body contained `#{$name}`.
    Complete { has_interpolation: bool },
    /// Found an unclosed interpolation, as in `url(images/#{$name.png)`.
    MalformedInterpolation,
    /// Found non-URL syntax, as in `url($path + ".png")`.
    Invalid,
}

/// Result of scanning an interpolated identifier at the start of `url(...)`.
enum ScssUrlHeadScan {
    /// Found a function head, as in `url(foo#{$name}(bar))`.
    Function,
    /// Found URL text, as in `url(foo#{$name}.png)`.
    Raw { has_interpolation: bool },
    /// Found an unclosed interpolation, as in `url(foo#{$name.png)`.
    MalformedInterpolation,
}

impl<'src> CssScanCursor<'src> {
    /// Returns the lexer context for raw, structured SCSS, or regular URL
    /// tokenization.
    pub(crate) fn url_body_lex_context(self, scss_exclusive_syntax_allowed: bool) -> CssLexContext {
        UrlBodyScanner::new(self).lex_context(scss_exclusive_syntax_allowed)
    }

    /// Returns whether a complete `url(...)` body uses Sass raw-URL syntax.
    ///
    /// `url(//cdn.example/app.css)` is raw, while
    /// `url(// comment\napp.css)` falls back to function parsing.
    pub(crate) fn is_scss_raw_url_body(self) -> bool {
        UrlBodyScanner::new(self).is_scss_raw_url_body()
    }

    /// Returns the position before the next SCSS URL-value boundary.
    pub(crate) fn scan_scss_url_value_chunk(self) -> usize {
        UrlBodyScanner::new(self).scan_scss_url_value_chunk()
    }
}

/// Classifies a URL body before the lexer emits its first body token.
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
    /// Creates a non-consuming URL-body scanner over the shared scan cursor.
    const fn new(cursor: CssScanCursor<'src>) -> Self {
        Self { cursor }
    }

    /// Classifies the first non-trivia URL-body content for the real lexer.
    ///
    /// Examples:
    /// ```scss
    /// url(fudge#{$x}.css)
    /// url(foo#{$name}(bar))
    /// url($path + ".css")
    /// ```
    fn lex_context(&mut self, scss_exclusive_syntax_allowed: bool) -> CssLexContext {
        if !scss_exclusive_syntax_allowed {
            // CSS raw URLs treat comments as body text. Only whitespace may be
            // emitted as trivia before the precomputed raw token starts.
            self.cursor.skip_whitespace();
            return self.lex_context_for_value(false);
        }

        self.cursor.skip_url_body_trivia();

        if self.is_at_scss_url_variable_concatenation() {
            CssLexContext::Regular
        } else {
            self.lex_context_for_value(true)
        }
    }

    /// Returns whether the body can be consumed as Sass raw URL contents.
    ///
    /// Internal whitespace is only valid immediately before `)`; otherwise
    /// Sass parses the body as an ordinary function expression.
    fn is_scss_raw_url_body(&mut self) -> bool {
        self.cursor.skip_whitespace();

        matches!(
            self.scan_scss_raw_url_body(false),
            ScssRawUrlBodyScan::Complete { .. }
        )
    }

    fn scan_scss_raw_url_body(&mut self, mut has_interpolation: bool) -> ScssRawUrlBodyScan {
        while let Some(current) = self.cursor.current_byte() {
            match current {
                b')' => {
                    return ScssRawUrlBodyScan::Complete { has_interpolation };
                }
                b'\\' => {
                    if !self.cursor.is_valid_escape_at(1) {
                        break;
                    }
                    self.consume_url_escape();
                }
                b'#' if self.cursor.peek_byte() == Some(b'{') => {
                    if !self.consume_complete_scss_url_interpolation() {
                        return ScssRawUrlBodyScan::MalformedInterpolation;
                    }

                    has_interpolation = true;
                }
                _ if is_css_whitespace_byte(current) => {
                    self.cursor.skip_whitespace();
                    if self.cursor.current_byte() == Some(b')') {
                        return ScssRawUrlBodyScan::Complete { has_interpolation };
                    }
                    break;
                }
                _ if Self::is_scss_raw_url_byte(current) => {
                    self.cursor.advance_byte_or_char(current);
                }
                _ => break,
            }
        }

        ScssRawUrlBodyScan::Invalid
    }

    /// Detects `$name + ...` URL bodies before raw URL lexing can consume them.
    ///
    /// Examples:
    /// ```scss
    /// url($path + ".css")
    /// url($path /* c */ + ".css")
    /// ```
    fn is_at_scss_url_variable_concatenation(&self) -> bool {
        let mut cursor = self.cursor;

        if cursor.current_byte() != Some(b'$') {
            return false;
        }

        cursor.advance(1);

        if !cursor.is_ident_start() {
            return false;
        }

        cursor.advance_ident_sequence();
        cursor.skip_scss_expression_trivia();
        cursor.current_byte() == Some(b'+')
    }

    /// Classifies a URL body as raw text or an interpolated SCSS function call.
    ///
    /// Examples:
    /// ```scss
    /// url(foo#{$name}(bar))
    /// url(fudge#{$x}.css)
    /// ```
    fn lex_context_for_value(&mut self, scss_exclusive_syntax_allowed: bool) -> CssLexContext {
        let Some(current) = self.cursor.current_byte() else {
            return CssLexContext::Regular;
        };

        if !Self::is_url_raw_value_start(current) {
            return CssLexContext::Regular;
        }

        let start = self.cursor.position();

        if scss_exclusive_syntax_allowed {
            let has_interpolation = match self.scan_scss_url_head() {
                ScssUrlHeadScan::Function => return CssLexContext::Regular,
                ScssUrlHeadScan::Raw { has_interpolation } => has_interpolation,
                ScssUrlHeadScan::MalformedInterpolation => {
                    return CssLexContext::UrlRawValue(
                        self.scan_url_raw_value_after_malformed_interpolation(start),
                    );
                }
            };

            match self.scan_scss_raw_url_body(has_interpolation) {
                ScssRawUrlBodyScan::Complete {
                    has_interpolation: true,
                } => {
                    return CssLexContext::ScssUrlValue { start };
                }
                ScssRawUrlBodyScan::Complete {
                    has_interpolation: false,
                } => {
                    return CssLexContext::UrlRawValue(UrlRawValueScan {
                        start,
                        end: self.cursor.position(),
                        terminated: true,
                    });
                }
                ScssRawUrlBodyScan::MalformedInterpolation => {
                    return CssLexContext::UrlRawValue(
                        self.scan_url_raw_value_after_malformed_interpolation(start),
                    );
                }
                ScssRawUrlBodyScan::Invalid => {}
            }
        }

        CssLexContext::UrlRawValue(
            self.scan_url_raw_value_from_current(start, scss_exclusive_syntax_allowed),
        )
    }

    /// Continues raw URL scanning after the caller has already fixed the token start.
    ///
    /// Example: `url(fudge#{$x}.css)` may resume at `.css`, but the raw token
    /// still starts at `fudge`.
    fn scan_url_raw_value_from_current(
        &mut self,
        start: usize,
        scss_exclusive_syntax_allowed: bool,
    ) -> UrlRawValueScan {
        while let Some(current) = self.cursor.current_byte() {
            if scss_exclusive_syntax_allowed && self.cursor.is_at_scss_interpolation() {
                // `url(foo#{bar("x")}.css)`: an inner `)` belongs to the
                // interpolation and must not terminate the raw URL token.
                if self.consume_complete_scss_url_interpolation() {
                    continue;
                }
            }

            match lookup_byte(current) {
                PNC => {
                    return UrlRawValueScan {
                        start,
                        end: self.cursor.position(),
                        terminated: true,
                    };
                }
                BSL if self.cursor.is_valid_escape_at(1) => self.consume_url_escape(),
                _ => self.cursor.advance_byte_or_char(current),
            }
        }

        UrlRawValueScan {
            start,
            end: self.cursor.position(),
            terminated: false,
        }
    }

    fn scan_url_raw_value_after_malformed_interpolation(
        &mut self,
        start: usize,
    ) -> UrlRawValueScan {
        self.cursor.advance(1);
        self.scan_url_raw_value_from_current(start, true)
    }

    fn scan_scss_url_value_chunk(&mut self) -> usize {
        while let Some(current) = self.cursor.current_byte() {
            match current {
                b')' => break,
                b'#' if self.cursor.peek_byte() == Some(b'{') => break,
                b'\\' if self.cursor.is_valid_escape_at(1) => self.consume_url_escape(),
                _ if is_css_whitespace_byte(current) => break,
                _ => self.cursor.advance_byte_or_char(current),
            }
        }

        self.cursor.position()
    }

    /// Scans an identifier-shaped URL head and classifies its continuation.
    ///
    /// Examples:
    /// ```scss
    /// url(foo#{$name}(bar))
    /// url(fudge#{$x}.css)
    /// ```
    fn scan_scss_url_head(&mut self) -> ScssUrlHeadScan {
        if !self.is_at_scss_identifier_fragment() {
            return ScssUrlHeadScan::Raw {
                has_interpolation: false,
            };
        }

        let mut has_interpolation = false;

        loop {
            if self.is_at_scss_identifier_hyphen() {
                self.cursor.advance(1);
            } else if self.cursor.is_at_scss_interpolation() {
                if !self.consume_complete_scss_url_interpolation() {
                    return ScssUrlHeadScan::MalformedInterpolation;
                }

                has_interpolation = true;
            } else if self.cursor.is_ident_start() {
                self.cursor.advance_ident_sequence();
            } else {
                break;
            }
        }

        if has_interpolation && self.cursor.current_byte() == Some(b'(') {
            ScssUrlHeadScan::Function
        } else {
            ScssUrlHeadScan::Raw { has_interpolation }
        }
    }

    /// Consumes `#{$name}` in `url(foo#{$name}.png)` without moving on failure.
    fn consume_complete_scss_url_interpolation(&mut self) -> bool {
        let mut cursor = self.cursor;
        if !cursor.consume_scss_interpolation_in_raw_url() {
            return false;
        }

        self.cursor = cursor;
        true
    }

    /// Consumes a raw-URL escape after `\`, preserving UTF-8 boundaries for
    /// escaped non-ASCII characters.
    fn consume_url_escape(&mut self) {
        debug_assert!(self.cursor.current_byte() == Some(b'\\'));
        self.cursor.advance(1);

        match self.cursor.current_byte() {
            Some(byte) if byte.is_ascii_hexdigit() => {
                self.cursor.consume_escape_sequence();
            }
            Some(byte) if byte.is_ascii() => self.cursor.advance(1),
            // URL raw values are otherwise byte-oriented, but escaped
            // non-ASCII characters must still advance on a UTF-8 boundary so
            // the next loop iteration does not slice in the middle of a code
            // point.
            Some(current) => self.cursor.advance_byte_or_char(current),
            None => {}
        }
    }

    /// Returns true for bytes that may begin raw URL body text.
    ///
    /// Example: `url(foo.css)`.
    fn is_url_raw_value_start(byte: u8) -> bool {
        matches!(
            lookup_byte(byte),
            IDT | DOL | UNI | PRD | SLH | ZER | DIG | TLD | HAS
        )
    }

    /// Returns whether `byte` may appear unescaped in Sass raw URL contents.
    ///
    /// Example: `/` in `url(//cdn.example/app.css)`.
    fn is_scss_raw_url_byte(byte: u8) -> bool {
        matches!(byte, b'!' | b'#' | b'%' | b'&' | b'*'..=b'~') || !byte.is_ascii()
    }

    /// Returns true when `position + offset` can continue an interpolated URL head.
    ///
    /// Examples: `foo#{$name}` or `#{$name}`.
    fn is_at_scss_identifier_fragment_at(&self, offset: usize) -> bool {
        self.cursor.is_at_scss_interpolation_at(offset) || self.cursor.is_ident_start_at(offset)
    }

    /// Returns true when the current byte can continue an interpolated URL head.
    ///
    /// Examples: `foo#{$name}` or `#{$name}`.
    fn is_at_scss_identifier_fragment(&self) -> bool {
        self.is_at_scss_identifier_fragment_at(0)
    }

    /// Returns true when `-` belongs to the interpolated URL head.
    ///
    /// Example: `url(foo-#{$name}(bar))`.
    fn is_at_scss_identifier_hyphen(&self) -> bool {
        self.cursor.current_byte() == Some(b'-') && self.is_at_scss_identifier_fragment_at(1)
    }
}
