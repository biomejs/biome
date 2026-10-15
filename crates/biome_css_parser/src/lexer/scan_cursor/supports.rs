use super::{CssScanCursor, StringBodyScanStop};
use crate::lexer::CssStringQuote;
use biome_css_syntax::{CssNumberScanOptions, is_css_whitespace_byte, scan_css_number};
use smallvec::SmallVec;
use std::collections::HashSet;

/// Stores scan results for one immutable stylesheet across parser rewinds.
///
/// ```css
/// @supports at-rule(@future extra) {}
/// ```
#[derive(Default)]
pub(crate) struct SupportsAtRuleScanCache {
    failed_starts: HashSet<usize>,
    #[cfg(test)]
    raw_url_bytes: usize,
}

impl SupportsAtRuleScanCache {
    pub(crate) fn is_failed(&self, start: usize) -> bool {
        self.failed_starts.contains(&start)
    }

    #[cfg(test)]
    pub(crate) fn failed_body_count(&self) -> usize {
        self.failed_starts.len()
    }

    #[cfg(test)]
    pub(crate) fn raw_url_scan_bytes(&self) -> usize {
        self.raw_url_bytes
    }
}

impl CssScanCursor<'_> {
    /// Returns the absolute position of the closing `)` for an `at-rule()`
    /// body, starting immediately after its opening `(`.
    ///
    /// Unclosed strings or delimiters, mismatched closers, and bad URL tokens
    /// return `None`. Failed scans cache open parenthesis bodies.
    ///
    /// ```css
    /// @supports not at-rule(@future; extra) {}
    /// ```
    pub(crate) fn scan_supports_at_rule_fallback_end(
        self,
        cache: &mut SupportsAtRuleScanCache,
    ) -> Option<usize> {
        let mut scanner = SupportsBodyScanner {
            cursor: self,
            closing: SmallVec::new(),
            #[cfg(test)]
            raw_url_bytes: 0,
        };
        let end = scanner.scan();
        #[cfg(test)]
        {
            cache.raw_url_bytes += scanner.raw_url_bytes;
        }
        if end.is_none() {
            cache
                .failed_starts
                .extend(scanner.closing.iter().filter_map(|frame| match frame {
                    SupportsBodyFrame::Parenthesis { body_start } => Some(*body_start),
                    _ => None,
                }));
        }
        end
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SupportsBodyFrame {
    Parenthesis { body_start: usize },
    Delimiter(u8),
    String(CssStringQuote),
}

struct SupportsBodyScanner<'src> {
    cursor: CssScanCursor<'src>,
    closing: SmallVec<[SupportsBodyFrame; 4]>,
    #[cfg(test)]
    raw_url_bytes: usize,
}

