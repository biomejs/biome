//! An extremely fast, lookup table based, СSS lexer which yields SyntaxKind tokens used by the rome-css parser.
#[rustfmt::skip]
mod tests;
mod scan_cursor;
mod source_cursor;

use self::scan_cursor::CssScanCursor;
use self::scan_cursor::{
    PendingUrlRawValueScan, StringBodyScan, StringBodyScanStop, UrlBodyStartScan,
};
use self::source_cursor::SourceCursor;
use crate::CssParserOptions;
use biome_css_syntax::{
    CssFileSource, CssSyntaxKind, CssSyntaxKind::*, T, TextLen, TextRange, TextSize,
};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_unicode_table::{
    Dispatch::{self, *},
    lookup_byte,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum CssLexContext {
    /// Default context: no particular rules are applied to the lexer logic.
    #[default]
    Regular,
    /// Applied when lexing CSS selectors.
    /// Doesn't skip whitespace trivia for a combinator.
    Selector,

    /// Applied when lexing CSS pseudo nth selectors.
    /// Distinct '-' from identifiers and '+' from numbers.
    PseudoNthSelector,

    /// Applied when lexing CSS url function.
    /// Chooses whether the first body token should stay on regular tokenization
    /// or become a raw URL literal.
    UrlBody { scss_exclusive_syntax_allowed: bool },
    /// Applied when lexing CSS url function raw bodies after classification.
    /// Greedily consume tokens in the URL function until encountering `)`.
    UrlRawValue,

    /// Applied when lexing CSS color literals.
    /// Starting from #
    /// support #000 #000f #ffffff #ffffffff
    /// https://drafts.csswg.org/css-color/#typedef-hex-color
    Color,

    /// Applied when lexing CSS unicode range.
    /// Starting from U+ or u+
    /// support U+0-9A-F? U+0-9A-F{1,6} U+0-9A-F{1,6}?
    /// https://drafts.csswg.org/css-fonts/#unicode-range-desc
    UnicodeRange,

    /// Applied when lexing an SCSS string that contains interpolation.
    /// Carries the opening quote so lexing can stop on the matching delimiter.
    ScssString(CssStringQuote),
    /// Applied while lexing the regular `#{...}` body that originated from an
    /// outer SCSS string. Behaves like regular lexing, but preserves the outer
    /// quote for malformed recovery.
    ScssStringInterpolation(CssStringQuote),

    /// Applied when lexing Tailwind CSS utility classes.
    /// Currently, only applicable to when we encounter a `@apply` rule.
    TailwindUtility,
    /// Applied when lexing Tailwind CSS utility names in `@utility`.
    TailwindUtilityName,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum CssStringQuote {
    Single,
    Double,
}

impl CssStringQuote {
    #[inline]
    fn as_byte(self) -> u8 {
        match self {
            Self::Single => b'\'',
            Self::Double => b'"',
        }
    }
}

impl TryFrom<u8> for CssStringQuote {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'\'' => Ok(Self::Single),
            b'"' => Ok(Self::Double),
            _ => Err(()),
        }
    }
}

impl LexContext for CssLexContext {
    /// Returns true if this is [CssLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

/// Context in which the [CssLexContext]'s current should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CssReLexContext {
    #[expect(dead_code)]
    Regular,
    /// See [CssLexContext::UnicodeRange]
    UnicodeRange,
    /// Re-lexes `+` and `-` as standalone tokens for SCSS expression parsing.
    ScssExpression,
}

/// An extremely fast, lookup table based, lossless CSS lexer
#[derive(Debug)]
pub(crate) struct CssLexer<'src> {
    /// Source text plus the next-token byte position within it.
    cursor: SourceCursor<'src>,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,
    /// `true` if there has been trivia between the last non-trivia token and the next non-trivia token.
    after_whitespace: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source.
    ///
    /// The range of the current token can be computed by `self.position() - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: CssSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,

    options: CssParserOptions,
    source_type: CssFileSource,
    pending_scss_string_start: Option<PendingScssInterpolatedStringStart>,
    pending_url_raw_value_scan: Option<PendingUrlRawValueScan>,
}

impl<'src> Lexer<'src> for CssLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;

    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = CssSyntaxKind;
    type LexContext = CssLexContext;
    type ReLexContext = CssReLexContext;

    fn source(&self) -> &'src str {
        self.cursor.source()
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();

        let kind = match self.current_byte() {
            Some(current) => match context {
                CssLexContext::Regular => self.consume_token(current),
                CssLexContext::ScssStringInterpolation(quote) => {
                    self.consume_token_in_scss_string_interpolation(current, quote)
                }
                CssLexContext::Selector => self.consume_selector_token(current),
                CssLexContext::PseudoNthSelector => self.consume_pseudo_nth_selector_token(current),
                CssLexContext::UrlBody {
                    scss_exclusive_syntax_allowed,
                } => self.consume_url_body_token(current, scss_exclusive_syntax_allowed),
                CssLexContext::UrlRawValue => self.consume_url_raw_value_token(current),
                CssLexContext::Color => self.consume_color_token(current),
                CssLexContext::UnicodeRange => self.consume_unicode_range_token(current),
                CssLexContext::ScssString(quote) => self.lex_scss_string_chunk_token(quote),
                CssLexContext::TailwindUtility => self.consume_token_tailwind_utility(current),
                CssLexContext::TailwindUtilityName => {
                    self.consume_token_tailwind_utility_name(current)
                }
            },
            None => EOF,
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_flags
            .set(TokenFlags::PRECEDING_WHITESPACE, self.after_whitespace);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
            self.after_whitespace = false;
        } else if kind == Self::WHITESPACE {
            self.after_whitespace = true;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    fn has_preceding_whitespace(&self) -> bool {
        self.current_flags.has_preceding_whitespace()
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
            after_whitespace,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        self.cursor.set_position(usize::from(position));
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.after_newline = after_line_break;
        self.after_whitespace = after_whitespace;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
        self.pending_scss_string_start = None;
        self.pending_url_raw_value_scan = None;
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let Some(current) = self.current_byte() else {
            return;
        };
        self.cursor.advance_byte_or_char(current);
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.cursor.advance(n);
    }

    #[inline]
    fn assert_at_char_boundary(&self, offset: usize) {
        debug_assert!(self.source().is_char_boundary(self.position() + offset));
    }

    #[inline]
    fn assert_current_char_boundary(&self) {
        debug_assert!(self.source().is_char_boundary(self.position()));
    }

    #[inline]
    fn current_byte(&self) -> Option<u8> {
        self.cursor.current_byte()
    }

    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.cursor.peek_byte()
    }

    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.cursor.byte_at(offset)
    }
}

