//! An extremely fast, lookup table based, СSS lexer which yields SyntaxKind tokens used by the rome-css parser.
#[rustfmt::skip]
mod tests;

use crate::CssParserOptions;
use biome_css_syntax::{CssSyntaxKind, CssSyntaxKind::*, T, TextLen, TextSize};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_unicode_table::{
    Dispatch::{self, *},
    is_css_non_ascii, lookup_byte,
};
use std::char::REPLACEMENT_CHARACTER;

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
    /// Greedily consume tokens in the URL function until encountering ")"
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
}

/// An extremely fast, lookup table based, lossless CSS lexer
#[derive(Debug)]
pub(crate) struct CssLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source.
    ///
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: CssSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,

    options: CssParserOptions,
}

impl<'src> Lexer<'src> for CssLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;

    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = CssSyntaxKind;
    type LexContext = CssLexContext;
    type ReLexContext = CssReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn position(&self) -> usize {
        self.position
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
                CssLexContext::Selector => self.consume_selector_token(current),
                CssLexContext::PseudoNthSelector => self.consume_pseudo_nth_selector_token(current),
                CssLexContext::UrlRawValue => self.consume_url_raw_value_token(current),
                CssLexContext::Color => self.consume_color_token(current),
                CssLexContext::UnicodeRange => self.consume_unicode_range_token(current),
            },
            None => EOF,
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
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
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.after_newline = after_line_break;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> CssLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            after_newline: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            diagnostics: vec![],
            options: CssParserOptions::default(),
        }
    }

    pub(crate) fn with_options(self, options: CssParserOptions) -> Self {
        Self { options, ..self }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn consume_byte(&mut self, tok: CssSyntaxKind) -> CssSyntaxKind {
        self.advance(1);
        tok
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn current_char_unchecked(&self) -> char {
        self.char_unchecked_at(0)
    }

    /// Peek the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn peek_char_unchecked(&self) -> char {
        self.char_unchecked_at(1)
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn char_unchecked_at(&self, offset: usize) -> char {
        // Precautionary measure for making sure the unsafe code below does not read over memory boundary
        debug_assert!(!self.is_eof());
        self.assert_at_char_boundary(offset);

        // Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
        let string = unsafe {
            std::str::from_utf8_unchecked(
                self.source
                    .as_bytes()
                    .get_unchecked((self.position + offset)..),
            )
        };
        if let Some(chr) = string.chars().next() {
            chr
        } else {
            // Safety: we always call this when we are at a valid char, so this branch is completely unreachable
            unsafe {
                core::hint::unreachable_unchecked();
            }
        }
    }

    /// Check if the lexer is at a valid escape. U+005C REVERSE SOLIDUS (\)
    fn is_valid_escape_at(&self, offset: usize) -> bool {
        match self.byte_at(offset) {
            Some(b'\n' | b'\r') | None => false,
            Some(_) => true,
        }
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

            PRD => {
                if self.is_number_start() {
                    self.consume_number(current)
                } else {
                    self.consume_byte(T![.])
                }
            }

            LSS => self.consume_lss(),

            IDT | DOL if self.peek_byte() == Some(b'=') => {
                self.advance(1);
                self.consume_byte(T!["$="])
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
            BTO => self.consume_byte(T!('[')),
            BTC => self.consume_byte(T![']']),
            COM => self.consume_byte(T![,]),
            MOR => self.consume_mor(),
            TLD => self.consume_tilde(),
            PIP => self.consume_pipe(),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            PRC => self.consume_byte(T![%]),
            Dispatch::AMP => self.consume_byte(T![&]),

            UNI => {
                // A BOM can only appear at the start of a file, so if we haven't advanced at all yet,
                // perform the check. At any other position, the BOM is just considered plain whitespace.
                if self.position == 0 {
                    if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                        self.unicode_bom_length = bom_size;
                        return bom;
                    }
                }
                self.consume_unexpected_character()
            }

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
        if let Some(chr) = self.current_byte() {
            let dispatch = lookup_byte(chr);
            return match dispatch {
                // TLD byte covers `url(~package/tilde.css)`;
                // HAS byte covers `url(#IDofSVGpath);`
                IDT | DOL | UNI | PRD | SLH | ZER | DIG | TLD | HAS => self.consume_url_raw_value(),
                _ => self.consume_token(current),
            };
        }
        self.consume_token(current)
    }

    fn consume_url_raw_value(&mut self) -> CssSyntaxKind {
        let start = self.text_position();
        while let Some(chr) = self.current_byte() {
            let dispatch = lookup_byte(chr);
            match dispatch {
                PNC => {
                    return CSS_URL_VALUE_RAW_LITERAL;
                }
                BSL if self.is_valid_escape_at(1) => {
                    // We can escape any character, so we just skip over the escape sequence
                    // Even a closing parenthesis (PNC token):
                    // url(https://example.com/ima\)ge.png);
                    //                            ^^ escaped closing paren
                    self.advance(2)
                }
                _ => self.advance(1),
            }
        }
        let diagnostic = ParseDiagnostic::new("Invalid url raw value", start..self.text_position());
        self.diagnostics.push(diagnostic);
        CSS_URL_VALUE_RAW_LITERAL
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
        self.assert_current_char_boundary();
        let start = self.text_position();

        self.advance(1); // Skip over the quote
        let mut state = LexStringState::InString;

        while let Some(chr) = self.current_byte() {
            let dispatch = lookup_byte(chr);

            match dispatch {
                QOT if quote == chr => {
                    self.advance(1);
                    state = match state {
                        LexStringState::InString => LexStringState::Terminated,
                        state => state,
                    };
                    break;
                }
                // '\t' etc
                BSL => {
                    let escape_start = self.text_position();
                    self.advance(1);

                    match self.current_byte() {
                        Some(b'\n' | b'\r') => self.advance(1),

                        // Handle escaped `'` but only if this is a end quote string.
                        Some(b'\'') if quote == b'\'' => {
                            self.advance(1);
                        }

                        // Handle escaped `'` but only if this is a end quote string.
                        Some(b'"') if quote == b'"' => {
                            self.advance(1);
                        }

                        Some(c) if c.is_ascii_hexdigit() => {
                            let hex = self.consume_escape_sequence(c);

                            if hex == REPLACEMENT_CHARACTER {
                                state = LexStringState::InvalidEscapeSequence;

                                let diagnostic = ParseDiagnostic::new(
                                    "Invalid escape sequence",
                                    escape_start..self.text_position(),
                                );
                                self.diagnostics.push(diagnostic);
                            }
                        }

                        Some(chr) => {
                            self.advance_byte_or_char(chr);
                        }

                        None => {}
                    }
                }
                WHS if matches!(chr, b'\n' | b'\r') => {
                    let unterminated =
                        ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                            .with_hint("The closing quote must be on the same line.");

                    self.diagnostics.push(unterminated);

                    return ERROR_TOKEN;
                }
                // we don't need to handle IDT because it's always len 1.
                UNI => self.advance_char_unchecked(),

                _ => self.advance(1),
            }
        }

        match state {
            LexStringState::Terminated => CSS_STRING_LITERAL,
            LexStringState::InString => {
                let unterminated =
                    ParseDiagnostic::new("Missing closing quote", start..self.text_position())
                        .with_detail(
                            self.source.text_len()..self.source.text_len(),
                            "file ends here",
                        );
                self.diagnostics.push(unterminated);

                ERROR_TOKEN
            }
            LexStringState::InvalidEscapeSequence => ERROR_TOKEN,
        }
    }

    fn consume_escape_sequence(&mut self, current: u8) -> char {
        debug_assert!(current.is_ascii_hexdigit());

        // SAFETY: The current byte is a hex digit.
        let mut hex = (current as char).to_digit(16).unwrap();
        self.advance(1);
        // Consume as many hex digits as possible, but no more than 6.
        // Note that this means 1-6 hex digits have been consumed in total.
        for _ in 0..5 {
            let Some(digit) = self.current_byte().and_then(|c| {
                if c.is_ascii_hexdigit() {
                    (c as char).to_digit(16)
                } else {
                    None
                }
            }) else {
                break;
            };
            self.advance(1);

            hex = hex * 16 + digit;
        }

        // If the next input code point is whitespace, consume it as well.
        if matches!(self.current_byte(), Some(b'\t' | b' ')) {
            self.advance(1);
        }

        // Interpret the hex digits as a hexadecimal number. If this number is zero, or
        // is for a surrogate, or is greater than the maximum allowed code point, return
        // U+FFFD REPLACEMENT CHARACTER (�).
        match hex {
            // If this number is zero
            0 => REPLACEMENT_CHARACTER,
            // or is for a surrogate
            55_296..=57_343 => REPLACEMENT_CHARACTER,
            // or is greater than the maximum allowed code point
            1_114_112.. => REPLACEMENT_CHARACTER,
            _ => char::from_u32(hex).unwrap_or(REPLACEMENT_CHARACTER),
        }
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
            // Consume it.
            self.advance(1);

            // U+002E FULL STOP (.) followed by a digit...
            if self
                .current_byte()
                .is_some_and(|byte| byte.is_ascii_digit())
            {
                // While the next input code point is a digit, consume it.
                self.consume_number_sequence();
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

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 27];
        let (count, only_ascii_used) = self.consume_ident_sequence(&mut buf);

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
            b"var" => VAR_KW,
            b"highlight" => HIGHLIGHT_KW,
            b"part" => PART_KW,
            b"has" => HAS_KW,
            b"dir" => DIR_KW,
            b"global" => GLOBAL_KW,
            b"local" => LOCAL_KW,
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
            b"style" => STYLE_KW,
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
            b"url" => URL_KW,
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
            _ => IDENT,
        }
    }

    /// Consumes a sequence of identifier characters from a byte stream, appending
    /// them to the provided buffer in lowercase ASCII form.
    ///
    /// This function iteratively processes bytes from the stream, which are part
    /// of an identifier, and appends their lowercase ASCII representation to the buffer.
    /// It stops processing either when the buffer is full or when a non-identifier
    /// character is encountered.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to a byte array where the identifier characters
    ///   will be appended. This buffer should be pre-allocated and have enough
    ///   space to hold the expected identifier.
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
    fn consume_ident_sequence(&mut self, buf: &mut [u8]) -> (usize, bool) {
        debug_assert!(self.is_ident_start());

        let mut idx = 0;
        let mut only_ascii_used = true;
        // Repeatedly consume the next input code point from the stream.
        while let Some(current) = self.current_byte() {
            if let Some(part) = self.consume_ident_part(current) {
                if only_ascii_used && !part.is_ascii() {
                    only_ascii_used = false;
                }

                if only_ascii_used {
                    // Ensure that there is space in the buffer.
                    // Since we're only dealing with ASCII, we need at most 1 byte.
                    if let Some(buf) = buf.get_mut(idx..idx + 1) {
                        // Convert the ASCII character to lowercase.
                        buf[0] = part.to_ascii_lowercase() as u8;
                        idx += 1;
                    }
                }
            } else {
                break;
            }
        }

        (idx, only_ascii_used)
    }

    /// Consume a character that forms part of a CSS identifier.
    ///
    /// Before calling this function, you should make sure that there is a valid identifier start
    /// using [Self::is_ident_start].
    ///
    /// Also handles CSS escape sequences in identifiers and attach appropriate diagnostics for invalid cases.
    ///
    /// Returns the consumed character wrapped in `Some` if it is part of an identifier,
    /// and `None` if it is not.
    fn consume_ident_part(&mut self, current: u8) -> Option<char> {
        let chr = match lookup_byte(current) {
            IDT | MIN | DIG | ZER => {
                self.advance(1);
                // SAFETY: We know that the current byte is a hyphen or a number.
                current as char
            }
            // name code point
            UNI => {
                // SAFETY: We know that the current byte is a valid unicode code point
                let chr = self.current_char_unchecked();
                if is_css_non_ascii(chr) {
                    self.advance(chr.len_utf8());
                    chr
                } else {
                    return None;
                }
            }
            // U+005C REVERSE SOLIDUS (\)
            // If the first and second code points are a valid escape, continue consume.
            // Otherwise, break.
            // BSL if self.is_valid_escape_at(1) => '\\',
            BSL if self.is_valid_escape_at(1) => {
                let escape_start = self.text_position();
                self.advance(1);

                match self.current_byte() {
                    // Any valid escape sequence can be used as an identifier,
                    // even if it becomes the REPLACEMENT CHARACTER (like `\0`).
                    // This is important to handle for cases like the "media
                    // min-width hack": http://browserbu.gs/css-hacks/media-min-width-0-backslash-0/.
                    Some(c) if c.is_ascii_hexdigit() => self.consume_escape_sequence(c),

                    Some(_) => {
                        let chr = self.current_char_unchecked();
                        self.advance(chr.len_utf8());
                        chr
                    }

                    None => {
                        let diagnostic = ParseDiagnostic::new(
                            "Invalid escape sequence",
                            escape_start..self.text_position(),
                        );
                        self.diagnostics.push(diagnostic);

                        return None;
                    }
                }
            }
            _ => return None,
        };

        Some(chr)
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

                while let Some(chr) = self.current_byte() {
                    match chr {
                        b'*' if self.peek_byte() == Some(b'/') => {
                            self.advance(2);

                            if has_newline {
                                self.after_newline = true;
                                return MULTILINE_COMMENT;
                            } else {
                                return COMMENT;
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
                            self.position..self.position + 1,
                            "... but the file ends here",
                        );

                self.diagnostics.push(err);

                if has_newline {
                    MULTILINE_COMMENT
                } else {
                    COMMENT
                }
            }
            Some(b'/') if self.options.allow_wrong_line_comments => {
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
        // See https://drafts.csswg.org/css-syntax-3/#typedef-ident-token
        let Some(current) = self.current_byte() else {
            return false;
        };
        // Look at the first code point:
        match lookup_byte(current) {
            // U+002D HYPHEN-MINUS
            MIN => {
                let Some(next) = self.peek_byte() else {
                    return false;
                };
                match lookup_byte(next) {
                    MIN => {
                        let Some(next) = self.byte_at(2) else {
                            return false;
                        };
                        match lookup_byte(next) {
                            IDT | MIN | DIG | ZER => true,
                            // If the third code point is a name-start code point
                            // return true.
                            UNI => is_css_non_ascii(self.char_unchecked_at(2)),
                            // or the third and fourth code points are a valid escape
                            // return true.
                            BSL => self.is_valid_escape_at(3),
                            _ => false,
                        }
                    }
                    IDT => true,
                    // If the second code point is a name-start code point
                    // return true.
                    UNI => is_css_non_ascii(self.peek_char_unchecked()),
                    // or the second and third code points are a valid escape
                    // return true.
                    BSL => self.is_valid_escape_at(2),
                    _ => false,
                }
            }
            IDT => true,
            UNI => is_css_non_ascii(self.current_char_unchecked()),
            // U+005C REVERSE SOLIDUS (\)
            // If the first and second code points are a valid escape, return true. Otherwise,
            // return false.
            BSL => self.is_valid_escape_at(1),
            _ => false,
        }
    }
}

impl<'src> ReLexer<'src> for CssLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match self.current_byte() {
            Some(current) => match context {
                CssReLexContext::Regular => self.consume_token(current),
                CssReLexContext::UnicodeRange => self.consume_unicode_range_token(current),
            },
            None => EOF,
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for CssLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum LexStringState {
    /// String that contains an invalid escape sequence
    InvalidEscapeSequence,

    /// Between the opening `"` and closing `"` quotes.
    InString,

    /// Properly terminated string
    Terminated,
}