impl SupportsBodyScanner<'_> {
    fn scan(&mut self) -> Option<usize> {
        self.closing.push(SupportsBodyFrame::Parenthesis {
            body_start: self.cursor.position(),
        });
        while let Some(current) = self.cursor.current_byte() {
            if let Some(SupportsBodyFrame::String(quote)) = self.closing.last().copied() {
                self.scan_string(quote)?;
                continue;
            }

            match current {
                b'/' if self.cursor.peek_byte() == Some(b'*') => {
                    self.cursor.consume_block_comment();
                }
                b'/' if self.cursor.is_at_line_comment() => {
                    self.cursor.consume_line_comment();
                }
                b'\'' | b'"' => {
                    let quote = CssStringQuote::try_from(current).ok()?;
                    self.cursor.advance(1);
                    self.closing.push(SupportsBodyFrame::String(quote));
                }
                b'(' => {
                    self.cursor.advance(1);
                    self.closing.push(SupportsBodyFrame::Parenthesis {
                        body_start: self.cursor.position(),
                    });
                }
                b'[' | b'{' => {
                    let closing = match current {
                        b'[' => b']',
                        _ => b'}',
                    };
                    self.cursor.advance(1);
                    self.closing.push(SupportsBodyFrame::Delimiter(closing));
                }
                b')' | b']' | b'}' => {
                    let is_matching_closer = match self.closing.last() {
                        Some(SupportsBodyFrame::Parenthesis { .. }) => current == b')',
                        Some(SupportsBodyFrame::Delimiter(closing)) => current == *closing,
                        _ => false,
                    };
                    if !is_matching_closer {
                        return None;
                    }
                    self.closing.pop();
                    if self.closing.is_empty() {
                        return Some(self.cursor.position());
                    }
                    self.cursor.advance(1);
                }
                b'\\' if !self.cursor.is_valid_escape_at(1) => return None,
                b'<' if !self.cursor.is_scss
                    && self.cursor.peek_byte() == Some(b'!')
                    && self.cursor.byte_at(2) == Some(b'-')
                    && self.cursor.byte_at(3) == Some(b'-') =>
                {
                    // `<!--` ends before the URL token in `<!--url(x])`.
                    self.cursor.advance(4);
                }
                b'#' => {
                    self.cursor.advance(1);
                    self.cursor.advance_ident_sequence();
                }
                b'@' => {
                    self.cursor.advance(1);
                    // The name in `@url(...)` belongs to the at-keyword.
                    if self.cursor.is_ident_start() {
                        self.cursor.advance_ident_sequence();
                    }
                }
                b'u' | b'U' if self.cursor.is_scss && self.cursor.peek_byte() == Some(b'+') => {
                    self.consume_unicode_range();
                }
                _ if self.consume_number() => {}
                _ if self.cursor.is_ident_start() => self.consume_identifier()?,
                _ => self.cursor.advance_byte_or_char(current),
            }
        }

        None
    }

    fn scan_string(&mut self, quote: CssStringQuote) -> Option<()> {
        let scan = if self.cursor.is_scss {
            self.cursor.scan_interpolated_string_body(quote)
        } else {
            self.cursor.scan_plain_string_body(quote)
        };

        match scan.stop {
            StringBodyScanStop::ClosingQuote { position } => {
                self.cursor.cursor.set_position(position + 1);
                self.closing.pop();
            }
            StringBodyScanStop::Interpolation { position } => {
                self.cursor.cursor.set_position(position + 2);
                self.closing.push(SupportsBodyFrame::Delimiter(b'}'));
            }
            StringBodyScanStop::Newline { .. } | StringBodyScanStop::Eof { .. } => return None,
        }

        Some(())
    }

    fn consume_number(&mut self) -> bool {
        let Some(end) = scan_css_number(
            self.cursor.cursor.source(),
            self.cursor.position(),
            CssNumberScanOptions::default().with_scss_ellipsis_boundary(self.cursor.is_scss),
        ) else {
            return false;
        };

        self.cursor.cursor.set_position(end);
        // The unit in `1url(...)` belongs to a dimension, not a URL function.
        if self.cursor.is_ident_start() {
            self.cursor.advance_ident_sequence();
        }
        true
    }

    /// Sass leaves `url(...)` separate after `U+1` in `at-rule(U+1url(x]))`.
    fn consume_unicode_range(&mut self) {
        self.cursor.advance(2);
        self.cursor.skip_scss_expression_trivia();
        let start = self.cursor.position();
        self.consume_unicode_codepoint();
        if self.cursor.position() == start {
            return;
        }
        self.cursor.skip_scss_expression_trivia();
        if self.cursor.current_byte() == Some(b'-') {
            self.cursor.advance(1);
            self.cursor.skip_scss_expression_trivia();
            self.consume_unicode_codepoint();
        }
    }

    fn consume_unicode_codepoint(&mut self) {
        while self
            .cursor
            .current_byte()
            .is_some_and(|byte| byte.is_ascii_hexdigit() || byte == b'?')
        {
            self.cursor.advance(1);
        }
    }

    fn consume_identifier(&mut self) -> Option<()> {
        let mut name = [0; 3];
        let scan = self.cursor.consume_ident_sequence(&mut name);
        let is_url = scan.count == name.len()
            && scan.only_ascii_used
            && scan.last_was_buffered
            && name.eq_ignore_ascii_case(b"url");
        if !is_url || self.cursor.current_byte() != Some(b'(') {
            return Some(());
        }

        self.cursor.advance(1);
        self.closing.push(SupportsBodyFrame::Parenthesis {
            body_start: self.cursor.position(),
        });
        let body = self.cursor;
        #[cfg(test)]
        let start = self.cursor.position();
        self.cursor.skip_whitespace();
        let result = if matches!(self.cursor.current_byte(), Some(b'\'' | b'"')) {
            Some(())
        } else {
            self.consume_url_body()
        };
        #[cfg(test)]
        {
            self.raw_url_bytes += self.cursor.position() - start;
        }
        if result.is_none() && self.cursor.is_scss {
            // Sass also permits ordinary expressions, as in `url(var(--asset))`.
            self.cursor = body;
            Some(())
        } else {
            result
        }
    }

    /// Leaves the cursor at `)` in `url(data:image/svg+xml,a,b)`.
    /// Invalid URL bytes stop the scan before recovery retries later queries.
    fn consume_url_body(&mut self) -> Option<()> {
        while let Some(current) = self.cursor.current_byte() {
            match current {
                b')' => return Some(()),
                b'"' | b'\'' | b'(' | 0x01..=0x08 | 0x0b | 0x0e..=0x1f | 0x7f => {
                    return None;
                }
                b'\\' => {
                    if !self.cursor.is_valid_escape_at(1) {
                        return None;
                    }
                    self.cursor.advance(1);
                    let escaped = self.cursor.current_byte()?;
                    if escaped.is_ascii_hexdigit() {
                        self.cursor.consume_escape_sequence();
                    } else {
                        self.cursor.advance_byte_or_char(escaped);
                    }
                }
                _ if is_css_whitespace_byte(current) => {
                    self.cursor.skip_whitespace();
                    return (self.cursor.current_byte() == Some(b')')).then_some(());
                }
                _ => self.cursor.advance_byte_or_char(current),
            }
        }
        None
    }
}