impl<'src> CssLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            cursor: SourceCursor::new(source, 0),
            after_newline: false,
            after_whitespace: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            diagnostics: vec![],
            options: CssParserOptions::default(),
            source_type: CssFileSource::default(),
            pending_scss_string_start: None,
            pending_url_raw_value_scan: None,
        }
    }

    pub(crate) fn with_options(self, options: CssParserOptions) -> Self {
        Self { options, ..self }
    }

    pub(crate) fn with_source_type(self, source_type: CssFileSource) -> Self {
        Self {
            source_type,
            ..self
        }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn consume_byte(&mut self, tok: CssSyntaxKind) -> CssSyntaxKind {
        self.advance(1);
        tok
    }

    /// Returns true when lexing SCSS so SCSS-only tokens (`==`, `!=`, `...`, `//`) are enabled.
    ///
    /// Example:
    /// ```scss
    /// @if $a == $b { @include foo($args...); }
    /// ```
    ///
    /// Docs: https://sass-lang.com/documentation/operators
    #[inline]
    fn is_scss(&self) -> bool {
        self.source_type.is_scss()
    }

    /// Enables `//` line-comment lexing only for SCSS (or permissive mode), since
    /// plain CSS treats `//` as two delimiters, not a comment.
    ///
    /// Example:
    /// ```scss
    /// // overrides
    /// $color: red;
    /// ```
    ///
    /// Docs: https://sass-lang.com/documentation/syntax/comments
    #[inline]
    fn is_line_comment_enabled(&self) -> bool {
        self.options.allow_wrong_line_comments || self.is_scss()
    }

    fn scan_cursor(&self) -> CssScanCursor<'src> {
        self.scan_cursor_at(self.position())
    }

    fn scan_cursor_at(&self, position: usize) -> CssScanCursor<'src> {
        // The lexer owns the stable scan environment. Shared scanners vary
        // only by the byte position they inspect.
        CssScanCursor::new(
            SourceCursor::new(self.source(), position),
            self.is_scss(),
            self.is_line_comment_enabled(),
        )
    }

    fn byte_before(&self, position: usize, offset: usize) -> Option<u8> {
        position
            .checked_sub(offset)
            .and_then(|index| self.source().as_bytes().get(index))
            .copied()
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.cursor.set_position(position);
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn current_char_unchecked(&self) -> char {
        self.char_unchecked_at(0)
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn char_unchecked_at(&self, offset: usize) -> char {
        debug_assert!(!self.is_eof());
        self.assert_at_char_boundary(offset);
        self.cursor.char_at(offset)
    }

    /// Lexes the next token
    ///
    /// Guaranteed to not be at the end of the file
    // A lookup table of `byte -> fn(l: &mut Lexer) -> Token` is exponentially slower than this approach
    fn consume_token(&mut self, current: u8) -> CssSyntaxKind {
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => {
                let kind = self.consume_newline_or_whitespaces();
                if kind == Self::NEWLINE {
                    self.after_newline = true;
                }
                kind
            }
            QOT => self.consume_string_literal(current),
            SLH => self.consume_slash(),

            DIG | ZER => self.consume_number(current),

            MIN => self.consume_min(current),

            PLS => {
                if self.is_number_start() {
                    self.consume_number(current)
                } else {
                    self.consume_byte(T![+])
                }
            }

            PRD => self.consume_prd(current),

            LSS => self.consume_lss(),

            DOL => self.consume_dol(),

            // Check for BOM first at position 0, before checking if UNI is an identifier start.
            // The BOM character (U+FEFF) is in the valid CSS non-ASCII identifier range,
            // so without this check it would be incorrectly consumed as part of an identifier.
            UNI if self.position() == 0 => {
                if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                // Not a BOM, check other UNI cases below
                if self.options.is_metavariable_enabled() && self.is_metavariable_start() {
                    self.consume_metavariable(GRIT_METAVARIABLE)
                } else if self.is_ident_start() {
                    self.consume_identifier()
                } else {
                    self.consume_unexpected_character()
                }
            }
            UNI if self.options.is_metavariable_enabled() && self.is_metavariable_start() => {
                self.consume_metavariable(GRIT_METAVARIABLE)
            }
            IDT | UNI | BSL if self.is_ident_start() => self.consume_identifier(),

            MUL => self.consume_mul(),
            CRT => self.consume_ctr(),
            COL => self.consume_col(),
            AT_ => self.consume_byte(T![@]),
            SEM => self.consume_byte(T![;]),
            HAS => self.consume_byte(T![#]),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            COM => self.consume_byte(T![,]),
            MOR => self.consume_mor(),
            TLD => self.consume_tilde(),
            PIP => self.consume_pipe(),
            EQL => self.consume_eql(),
            EXL => self.consume_exl(),
            PRC => self.consume_byte(T![%]),
            Dispatch::AMP => self.consume_byte(T![&]),

            UNI => self.consume_unexpected_character(),

            _ => self.consume_unexpected_character(),
        }
    }

    fn consume_color_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => self.consume_color(),
            _ => self.consume_token(current),
        }
    }

    fn consume_color(&mut self) -> CssSyntaxKind {
        let start = self.text_position();
        let mut length = 0;
        while matches!(
            self.current_byte(),
            Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
        ) {
            self.advance(1);
            length += 1;
        }
        if !matches!(length, 3 | 4 | 6 | 8) {
            let diagnostic = ParseDiagnostic::new("Invalid color", start..self.text_position());
            self.diagnostics.push(diagnostic);
        }

        CSS_COLOR_LITERAL
    }

    /// Consumes a Unicode range token and returns its corresponding syntax kind.
    fn consume_unicode_range_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b'u' | b'U' if matches!(self.peek_byte(), Some(b'+')) => {
                self.advance(1);
                self.consume_byte(T!["U+"])
            }
            b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'?' => self.consume_unicode_range(),
            b'-' => self.consume_byte(T![-]),
            _ => self.consume_token(current),
        }
    }

    /// Consumes a Unicode range and determines its syntax kind.
    ///
    /// This method reads consecutive bytes representing valid hexadecimal characters ('0'-'9', 'a'-'f',
    /// 'A'-'F') or the wildcard character '?'.
    /// It tracks the length of the range and detects if it contains a wildcard.
    /// If the length is invalid (either zero or greater than six), it generates
    /// a `ParseDiagnostic` indicating an invalid Unicode range.
    fn consume_unicode_range(&mut self) -> CssSyntaxKind {
        let start = self.text_position();
        let mut length = 0;
        let mut is_wildcard = false;

        while matches!(
            self.current_byte(),
            Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'?')
        ) {
            // If the current byte is a wildcard character, set the wildcard flag to true.
            if self.current_byte() == Some(b'?') {
                is_wildcard = true;
            }

            self.advance(1);
            length += 1;
        }

        if length == 0 || length > 6 {
            let diagnostic = ParseDiagnostic::new(
                "Invalid unicode range",
                start..self.text_position(),
            )
            .with_hint(
                "Valid length (minimum 1 or maximum 6 hex digits) in the start of unicode range.",
            );
            self.diagnostics.push(diagnostic);
        }

        if is_wildcard {
            CSS_UNICODE_RANGE_WILDCARD_LITERAL
        } else {
            CSS_UNICODE_CODEPOINT_LITERAL
        }
    }

    fn consume_selector_token(&mut self, current: u8) -> CssSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_byte(CSS_SPACE_LITERAL),
            _ => self.consume_token(current),
        }
    }

    fn consume_url_raw_value_token(&mut self, current: u8) -> CssSyntaxKind {
        self.scan_cursor()
            .scan_url_raw_value()
            .and_then(|scan| self.consume_pending_url_raw_value(scan))
            .unwrap_or_else(|| self.consume_token(current))
    }

    fn consume_url_body_token(
        &mut self,
        current: u8,
        scss_exclusive_syntax_allowed: bool,
    ) -> CssSyntaxKind {
        if let Some(scan) = self.take_pending_url_raw_value_scan_at_current_position() {
            return self
                .consume_pending_url_raw_value(scan)
                .unwrap_or_else(|| self.consume_token(current));
        }

        match self.scan_url_body_start(self.position(), scss_exclusive_syntax_allowed) {
            UrlBodyStartScan::InterpolatedFunction | UrlBodyStartScan::Other => {
                self.consume_token(current)
            }
            UrlBodyStartScan::RawValue(scan) => {
                if scan.start == self.position() {
                    self.consume_pending_url_raw_value(scan)
                        .unwrap_or_else(|| self.consume_token(current))
                } else {
                    // Leading trivia is still emitted as normal trivia tokens.
                    // Cache the raw-value scan so the next non-trivia token can
                    // commit the already-classified URL body in one step.
                    self.pending_url_raw_value_scan = Some(scan);
                    self.consume_token(current)
                }
            }
        }
    }

    fn consume_pending_url_raw_value(
        &mut self,
        scan: PendingUrlRawValueScan,
    ) -> Option<CssSyntaxKind> {
        if scan.start != self.position() {
            return None;
        }

        self.set_position(scan.end);

        if !scan.terminated {
            let diagnostic = ParseDiagnostic::new(
                "Invalid url raw value",
                TextSize::from(scan.start as u32)..self.text_position(),
            );
            self.diagnostics.push(diagnostic);
        }

        Some(CSS_URL_VALUE_RAW_LITERAL)
    }

    fn consume_pseudo_nth_selector_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b'-' => self.consume_byte(T![-]),
            b'+' => self.consume_byte(T![+]),
            b'n' | b'N' => self.consume_byte(T![n]),
            _ => self.consume_token(current),
        }
    }

    fn consume_string_literal(&mut self, quote: u8) -> CssSyntaxKind {
        let Ok(quote) = CssStringQuote::try_from(quote) else {
            debug_assert!(false, "string literal lexing must start on a quote byte");
            return ERROR_TOKEN;
        };

        if self.is_scss() {
            return match self.lex_string_token(StringLexMode::ScssStart { quote }) {
                StringLexResult::Token(kind) => kind,
                StringLexResult::Unterminated => ERROR_TOKEN,
            };
        }

        match self.lex_string_token(StringLexMode::Plain { quote }) {
            StringLexResult::Token(kind) => kind,
            StringLexResult::Unterminated => ERROR_TOKEN,
        }
    }

    fn consume_token_in_scss_string_interpolation(
        &mut self,
        current: u8,
        quote: CssStringQuote,
    ) -> CssSyntaxKind {
        if current == quote.as_byte() {
            return match self.scan_same_quote_in_interpolation(quote) {
                SameQuoteInInterpolation::NestedString(scan) => {
                    match self.finish_scss_string_start(self.position(), quote, scan) {
                        StringLexResult::Token(kind) => kind,
                        StringLexResult::Unterminated => {
                            debug_assert!(
                                false,
                                "same-quote interpolation scans must classify unterminated strings as outer quotes"
                            );
                            self.consume_byte(SCSS_STRING_QUOTE)
                        }
                    }
                }
                SameQuoteInInterpolation::OuterQuote => self.consume_byte(SCSS_STRING_QUOTE),
            };
        }

        self.consume_token(current)
    }

    /// Classifies a same-quote byte while inside string-origin interpolation.
    ///
    /// Inside `#{...}` from an outer interpolated string, the current quote may
    /// start a real nested string or may be the outer string closing early
    /// before `}`. This helper performs a non-mutating scan so the valid nested
    /// string path can reuse the scan result for real token commitment.
    fn scan_same_quote_in_interpolation(&self, quote: CssStringQuote) -> SameQuoteInInterpolation {
        let scan = self
            .scan_cursor_at(self.position() + 1)
            .scan_interpolated_string_body(quote);

        match scan.stop {
            StringBodyScanStop::ClosingQuote { .. } | StringBodyScanStop::Interpolation { .. } => {
                SameQuoteInInterpolation::NestedString(scan)
            }
            StringBodyScanStop::Newline { .. } | StringBodyScanStop::Eof { .. } => {
                SameQuoteInInterpolation::OuterQuote
            }
        }
    }

    fn lex_string_token(&mut self, mode: StringLexMode) -> StringLexResult {
        let start = self.position();

        match mode {
            StringLexMode::Plain { quote } => {
                let scan = self.scan_cursor_at(start + 1).scan_plain_string_body(quote);
                self.commit_plain_string_literal(start, scan)
            }
            StringLexMode::ScssStart { quote } => {
                let scan = self
                    .scan_cursor_at(start + 1)
                    .scan_interpolated_string_body(quote);
                self.finish_scss_string_start(start, quote, scan)
            }
        }
    }

    fn finish_scss_string_start(
        &mut self,
        start: usize,
        quote: CssStringQuote,
        scan: StringBodyScan,
    ) -> StringLexResult {
        if matches!(scan.stop, StringBodyScanStop::Interpolation { .. }) {
            self.pending_scss_string_start = Some(PendingScssInterpolatedStringStart {
                quote,
                content_start: start + 1,
                initial_chunk_scan: scan,
            });

            return StringLexResult::Token(self.consume_byte(SCSS_STRING_QUOTE));
        }

        self.commit_plain_string_literal(start, scan)
    }

    fn lex_scss_string_chunk_token(&mut self, quote: CssStringQuote) -> CssSyntaxKind {
        if self.current_byte() == Some(quote.as_byte()) {
            self.pending_scss_string_start = None;
            return self.consume_byte(SCSS_STRING_QUOTE);
        }

        if self.is_at_scss_interpolation() {
            self.pending_scss_string_start = None;
            return self.consume_byte(T![#]);
        }

        self.commit_scss_string_content(quote)
    }

    fn is_at_scss_interpolation(&self) -> bool {
        self.scan_cursor().is_at_scss_interpolation()
    }

    fn commit_scss_string_content(&mut self, quote: CssStringQuote) -> CssSyntaxKind {
        self.assert_current_char_boundary();
        let start = self.position();

        let scan = self
            .take_pending_scss_string_start_scan(quote)
            .unwrap_or_else(|| self.scan_cursor().scan_interpolated_string_body(quote));

        match scan.stop {
            StringBodyScanStop::Interpolation { position }
            | StringBodyScanStop::ClosingQuote { position } => {
                debug_assert!(
                    position > start,
                    "SCSS string content must never be zero-length"
                );

                self.set_position(position);
                self.push_string_issues(&scan.invalid_escape_ranges);
                // Invalid escapes still belong to this string chunk; keep the
                // semantic token kind and surface the problem via diagnostics.
                SCSS_STRING_CONTENT_LITERAL
            }
            StringBodyScanStop::Newline { position, .. } => {
                self.set_position(position);
                self.push_string_issues(&scan.invalid_escape_ranges);
                let unterminated = ParseDiagnostic::new(
                    "Missing closing quote",
                    TextSize::from(start as u32)..self.text_position(),
                )
                .with_hint("The closing quote must be on the same line.");
                self.diagnostics.push(unterminated);
                ERROR_TOKEN
            }
            StringBodyScanStop::Eof { position } => {
                self.set_position(position);
                self.push_string_issues(&scan.invalid_escape_ranges);
                let unterminated = ParseDiagnostic::new(
                    "Missing closing quote",
                    TextSize::from(start as u32)..self.text_position(),
                )
                .with_detail(
                    self.source().text_len()..self.source().text_len(),
                    "file ends here",
                );
                self.diagnostics.push(unterminated);
                ERROR_TOKEN
            }
        }
    }

    fn commit_plain_string_literal(
        &mut self,
        start: usize,
        scan: StringBodyScan,
    ) -> StringLexResult {
        match scan.stop {
            StringBodyScanStop::ClosingQuote { position } => {
                self.set_position(position + 1);
                self.push_string_issues(&scan.invalid_escape_ranges);
                StringLexResult::Token(if !scan.invalid_escape_ranges.is_empty() {
                    ERROR_TOKEN
                } else {
                    CSS_STRING_LITERAL
                })
            }
            StringBodyScanStop::Newline { position, .. } => {
                self.set_position(position);
                self.push_string_issues(&scan.invalid_escape_ranges);
                let unterminated = ParseDiagnostic::new(
                    "Missing closing quote",
                    TextSize::from(start as u32)..self.text_position(),
                )
                .with_hint("The closing quote must be on the same line.");
                self.diagnostics.push(unterminated);
                StringLexResult::Unterminated
            }
            StringBodyScanStop::Eof { position } => {
                self.set_position(position);
                self.push_string_issues(&scan.invalid_escape_ranges);
                let unterminated = ParseDiagnostic::new(
                    "Missing closing quote",
                    TextSize::from(start as u32)..self.text_position(),
                )
                .with_detail(
                    self.source().text_len()..self.source().text_len(),
                    "file ends here",
                );
                self.diagnostics.push(unterminated);
                StringLexResult::Unterminated
            }
            StringBodyScanStop::Interpolation { .. } => {
                debug_assert!(
                    false,
                    "plain string literal finishing must not see an interpolation stop"
                );
                StringLexResult::Token(CSS_STRING_LITERAL)
            }
        }
    }

    fn push_string_issues(&mut self, invalid_escape_ranges: &[TextRange]) {
        for range in invalid_escape_ranges {
            self.diagnostics
                .push(ParseDiagnostic::new("Invalid escape sequence", *range));
        }
    }

    fn take_pending_scss_string_start_scan(
        &mut self,
        quote: CssStringQuote,
    ) -> Option<StringBodyScan> {
        self.pending_scss_string_start
            .take()
            .filter(|pending| pending.quote == quote && pending.content_start == self.position())
            .map(|pending| pending.initial_chunk_scan)
    }

    /// Lexes a CSS number literal
    fn consume_number(&mut self, current: u8) -> CssSyntaxKind {
        debug_assert!(self.is_number_start());

        if matches!(current, b'+' | b'-') {
            self.advance(1);
        }

        // While the next input code point is a digit, consume it.
        self.consume_number_sequence();

        // According to the spec if the next 2 input code points are U+002E FULL STOP (.) followed by a digit we need to consume them.
        // However we want to parse numbers like `1.` and `1.e10` where we don't have a number after (.)
        // If the next input code points are U+002E FULL STOP (.)...
        if matches!(self.current_byte(), Some(b'.')) {
            // In SCSS, leave the dot for the ellipsis token (e.g., `10...` or `$args...`).
            let is_scss_ellipsis =
                self.is_scss() && self.peek_byte() == Some(b'.') && self.byte_at(2) == Some(b'.');

            if !is_scss_ellipsis {
                // Consume the dot.
                self.advance(1);

                // If U+002E FULL STOP (.) is followed by a digit, consume the number sequence.
                if self
                    .current_byte()
                    .is_some_and(|byte| byte.is_ascii_digit())
                {
                    self.consume_number_sequence();
                }
            }
        }

        // If the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E) or
        // U+0065 LATIN SMALL LETTER E (e), optionally followed by U+002D HYPHEN-MINUS
        // (-) or U+002B PLUS SIGN (+), followed by a digit, then:
        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            match (self.peek_byte(), self.byte_at(2)) {
                (Some(b'-' | b'+'), Some(byte)) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(3);

                    // While the next input code point is a digit, consume it.
                    self.consume_number_sequence()
                }
                (Some(byte), _) if byte.is_ascii_digit() => {
                    // Consume them.
                    self.advance(2);

                    // While the next input code point is a digit, consume it.
                    self.consume_number_sequence()
                }
                _ => {}
            }
        }

        // A Number immediately followed by an identifier is considered a single
        // <dimension> token according to the spec: https://www.w3.org/TR/css-values-4/#dimensions.
        // Spaces are not allowed, but by default this lexer will skip over
        // whitespace tokens, making it difficult to determine on the next token
        // if it is actually a dimension unit or a separated identifier. So, if
        // the next characters constitute an identifier after parsing a number,
        // this special CSS_DIMENSION_VALUE token will indicate to the parser
        // that it is part of a dimension and the next token can safely be
        // consumed as the unit.
        //
        // The parser will re-cast these tokens as CSS_NUMBER_LITERALs when
        // creating the Dimension node to hide this internal detail.
        if matches!(self.current_byte(), Some(b'%')) {
            CSS_PERCENTAGE_VALUE
        } else if self.is_ident_start() {
            CSS_DIMENSION_VALUE
        } else {
            CSS_NUMBER_LITERAL
        }
    }

    fn consume_number_sequence(&mut self) {
        // While the next input code point is a digit, consume it.
        while let Some(b'0'..=b'9') = self.current_byte() {
            self.advance(1);
        }
    }

    fn consume_identifier(&mut self) -> CssSyntaxKind {
        debug_assert!(self.is_ident_start());

        self.consume_identifier_with_slash(false)
    }

    fn consume_identifier_with_slash(&mut self, allow_slash: bool) -> CssSyntaxKind {
        debug_assert!(self.is_ident_start());

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 27];
        let (count, only_ascii_used) = self.consume_ident_sequence(&mut buf, allow_slash);

        if !only_ascii_used {
            return IDENT;
        }

        match &buf[..count] {
            b"media" => MEDIA_KW,
            b"keyframes" => KEYFRAMES_KW,
            b"-webkit-keyframes" => KEYFRAMES_KW,
            b"-moz-keyframes" => KEYFRAMES_KW,
            b"-o-keyframes" => KEYFRAMES_KW,
            b"-ms-keyframes" => KEYFRAMES_KW,
            b"and" => AND_KW,
            b"only" => ONLY_KW,
            b"or" => OR_KW,
            b"i" => I_KW,
            b"important" => IMPORTANT_KW,
            b"from" => FROM_KW,
            b"to" => TO_KW,
            b"cover" => COVER_KW,
            b"contain" => CONTAIN_KW,
            b"entry" => ENTRY_KW,
            b"exit" => EXIT_KW,
            b"entry-crossing" => ENTRY_CROSSING_KW,
            b"exit-crossing" => EXIT_CROSSING_KW,
            b"through" => THROUGH_KW,
            b"var" => VAR_KW,
            b"highlight" => HIGHLIGHT_KW,
            b"part" => PART_KW,
            b"has" => HAS_KW,
            b"dir" => DIR_KW,
            b"global" => GLOBAL_KW,
            b"local" => LOCAL_KW,
            b"slotted" => SLOTTED_KW,
            b"deep" => DEEP_KW,
            b"-moz-any" => ANY_KW,
            b"-webkit-any" => ANY_KW,
            b"past" => PAST_KW,
            b"current" => CURRENT_KW,
            b"future" => FUTURE_KW,
            b"host" => HOST_KW,
            b"host-context" => HOST_CONTEXT_KW,
            b"not" => NOT_KW,
            b"matches" => MATCHES_KW,
            b"is" => IS_KW,
            b"where" => WHERE_KW,
            b"lang" => LANG_KW,
            b"of" => OF_KW,
            b"n" => N_KW,
            b"even" => EVEN_KW,
            b"odd" => ODD_KW,
            b"nth-child" => NTH_CHILD_KW,
            b"nth-last-child" => NTH_LAST_CHILD_KW,
            b"nth-of-type" => NTH_OF_TYPE_KW,
            b"nth-last-of-type" => NTH_LAST_OF_TYPE_KW,
            b"nth-col" => NTH_COL_KW,
            b"nth-last-col" => NTH_LAST_COL_KW,
            b"active-view-transition-type" => ACTIVE_VIEW_TRANSITION_TYPE_KW,
            b"ltr" => LTR_KW,
            b"rtl" => RTL_KW,
            b"charset" => CHARSET_KW,
            b"color-profile" => COLOR_PROFILE_KW,
            b"counter-style" => COUNTER_STYLE_KW,
            b"property" => PROPERTY_KW,
            b"container" => CONTAINER_KW,
            b"each" => EACH_KW,
            b"debug" => DEBUG_KW,
            b"warn" => WARN_KW,
            b"error" => ERROR_KW,
            b"content" => CONTENT_KW,
            b"at-root" => AT_ROOT_KW,
            b"extend" => EXTEND_KW,
            b"for" => FOR_KW,
            b"forward" => FORWARD_KW,
            b"hide" => HIDE_KW,
            b"include" => INCLUDE_KW,
            b"mixin" => MIXIN_KW,
            b"optional" => OPTIONAL_KW,
            b"while" => WHILE_KW,
            b"without" => WITHOUT_KW,
            b"show" => SHOW_KW,
            b"sass" => SASS_KW,
            b"style" => STYLE_KW,
            b"state" => STATE_KW,
            b"font-face" => FONT_FACE_KW,
            b"font-feature-values" => FONT_FEATURE_VALUES_KW,
            // font-feature-values items
            b"stylistic" => STYLISTIC_KW,
            b"historical-forms" => HISTORICAL_FORMS_KW,
            b"styleset" => STYLESET_KW,
            b"character-variant" => CHARACTER_VARIANT_KW,
            b"swash" => SWASH_KW,
            b"ornaments" => ORNAMENTS_KW,
            b"annotation" => ANNOTATION_KW,
            b"font-palette-values" => FONT_PALETTE_VALUES_KW,
            b"auto" => AUTO_KW,
            b"thin" => THIN_KW,
            b"medium" => MEDIUM_KW,
            b"thick" => THICK_KW,
            b"none" => NONE_KW,
            b"hidden" => HIDDEN_KW,
            b"dotted" => DOTTED_KW,
            b"dashed" => DASHED_KW,
            b"solid" => SOLID_KW,
            b"double" => DOUBLE_KW,
            b"groove" => GROOVE_KW,
            b"ridge" => RIDGE_KW,
            b"inset" => INSET_KW,
            b"outset" => OUTSET_KW,
            // CSS-Wide keywords
            b"initial" => INITIAL_KW,
            b"inherit" => INHERIT_KW,
            b"unset" => UNSET_KW,
            b"revert" => REVERT_KW,
            b"revert-layer" => REVERT_LAYER_KW,
            b"default" => DEFAULT_KW,
            // length units
            b"em" => EM_KW,
            b"rem" => REM_KW,
            b"ex" => EX_KW,
            b"rex" => REX_KW,
            b"cap" => CAP_KW,
            b"rcap" => RCAP_KW,
            b"ch" => CH_KW,
            b"rch" => RCH_KW,
            b"ic" => IC_KW,
            b"ric" => RIC_KW,
            b"lh" => LH_KW,
            b"rlh" => RLH_KW,
            // Viewport-percentage Lengths
            b"vw" => VW_KW,
            b"svw" => SVW_KW,
            b"lvw" => LVW_KW,
            b"dvw" => DVW_KW,
            b"vh" => VH_KW,
            b"svh" => SVW_KW,
            b"lvh" => LVH_KW,
            b"dvh" => DVH_KW,
            b"vi" => VI_KW,
            b"svi" => SVI_KW,
            b"lvi" => LVI_KW,
            b"dvi" => DVI_KW,
            b"vb" => VB_KW,
            b"svb" => SVB_KW,
            b"lvb" => LVB_KW,
            b"dvb" => DVB_KW,
            b"vmin" => VMIN_KW,
            b"svmin" => SVMIN_KW,
            b"lvmin" => LVMIN_KW,
            b"dvmin" => DVMIN_KW,
            b"vmax" => VMAX_KW,
            b"svmax" => SVMAX_KW,
            b"lvmax" => LVMAX_KW,
            b"dvmax" => DVMAX_KW,
            // Absolute lengths
            b"cm" => CM_KW,
            b"mm" => MM_KW,
            b"q" => Q_KW,
            b"in" => IN_KW,
            b"pc" => PC_KW,
            b"pt" => PT_KW,
            b"px" => PX_KW,
            b"mozmm" => MOZMM_KW,
            // mini app
            b"rpx" => RPX_KW,
            // container lengths
            b"cqw" => CQW_KW,
            b"cqh" => CQH_KW,
            b"cqi" => CQI_KW,
            b"cqb" => CQB_KW,
            b"cqmin" => CQMIN_KW,
            b"cqmax" => CQMAX_KW,
            // angle units
            b"deg" => DEG_KW,
            b"grad" => GRAD_KW,
            b"rad" => RAD_KW,
            b"turn" => TURN_KW,
            // time units
            b"s" => S_KW,
            b"ms" => MS_KW,
            // frequency units
            b"hz" => HZ_KW,
            b"khz" => KHZ_KW,
            // resolution units
            b"dpi" => DPI_KW,
            b"dpcm" => DPCM_KW,
            b"dppx" => DPPX_KW,
            b"x" => X_KW,
            // flex units
            b"fr" => FR_KW,
            // page at rule
            b"left" => LEFT_KW,
            b"right" => RIGHT_KW,
            b"first" => FIRST_KW,
            b"blank" => BLANK_KW,
            b"page" => PAGE_KW,
            b"top-left-corner" => TOP_LEFT_CORNER_KW,
            b"top-left" => TOP_LEFT_KW,
            b"top-center" => TOP_CENTER_KW,
            b"top-right" => TOP_RIGHT_KW,
            b"top-right-corner" => TOP_RIGHT_CORNER_KW,
            b"bottom-left-corner" => BOTTOM_LEFT_CORNER_KW,
            b"bottom-left" => BOTTOM_LEFT_KW,
            b"bottom-center" => BOTTOM_CENTER_KW,
            b"bottom-right" => BOTTOM_RIGHT_KW,
            b"bottom-right-corner" => BOTTOM_RIGHT_CORNER_KW,
            b"left-top" => LEFT_TOP_KW,
            b"left-middle" => LEFT_MIDDLE_KW,
            b"left-bottom" => LEFT_BOTTOM_KW,
            b"right-top" => RIGHT_TOP_KW,
            b"right-middle" => RIGHT_MIDDLE_KW,
            b"right-bottom" => RIGHT_BOTTOM_KW,
            b"layer" => LAYER_KW,
            b"supports" => SUPPORTS_KW,
            b"selector" => SELECTOR_KW,
            b"if" => IF_KW,
            b"else" => ELSE_KW,
            b"url" => URL_KW,
            b"attr" => ATTR_KW,
            b"type" => TYPE_KW,
            b"raw-string" => RAW_STRING_KW,
            b"number" => NUMBER_KW,
            b"src" => SRC_KW,
            b"scope" => SCOPE_KW,
            b"import" => IMPORT_KW,
            b"namespace" => NAMESPACE_KW,
            b"starting-style" => STARTING_STYLE_KW,
            b"document" => DOCUMENT_KW,
            b"-moz-document" => DOCUMENT_KW,
            b"url-prefix" => URL_PREFIX_KW,
            b"domain" => DOMAIN_KW,
            b"media-document" => MEDIA_DOCUMENT_KW,
            b"regexp" => REGEXP_KW,
            b"value" => VALUE_KW,
            b"as" => AS_KW,
            b"composes" => COMPOSES_KW,
            b"position-try" => POSITION_TRY_KW,
            b"view-transition" => VIEW_TRANSITION_KW,
            b"function" => FUNCTION_KW,
            b"return" => RETURN_KW,
            b"returns" => RETURNS_KW,
            b"use" => USE_KW,
            b"with" => WITH_KW,
            // Tailwind CSS 4.0 keywords
            b"theme" => THEME_KW,
            b"utility" => UTILITY_KW,
            b"variant" => VARIANT_KW,
            b"custom-variant" => CUSTOM_VARIANT_KW,
            b"apply" => APPLY_KW,
            b"source" => SOURCE_KW,
            b"reference" => REFERENCE_KW,
            b"config" => CONFIG_KW,
            b"plugin" => PLUGIN_KW,
            b"slot" => SLOT_KW,
            b"inline" => INLINE_KW,
            _ => IDENT,
        }
    }

    /// Consumes one identifier token via a single shared scan cursor and
    /// returns the keyword-buffer bookkeeping needed by the lexer.
    ///
    /// # Arguments
    ///
    /// * `buf` - Lowercased ASCII keyword buffer filled by the shared scanner
    ///   while the identifier remains ASCII-only.
    /// * `allow_slash` - Whether `/` should be accepted as an identifier
    ///   fragment for Tailwind utility lexing.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    ///
    /// * The number of bytes appended to the buffer (`usize`).
    /// * A boolean indicating whether only ASCII characters were used (`true` if so).
    ///
    /// # Panics
    ///
    /// This function will panic if the first character to be consumed is not a valid
    /// start of an identifier, as determined by `self.is_ident_start()`.
    fn consume_ident_sequence(&mut self, buf: &mut [u8], allow_slash: bool) -> (usize, bool) {
        debug_assert!(self.is_ident_start());

        let mut scan_cursor = self.scan_cursor();
        let scan = if allow_slash {
            scan_cursor.consume_ident_sequence_with_slash(buf)
        } else {
            scan_cursor.consume_ident_sequence(buf)
        };
        let mut position = scan.position;
        let mut count = scan.count;
        if self.options.is_tailwind_directives_enabled() && scan.stop_byte == Some(b'*') {
            // Tailwind keeps `--*` as a unit, but stops before the trailing
            // `-` in patterns like `--color-*`. Check the raw source bytes at
            // the scan boundary so escaped hyphens do not trigger this fixup.
            let prev_is_hyphen = self.byte_before(scan.position, 1) == Some(b'-');
            let prev_prev_is_hyphen = self.byte_before(scan.position, 2) == Some(b'-');

            if prev_is_hyphen && !prev_prev_is_hyphen {
                position = position.saturating_sub(1);

                // Only trim the keyword-buffer count if that trailing `-`
                // actually fit into the ASCII buffer.
                if scan.only_ascii_used && scan.last_was_buffered {
                    count = count.saturating_sub(1);
                }
            }
        }

        self.set_position(position);
        (count, scan.only_ascii_used)
    }

    /// Lexes a comment.
    fn consume_slash(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'/');

        match self.peek_byte() {
            Some(b'*') => {
                let start = self.text_position();

                // eat `/*`
                self.advance(2);

                let mut has_newline = false;
                let is_scss = self.is_scss();
                let mut depth = 1u32;

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'/' if is_scss && self.peek_byte() == Some(b'*') => {
                            self.advance(2);
                            depth = depth.saturating_add(1);
                        }
                        b'*' if self.peek_byte() == Some(b'/') => {
                            self.advance(2);

                            if is_scss {
                                depth = depth.saturating_sub(1);
                            }

                            if !is_scss || depth == 0 {
                                if has_newline {
                                    self.after_newline = true;
                                    return MULTILINE_COMMENT;
                                } else {
                                    return COMMENT;
                                }
                            }
                        }
                        b'\n' | b'\r' => {
                            has_newline = true;
                            self.advance(1)
                        }
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                let err =
                    ParseDiagnostic::new("Unterminated block comment", start..self.text_position())
                        .with_detail(
                            self.position()..self.position() + 1,
                            "... but the file ends here",
                        );

                self.diagnostics.push(err);

                if has_newline {
                    MULTILINE_COMMENT
                } else {
                    COMMENT
                }
            }
            Some(b'/') if self.is_line_comment_enabled() => {
                self.advance(2);

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'\n' | b'\r' => return COMMENT,
                        chr => self.advance_byte_or_char(chr),
                    }
                }

                COMMENT
            }
            _ => self.consume_byte(T![/]),
        }
    }

    #[inline]
    fn consume_eql(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'=');

        if self.is_scss() && self.peek_byte() == Some(b'=') {
            self.advance(1);
            self.consume_byte(T![==])
        } else {
            self.consume_byte(T![=])
        }
    }

    #[inline]
    fn consume_exl(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'!');

        if self.is_scss() && self.peek_byte() == Some(b'=') {
            self.advance(1);
            self.consume_byte(T![!=])
        } else {
            self.consume_byte(T![!])
        }
    }

    #[inline]
    fn consume_pipe(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'|');

        match self.next_byte() {
            Some(b'|') => self.consume_byte(T![||]),
            Some(b'=') => self.consume_byte(T![|=]),
            _ => T![|],
        }
    }

    #[inline]
    fn consume_mor(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'>');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![>=]),
            _ => T![>],
        }
    }

    #[inline]
    fn consume_tilde(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'~');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![~=]),
            _ => T![~],
        }
    }

    #[inline]
    fn consume_col(&mut self) -> CssSyntaxKind {
        self.assert_byte(b':');

        match self.next_byte() {
            Some(b':') => self.consume_byte(T![::]),
            _ => T![:],
        }
    }

    #[inline]
    fn consume_mul(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'*');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![*=]),
            _ => T![*],
        }
    }

    #[inline]
    fn consume_ctr(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'^');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![^=]),
            _ => T![^],
        }
    }

    #[inline]
    fn consume_prd(&mut self, current: u8) -> CssSyntaxKind {
        self.assert_byte(b'.');

        if self.is_scss() && self.peek_byte() == Some(b'.') && self.byte_at(2) == Some(b'.') {
            self.advance(2);
            self.consume_byte(T![...])
        } else if self.is_number_start() {
            self.consume_number(current)
        } else {
            self.consume_byte(T![.])
        }
    }

    #[inline]
    fn consume_dol(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'$');

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T!["$="]),
            _ => T![$],
        }
    }

    #[inline]
    fn consume_lss(&mut self) -> CssSyntaxKind {
        self.assert_byte(b'<');

        // If the next 3 input code points are U+0021 EXCLAMATION MARK U+002D
        // HYPHEN-MINUS U+002D HYPHEN-MINUS (!--), consume them and return a CDO.
        if self.peek_byte() == Some(b'!')
            && self.byte_at(2) == Some(b'-')
            && self.byte_at(3) == Some(b'-')
        {
            self.advance(4);
            return CDO;
        }

        match self.next_byte() {
            Some(b'=') => self.consume_byte(T![<=]),
            _ => T![<],
        }
    }

    #[inline]
    fn consume_min(&mut self, current: u8) -> CssSyntaxKind {
        self.assert_byte(b'-');

        if self.is_number_start() {
            return self.consume_number(current);
        }

        // GREATER-THAN SIGN (->), consume them and return a CDC.
        if self.peek_byte() == Some(b'-') {
            if self.byte_at(2) == Some(b'>') {
                self.advance(3);
                return CDC;
            }

            // --dashed-identifier
            if self.is_ident_start() {
                return self.consume_identifier();
            }
        }

        // -identifier
        if self.is_ident_start() {
            return self.consume_identifier();
        }

        self.consume_byte(T![-])
    }

    #[inline]
    fn consume_unexpected_character(&mut self) -> CssSyntaxKind {
        self.assert_current_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Check if the lexer starts a number.
    fn is_number_start(&self) -> bool {
        match self.current_byte() {
            Some(b'+' | b'-') => match self.peek_byte() {
                // If the second code point is a digit, return true.
                Some(byte) if byte.is_ascii_digit() => true,
                // Otherwise, if the second code point is a U+002E FULL STOP (.) and the
                // third code point is a digit, return true.
                Some(b'.') if self.byte_at(2).is_some_and(|byte| byte.is_ascii_digit()) => true,
                _ => false,
            },
            Some(b'.') => match self.peek_byte() {
                // If the second code point is a digit, return true.
                Some(byte) if byte.is_ascii_digit() => true,
                _ => false,
            },
            Some(byte) => byte.is_ascii_digit(),
            _ => false,
        }
    }

    /// Check if the lexer starts an identifier.
    fn is_ident_start(&self) -> bool {
        // ASCII identifier bytes are already proven by dispatch, so avoid
        // building a shared scan cursor for that trivial hot path.
        matches!(self.current_byte().map(lookup_byte), Some(IDT))
            || self.scan_cursor().is_ident_start()
    }

    fn consume_token_tailwind_utility(&mut self, current: u8) -> CssSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => {
                let kind = self.consume_newline_or_whitespaces();
                if kind == Self::NEWLINE {
                    self.after_newline = true;
                }
                kind
            }
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            SEM => self.consume_byte(T![;]),
            _ => self.consume_tailwind_utility(),
        }
    }

    fn consume_token_tailwind_utility_name(&mut self, current: u8) -> CssSyntaxKind {
        if self.is_ident_start() {
            return self.consume_identifier_with_slash(true);
        }

        self.consume_token(current)
    }

    /// Consume a single tailwind utility as a css identifier.
    ///
    /// This is intentionally very loose, and pretty much considers anything that isn't whitespace or semicolon.
    fn consume_tailwind_utility(&mut self) -> CssSyntaxKind {
        while let Some(current) = self.current_byte() {
            let dispatched = lookup_byte(current);
            match dispatched {
                WHS | SEM | BEC => break,
                _ => {}
            }
            self.advance(1);
        }

        T![ident]
    }
}

impl<'src> ReLexer<'src> for CssLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position();
        self.set_position(usize::from(self.current_start));

        let re_lexed_kind = match self.current_byte() {
            Some(current) => match context {
                CssReLexContext::Regular => self.consume_token(current),
                CssReLexContext::UnicodeRange => self.consume_unicode_range_token(current),
                CssReLexContext::ScssExpression => self.consume_scss_expression_token(current),
            },
            None => EOF,
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.set_position(old_position);
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

impl<'src> CssLexer<'src> {
    pub(crate) fn has_pending_scss_string_start(&self) -> bool {
        self.pending_scss_string_start.is_some()
    }

    /// Returns true when `start` begins an interpolation-containing identifier
    /// immediately followed by `(`, without consuming lexer state.
    pub(crate) fn is_at_scss_interpolated_function(&self, start: usize) -> bool {
        self.scan_cursor_at(start)
            .is_at_scss_interpolated_function()
    }

    fn take_pending_url_raw_value_scan_at_current_position(
        &mut self,
    ) -> Option<PendingUrlRawValueScan> {
        let scan = self.pending_url_raw_value_scan?;

        if scan.start == self.position() {
            self.pending_url_raw_value_scan = None;
            Some(scan)
        } else {
            if self.position() > scan.start {
                // A fallback tokenization path already moved past the cached
                // raw-literal start, so the speculative range is no longer
                // aligned with the real lexer position.
                self.pending_url_raw_value_scan = None;
            }

            None
        }
    }

    fn consume_scss_expression_token(&mut self, current: u8) -> CssSyntaxKind {
        match current {
            b'+' => self.consume_byte(T![+]),
            b'-' => self.consume_byte(T![-]),
            _ => self.consume_token(current),
        }
    }

    fn scan_url_body_start(
        &self,
        start: usize,
        scss_exclusive_syntax_allowed: bool,
    ) -> UrlBodyStartScan {
        self.scan_cursor_at(start)
            .scan_url_body_start(scss_exclusive_syntax_allowed)
    }
}

impl<'src> LexerWithCheckpoint<'src> for CssLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.cursor.position() as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            after_whitespace: self.after_whitespace,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

/// Cached result of the initial SCSS interpolated-string scan.
///
/// Regular lexing first sees the opening quote. If the scanner finds an
/// unescaped `#{` before the matching closing quote, the lexer emits only the
/// opening `SCSS_STRING_QUOTE` token and stores this cache for the next
/// `CssLexContext::ScssString(...)` token fetch.
///
/// That next fetch reuses the cached scan result for the first string chunk
/// instead of rescanning from the opening quote or emitting a coarse
/// `CSS_STRING_LITERAL` that later has to be re-lexed.
#[derive(Debug, Clone, Eq, PartialEq)]
struct PendingScssInterpolatedStringStart {
    /// Delimiter of the outer interpolated string.
    quote: CssStringQuote,
    /// Absolute byte position immediately after the opening quote, where the
    /// first string-content token starts if the chunk is non-empty.
    content_start: usize,
    /// Cached scan result for the bytes between `content_start` and the first
    /// interpolation or closing-quote boundary.
    initial_chunk_scan: StringBodyScan,
}

/// Semantic mode used by the string lexer driver.
///
/// This is the public-facing split inside the lexer between:
/// - plain quoted strings
/// - SCSS interpolated-string start detection
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StringLexMode {
    /// Lex a plain quoted string that cannot start SCSS segmentation.
    Plain { quote: CssStringQuote },
    /// Lex the opening quote of an SCSS string, starting segmentation if an
    /// interpolation boundary appears before the closing quote.
    ScssStart { quote: CssStringQuote },
}

/// Result of lexing one token in a normal string lexer mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum StringLexResult {
    /// The lexer committed a concrete token.
    Token(CssSyntaxKind),
    /// The string terminated at newline or EOF before a normal closing token
    /// could be produced.
    Unterminated,
}

/// Classification of a same-quote byte while lexing string-origin
/// interpolation.
#[derive(Debug, Clone, Eq, PartialEq)]
enum SameQuoteInInterpolation {
    /// The quote starts a valid nested string and carries the reused scan
    /// result for committing that token.
    NestedString(StringBodyScan),
    /// The quote should be treated as the outer string closing early.
    OuterQuote,
}
