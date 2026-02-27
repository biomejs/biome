//! An extremely fast, lookup table based, Markdown lexer which yields SyntaxKind tokens used by the biome-markdown parser.

#[rustfmt::skip]
mod tests;

use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::{SyntaxKind, TextSize};
use biome_unicode_table::Dispatch::{self, AMP, *};
use biome_unicode_table::lookup_byte;

use crate::syntax::{MAX_BLOCK_PREFIX_INDENT, TAB_STOP_SPACES};

/// Lexer context for different markdown parsing modes.
///
/// Different contexts affect how the lexer tokenizes input:
/// - `Regular`: Normal markdown parsing with inline element detection
/// - `FencedCodeBlock`: Inside fenced code block, no markdown parsing
/// - `HtmlBlock`: Inside HTML block, minimal markdown parsing
/// - `LinkDefinition`: Inside link reference definition, whitespace separates tokens
/// - `CodeSpan`: Inside inline code span, backslashes are literal (no escapes)
/// - `EmphasisInline`: Emit single STAR/UNDERSCORE tokens for partial delimiter consumption
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MarkdownLexContext {
    /// Normal markdown parsing with full inline element detection.
    #[default]
    Regular,
    /// Inside a fenced code block - content is treated as raw text.
    /// No markdown parsing occurs within fenced code blocks.
    /// Reserved for context-aware lexing in fenced code blocks.
    #[expect(dead_code)]
    FencedCodeBlock,
    /// Inside an HTML block - content is treated as raw HTML.
    /// Minimal markdown parsing, primarily looking for block end conditions.
    /// Reserved for context-aware lexing in HTML blocks.
    #[expect(dead_code)]
    HtmlBlock,
    /// Inside a link reference definition (after `]:`).
    /// In this context, whitespace is significant and separates destination from title.
    /// Text tokens stop at whitespace to allow proper parsing.
    LinkDefinition,
    /// Inside an inline code span.
    /// Per CommonMark §6.1, backslash escapes are not processed inside code spans.
    /// Backslash is treated as a literal character, not an escape.
    CodeSpan,
    /// Inside emphasis delimiter processing.
    /// In this context, `*` and `_` are always emitted as single-character tokens
    /// (STAR, UNDERSCORE) rather than double tokens (DOUBLE_STAR, DOUBLE_UNDERSCORE).
    /// This allows partial consumption of delimiter runs when the match algorithm
    /// determines only 1 char should be used from a 2-char run.
    EmphasisInline,
}

impl LexContext for MarkdownLexContext {
    /// Returns true if this is [MarkdownLexContext::Regular]
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

/// Context in which the [MarkdownLexContext]'s current should be re-lexed.
/// Used for re-lexing scenarios where context changes how tokens are parsed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MarkdownReLexContext {
    /// Re-lex using regular markdown rules.
    #[expect(dead_code)]
    Regular,
    /// Re-lex for link definition context where whitespace is significant.
    LinkDefinition,
    /// Re-lex for emphasis inline context where `*` and `_` emit single tokens.
    /// Used when the emphasis matching algorithm needs to partially consume
    /// a DOUBLE_STAR or DOUBLE_UNDERSCORE token.
    EmphasisInline,
}

/// An extremely fast, lookup table based, lossless Markdown lexer
#[derive(Debug)]
pub(crate) struct MarkdownLexer<'src> {
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
    current_kind: MarkdownSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,
    force_ordered_list_marker: bool,
}

impl<'src> Lexer<'src> for MarkdownLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;

    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = MarkdownSyntaxKind;
    type LexContext = MarkdownLexContext;
    type ReLexContext = MarkdownReLexContext;

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
            Some(current) => self.consume_token(current, context),
            None => EOF,
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        // Reset after_newline for non-trivia tokens, except NEWLINE itself.
        // NEWLINE sets after_newline=true in consume_newline() and we preserve that.
        // This ensures the *next* token (after NEWLINE) has PRECEDING_LINE_BREAK set.
        if !kind.is_trivia()
            && kind != NEWLINE
            && kind != MD_HARD_LINE_LITERAL
            && !(kind == MD_TEXTUAL_LITERAL
                && self.after_newline
                && self.current_text_is_whitespace())
        {
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
            after_whitespace: _,
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

impl<'src> MarkdownLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            // Start of document is treated as start of line for indentation purposes
            after_newline: true,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            diagnostics: vec![],
            force_ordered_list_marker: false,
        }
    }

    pub fn set_force_ordered_list_marker(&mut self, value: bool) {
        self.force_ordered_list_marker = value;
    }

    pub(crate) fn consume_token(
        &mut self,
        current: u8,
        context: MarkdownLexContext,
    ) -> MarkdownSyntaxKind {
        let dispatched = lookup_byte(current);
        match dispatched {
            // Whitespace handling depends on context:
            // - At start of line (after_newline): whitespace is significant for indentation
            //   detection (e.g., 4+ spaces = code block), so emit as separate tokens
            // - In middle of line: whitespace is just text content, include in textual token
            // - Exception: 2+ spaces before newline is a hard line break
            // - In LinkDefinition context: whitespace is always significant (separates destination from title)
            // - In CodeSpan context: whitespace is literal content, no hard-line-break detection
            WHS => {
                if current == b'\n' || current == b'\r' {
                    self.consume_newline()
                } else if matches!(context, MarkdownLexContext::CodeSpan) {
                    // In code span context, whitespace is literal content.
                    // No hard-line-break detection - the renderer normalizes line endings to spaces.
                    self.consume_textual(context)
                } else if matches!(context, MarkdownLexContext::LinkDefinition) {
                    // In link definition context, whitespace separates tokens.
                    // We consume it as textual literal so it's not treated as trivia by the parser.
                    self.consume_link_definition_whitespace()
                } else if self.after_newline && matches!(current, b' ' | b'\t') {
                    // At line start, emit single whitespace tokens to allow
                    // indentation handling and quote marker spacing.
                    self.consume_single_whitespace_as_text()
                } else if matches!(current, b' ' | b'\t') && self.is_after_block_quote_marker() {
                    // After a block quote marker, emit a single whitespace token
                    // so the parser can skip the optional space.
                    self.consume_single_whitespace_as_text()
                } else if current == b' ' && self.is_potential_hard_line_break() {
                    // Handle hard line break (2+ spaces before newline) mid-line
                    self.consume_whitespace()
                } else {
                    // Whitespace is part of text in Markdown.
                    self.consume_textual(context)
                }
            }
            MUL | MIN | IDT => self.consume_thematic_break_or_emphasis(dispatched, context),
            PLS => self.consume_byte(PLUS),
            HAS => self.consume_hash(),
            TPL => self.consume_backtick(),
            TLD => self.consume_tilde(),
            MOR => self.consume_byte(R_ANGLE),
            LSS => self.consume_byte(L_ANGLE),
            EXL => self.consume_byte(BANG),
            BTO => self.consume_byte(L_BRACK),
            BTC => self.consume_byte(R_BRACK),
            PNO => self.consume_byte(L_PAREN),
            PNC => self.consume_byte(R_PAREN),
            COL => self.consume_byte(COLON),
            AMP => self.consume_entity_or_textual(context),
            BSL => {
                // Per CommonMark §6.1, backslash escapes are NOT processed inside code spans.
                // Backslash is literal, so `\`` produces a literal backslash followed by backtick.
                if matches!(context, MarkdownLexContext::CodeSpan) {
                    self.consume_textual(context)
                } else {
                    self.consume_escape()
                }
            }
            // = at line start could be setext heading underline
            EQL if self.after_newline => self.consume_setext_underline_or_textual(),
            _ => {
                // Check for ordered list markers: digits followed by . or ) at line start
                if current.is_ascii_digit()
                    && (self.after_newline || self.force_ordered_list_marker)
                {
                    self.consume_ordered_list_marker_or_textual()
                } else {
                    self.consume_textual(context)
                }
            }
        }
    }

    /// Consume a backslash escape sequence.
    ///
    /// Per CommonMark spec:
    /// - Backslash before ASCII punctuation makes it literal
    /// - Backslash before newline is a hard line break
    ///
    /// Escapable: `!"#$%&'()*+,-./:;<=>?@[\]^_\`{|}~`
    fn consume_escape(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        // Consume the backslash
        self.advance(1);

        // Check for hard line break: backslash followed by newline
        if matches!(self.current_byte(), Some(b'\n' | b'\r')) {
            match self.current_byte() {
                Some(b'\n') => {
                    self.advance(1);
                }
                Some(b'\r') => {
                    if self.peek_byte() == Some(b'\n') {
                        self.advance(2);
                    } else {
                        self.advance(1);
                    }
                }
                _ => {}
            }
            self.after_newline = true;
            return MD_HARD_LINE_LITERAL;
        }

        // Check if next character is escapable ASCII punctuation
        if let Some(next) = self.current_byte()
            && matches!(
                next,
                b'!' | b'"'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'('
                    | b')'
                    | b'*'
                    | b'+'
                    | b','
                    | b'-'
                    | b'.'
                    | b'/'
                    | b':'
                    | b';'
                    | b'<'
                    | b'='
                    | b'>'
                    | b'?'
                    | b'@'
                    | b'['
                    | b'\\'
                    | b']'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'{'
                    | b'|'
                    | b'}'
                    | b'~'
            )
        {
            // Consume the escaped character too
            self.advance(1);
        }

        MD_TEXTUAL_LITERAL
    }

    /// Try to consume an entity or numeric character reference per CommonMark §6.2.
    ///
    /// Valid patterns:
    /// - Named entity: `&name;` where name is 2-31 alphanumeric chars starting with letter
    /// - Decimal numeric: `&#digits;` where digits is 1-7 decimal digits
    /// - Hexadecimal: `&#xhex;` or `&#Xhex;` where hex is 1-6 hex digits
    ///
    /// If not valid, falls back to consuming as textual.
    fn consume_entity_or_textual(&mut self, context: MarkdownLexContext) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();
        debug_assert!(matches!(self.current_byte(), Some(b'&')));

        // Try to match entity reference pattern
        if let Some(entity_len) = self.match_entity_reference() {
            self.advance(entity_len);
            return MD_ENTITY_LITERAL;
        }

        // Not a valid entity - consume as textual
        self.consume_textual(context)
    }

    /// Check if text at current position matches a valid entity reference pattern.
    /// Returns the length of the entity if valid, None otherwise.
    ///
    /// Patterns per CommonMark §6.2:
    /// - Named: `&name;` where name is 2-31 alphanumeric chars starting with letter
    /// - Decimal: `&#digits;` where digits is 1-7 decimal digits
    /// - Hex: `&#xhex;` or `&#Xhex;` where hex is 1-6 hex digits
    fn match_entity_reference(&self) -> Option<usize> {
        // Must start with &
        if self.byte_at(0) != Some(b'&') {
            return None;
        }

        let next = self.byte_at(1)?;

        if next == b'#' {
            // Numeric character reference
            self.match_numeric_entity()
        } else if next.is_ascii_alphabetic() {
            // Named entity reference
            self.match_named_entity()
        } else {
            None
        }
    }

    /// Match a named entity reference: `&name;`
    /// Name must be 2-31 alphanumeric chars starting with a letter.
    fn match_named_entity(&self) -> Option<usize> {
        self.match_entity_with(1, 2, 31, |byte, index| {
            if index == 0 {
                byte.is_ascii_alphabetic()
            } else {
                byte.is_ascii_alphanumeric()
            }
        })
    }

    /// Match a numeric entity reference: `&#digits;` or `&#xhex;` / `&#Xhex;`
    fn match_numeric_entity(&self) -> Option<usize> {
        // Position 0 is '&', position 1 is '#'
        let next = self.byte_at(2)?;

        if next == b'x' || next == b'X' {
            // Hexadecimal: &#xhex; or &#Xhex;
            self.match_hex_entity()
        } else if next.is_ascii_digit() {
            // Decimal: &#digits;
            self.match_decimal_entity()
        } else {
            None
        }
    }

    /// Match a decimal numeric entity: `&#digits;` (1-7 decimal digits)
    fn match_decimal_entity(&self) -> Option<usize> {
        self.match_entity_with(2, 1, 7, |byte, _| byte.is_ascii_digit())
    }

    /// Match a hexadecimal numeric entity: `&#xhex;` or `&#Xhex;` (1-6 hex digits)
    fn match_hex_entity(&self) -> Option<usize> {
        self.match_entity_with(3, 1, 6, |byte, _| byte.is_ascii_hexdigit())
    }

    fn match_entity_with<F>(
        &self,
        start: usize,
        min_len: usize,
        max_len: usize,
        is_valid: F,
    ) -> Option<usize>
    where
        F: Fn(u8, usize) -> bool,
    {
        let mut i = start;
        let mut count = 0usize;

        while let Some(byte) = self.byte_at(i) {
            if byte == b';' {
                if (min_len..=max_len).contains(&count) {
                    return Some(i + 1);
                }
                return None;
            }

            if is_valid(byte, count) {
                count += 1;
                if count > max_len {
                    return None;
                }
                i += 1;
            } else {
                return None;
            }
        }

        None
    }

    fn text_position(&self) -> TextSize {
        TextSize::try_from(self.position).expect("Input to be smaller than 4 GB")
    }

    /// Returns true if the current token text is only spaces or tabs.
    fn current_text_is_whitespace(&self) -> bool {
        let start = u32::from(self.current_start) as usize;
        let end = self.position;
        if start >= end || end > self.source.len() {
            return false;
        }
        self.source.as_bytes()[start..end]
            .iter()
            .all(|b| *b == b' ' || *b == b'\t')
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    /// Reserved for single-byte token consumption patterns.
    #[expect(dead_code)]
    fn eat_byte(&mut self, tok: MarkdownSyntaxKind) -> MarkdownSyntaxKind {
        self.advance(1);
        tok
    }
    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source()
            .as_bytes()
            .get(self.position() + offset)
            .copied()
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        match self.current_byte() {
            Some(b'\n') => {
                self.advance(1);
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2)
                } else {
                    self.advance(1)
                }
            }
            _ => unreachable!(),
        }
        self.after_newline = true;
        NEWLINE
    }

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    /// If there are 2+ spaces followed by a newline, emits MD_HARD_LINE_LITERAL.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_whitespace(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let mut space_count = 0;
        while let Some(b' ') = self.current_byte() {
            self.advance(1);
            space_count += 1;
        }

        // Check for hard line break: 2+ spaces followed by newline
        if space_count >= 2 && matches!(self.current_byte(), Some(b'\n' | b'\r')) {
            // Consume the newline as part of the hard line break
            match self.current_byte() {
                Some(b'\n') => {
                    self.advance(1);
                }
                Some(b'\r') => {
                    if self.peek_byte() == Some(b'\n') {
                        self.advance(2);
                    } else {
                        self.advance(1);
                    }
                }
                _ => {}
            }
            self.after_newline = true;
            return MD_HARD_LINE_LITERAL;
        }

        WHITESPACE
    }

    /// Consumes whitespace in LinkDefinition context as textual literal.
    /// This prevents it from being treated as trivia by the parser, which is critical
    /// for correctly parsing link reference definitions where whitespace is a significant separator.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_link_definition_whitespace(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        while let Some(byte) = self.current_byte() {
            if byte == b' ' || byte == b'\t' {
                self.advance(1);
            } else {
                break;
            }
        }

        MD_TEXTUAL_LITERAL
    }

    /// Consume a single whitespace character at line start as text.
    fn consume_single_whitespace_as_text(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        if matches!(self.current_byte(), Some(b' ' | b'\t')) {
            self.advance(1);
        }

        MD_TEXTUAL_LITERAL
    }

    /// Returns true if the current whitespace follows a block quote marker at line start.
    fn is_after_block_quote_marker(&self) -> bool {
        if self.position == 0 {
            return false;
        }

        let bytes = self.source.as_bytes();
        let prev_pos = self.position - 1;
        if bytes.get(prev_pos) != Some(&b'>') {
            return false;
        }

        let before = &self.source[..prev_pos];
        let last_newline_pos = before.rfind(['\n', '\r']);
        let line_start = match last_newline_pos {
            Some(pos) => {
                let before_bytes = before.as_bytes();
                if before_bytes.get(pos) == Some(&b'\r')
                    && before_bytes.get(pos + 1) == Some(&b'\n')
                {
                    pos + 2
                } else {
                    pos + 1
                }
            }
            None => 0,
        };

        let prefix = &self.source[line_start..=prev_pos];

        let mut chars = prefix.chars().peekable();
        let mut indent = 0usize;

        while let Some(&c) = chars.peek() {
            if c == ' ' || c == '\t' {
                indent += if c == '\t' { TAB_STOP_SPACES } else { 1 };
                if indent > MAX_BLOCK_PREFIX_INDENT {
                    return false;
                }
                chars.next();
            } else {
                break;
            }
        }

        let mut saw_marker = false;
        while let Some(c) = chars.next() {
            if c == '>' {
                saw_marker = true;
                if let Some(&next) = chars.peek()
                    && (next == ' ' || next == '\t')
                {
                    chars.next();
                }
            } else {
                return false;
            }
        }

        saw_marker
    }

    /// Consumes thematic break literal, setext underline, or returns emphasis marker tokens.
    /// Called when we see *, -, or _.
    ///
    /// For `-` at line start:
    /// - 1-2 dashes followed by newline: setext underline (H2)
    /// - 3+ dashes followed by newline: thematic break (not setext; the parser may
    ///   convert dash-only thematic breaks to setext when preceded by a paragraph)
    fn consume_thematic_break_or_emphasis(
        &mut self,
        dispatched: Dispatch,
        context: MarkdownLexContext,
    ) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let start_char = match dispatched {
            MUL => b'*',
            MIN => b'-',
            IDT => {
                // IDT can match letters (A-Z, a-z) or underscore
                // Only underscore should be treated as emphasis marker
                match self.current_byte() {
                    Some(b'_') => b'_',
                    _ => return self.consume_textual(context),
                }
            }
            _ => return self.consume_textual(context),
        };

        // Save position to restore if not a thematic break
        let start_position = self.position;

        // For `-` at line start with 1-2 dashes, emit setext underline.
        // 3+ dashes could be thematic break, so let that logic handle it.
        // The parser may convert dash-only thematic breaks to setext when preceded by paragraph.
        if start_char == b'-' && self.after_newline {
            let mut dash_count = 0;
            // Consume only `-` characters (no spaces between)
            while matches!(self.current_byte(), Some(b'-')) {
                self.advance(1);
                dash_count += 1;
            }
            // Allow trailing spaces/tabs
            while matches!(self.current_byte(), Some(b' ' | b'\t')) {
                self.advance(1);
            }
            // 1-2 dashes followed by newline/EOF is a setext underline
            // 3+ dashes goes to thematic break logic below
            if (1..=2).contains(&dash_count)
                && matches!(self.current_byte(), Some(b'\n' | b'\r') | None)
            {
                return MD_SETEXT_UNDERLINE_LITERAL;
            }
            // Not a setext underline - restore and try thematic break
            self.position = start_position;
        }

        let mut count = 0;
        loop {
            // Count only the marker characters, skip whitespace
            if matches!(self.current_byte(), Some(ch) if ch == start_char) {
                self.advance(1);
                count += 1;
            } else if matches!(self.current_byte(), Some(b' ' | b'\t')) {
                self.advance(1);
            } else {
                break;
            }
        }

        // Check if this is a valid thematic break: 3+ of same char, only whitespace between,
        // followed by newline or EOF, AND must be at line start (CommonMark requirement)
        if self.after_newline
            && count >= 3
            && matches!(self.current_byte(), Some(b'\n' | b'\r') | None)
        {
            return MD_THEMATIC_BREAK_LITERAL;
        }

        // Not a thematic break - restore position and consume as emphasis marker
        self.position = start_position;

        // In EmphasisInline context, always emit single tokens for * and _.
        // This allows partial consumption of delimiter runs when the match algorithm
        // determines only 1 char should be used from a 2-char run.
        if matches!(context, MarkdownLexContext::EmphasisInline) {
            self.advance(1);
            return match start_char {
                b'*' => STAR,
                b'_' => UNDERSCORE,
                b'-' => MINUS,
                _ => unreachable!(),
            };
        }

        // Check for double emphasis markers (**, __)
        // Note: -- is not valid markdown emphasis, so we don't check for it
        if start_char != b'-' && self.peek_byte() == Some(start_char) {
            self.advance(2);
            return match start_char {
                b'*' => DOUBLE_STAR,
                b'_' => DOUBLE_UNDERSCORE,
                _ => unreachable!(),
            };
        }

        // Single marker
        self.advance(1);
        match start_char {
            b'*' => STAR,
            b'_' => UNDERSCORE,
            b'-' => MINUS,
            _ => unreachable!(),
        }
    }

    /// Try to consume an ordered list marker (e.g., "1.", "2)", "10.").
    /// Returns MD_ORDERED_LIST_MARKER if valid, otherwise falls back to textual.
    /// Per CommonMark: 1-9 digits followed by `.` or `)` followed by whitespace.
    fn consume_ordered_list_marker_or_textual(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let start_position = self.position;
        let mut digit_count = 0;

        // Consume 1-9 digits
        while let Some(byte) = self.current_byte() {
            if byte.is_ascii_digit() {
                self.advance(1);
                digit_count += 1;
                // CommonMark limits to 9 digits max
                if digit_count > 9 {
                    // Too many digits, not a valid marker
                    self.position = start_position;
                    return self.consume_textual(MarkdownLexContext::Regular);
                }
            } else {
                break;
            }
        }

        // Must have at least one digit
        if digit_count == 0 {
            self.position = start_position;
            return self.consume_textual(MarkdownLexContext::Regular);
        }

        // Must be followed by . or )
        let delimiter = self.current_byte();
        if !matches!(delimiter, Some(b'.' | b')')) {
            self.position = start_position;
            return self.consume_textual(MarkdownLexContext::Regular);
        }
        self.advance(1);

        // Must be followed by at least one space (or end of line for edge cases)
        // Per CommonMark, a space is required but we also allow tab
        if !matches!(self.current_byte(), Some(b' ' | b'\t') | None)
            && !matches!(self.current_byte(), Some(b'\n' | b'\r'))
        {
            self.position = start_position;
            return self.consume_textual(MarkdownLexContext::Regular);
        }

        MD_ORDERED_LIST_MARKER
    }

    /// Try to consume a setext heading underline (line of `=` characters).
    /// Returns MD_SETEXT_UNDERLINE_LITERAL if valid, otherwise falls back to textual.
    /// Per CommonMark: one or more `=` characters with optional spaces, nothing else on line.
    fn consume_setext_underline_or_textual(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        let start_position = self.position;
        let mut eq_count = 0;

        // Consume only `=` characters — no spaces between (CommonMark §4.3)
        while let Some(b'=') = self.current_byte() {
            self.advance(1);
            eq_count += 1;
        }

        // Allow optional trailing whitespace only
        while matches!(self.current_byte(), Some(b' ' | b'\t')) {
            self.advance(1);
        }

        // Must have at least one `=` and nothing else before newline or EOF
        if eq_count >= 1 && matches!(self.current_byte(), Some(b'\n' | b'\r') | None) {
            return MD_SETEXT_UNDERLINE_LITERAL;
        }

        // Not a valid setext underline - restore position and consume as textual
        self.position = start_position;
        self.consume_textual(MarkdownLexContext::Regular)
    }

    /// Consume hash character(s).
    ///
    /// Emits a single HASH token containing all consecutive `#` characters.
    /// The parser can determine the heading level by checking the token's length.
    ///
    /// Per CommonMark §4.2: ATX headings use 1-6 `#` characters.
    fn consume_hash(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        // Count consecutive hash characters
        let mut count = 0;
        while let Some(b'#') = self.byte_at(count) {
            count += 1;
        }

        // Emit all consecutive hashes as a single HASH token
        // The parser determines heading level from token length
        self.advance(count);
        HASH
    }

    /// Consume backtick(s).
    ///
    /// At line start with 3+ backticks: emits TRIPLE_BACKTICK for fenced code blocks.
    /// Otherwise: emits BACKTICK containing all consecutive backticks (for inline code spans).
    ///
    /// This allows multi-backtick code spans like `` `code` `` where the parser
    /// can determine backtick count from token text length.
    fn consume_backtick(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        // Count consecutive backticks
        let mut count = 0;
        while let Some(b'`') = self.byte_at(count) {
            count += 1;
        }

        // At line start with 3+ backticks: fenced code block
        if self.after_newline && count >= 3 {
            self.advance(count);
            return TRIPLE_BACKTICK;
        }

        // Otherwise: emit all consecutive backticks as a single BACKTICK token
        // The parser can determine the count from token text length
        self.advance(count);
        BACKTICK
    }

    /// Consume tilde(s) - either single for other uses or triple for fenced code blocks.
    ///
    /// At line start with 3+ tildes: emits TRIPLE_TILDE for fenced code blocks.
    /// Otherwise: emits TILDE containing all consecutive tildes.
    fn consume_tilde(&mut self) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        // Count consecutive tildes
        let mut count = 0;
        while let Some(b'~') = self.byte_at(count) {
            count += 1;
        }

        // At line start with 3+ tildes: fenced code block
        if self.after_newline && count >= 3 {
            self.advance(count);
            return TRIPLE_TILDE;
        }

        // Otherwise: emit all consecutive tildes as a single TILDE token
        self.advance(count);
        TILDE
    }

    /// Get the UTF8 char which starts at the current byte
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn current_char_unchecked(&self) -> char {
        // Precautionary measure for making sure the unsafe code below does not read over memory boundary
        debug_assert!(!self.is_eof());
        self.assert_at_char_boundary();

        // Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
        let string = unsafe {
            std::str::from_utf8_unchecked(self.source.as_bytes().get_unchecked(self.position..))
        };
        if let Some(chr) = string.chars().next() {
            chr
        } else {
            unreachable!("lexer expected a valid UTF-8 character at current position");
        }
    }

    /// Gets the current byte.
    ///
    /// ## Returns
    /// The current byte if the lexer isn't at the end of the file.
    #[inline]
    fn current_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.source.as_bytes()[self.position])
        }
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    /// Returns `true` if the parser is at or passed the end of the file.
    #[inline]
    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Consume consecutive textual characters until we hit a special markdown character.
    /// This groups multiple characters into a single MD_TEXTUAL_LITERAL token for efficiency.
    /// Spaces and tabs are included in the text token (treated as regular text content),
    /// but newlines end the token since they have semantic meaning as block separators.
    /// Also stops before trailing spaces that could form a hard line break (2+ spaces before newline).
    /// In LinkDefinition context, stops at any whitespace to allow proper destination/title parsing.
    #[inline]
    fn consume_textual(&mut self, context: MarkdownLexContext) -> MarkdownSyntaxKind {
        self.assert_at_char_boundary();

        if self.force_ordered_list_marker
            && let Some(byte) = self.current_byte()
            && (byte == b' ' || byte == b'\t')
        {
            let mut idx = self.position;
            while matches!(self.source.as_bytes().get(idx), Some(b' ' | b'\t')) {
                idx += 1;
            }
            if self.is_ordered_list_marker_at(idx) {
                self.advance(idx - self.position);
                return MD_TEXTUAL_LITERAL;
            }
        }

        // Consume at least one character
        let char = self.current_char_unchecked();
        self.advance(char.len_utf8());

        // Continue consuming characters until we hit a special markdown character
        // or end of file. Special characters are those that could start inline elements
        // or block structures: * - _ + # ` ~ > ! [ ] ( ) \ and newlines.
        // Spaces and tabs are now included as regular text content (except in LinkDefinition context).
        while let Some(byte) = self.current_byte() {
            if matches!(context, MarkdownLexContext::LinkDefinition)
                && (byte == b' ' || byte == b'\t')
            {
                break;
            }
            let dispatched = lookup_byte(byte);
            match dispatched {
                // Include spaces and tabs in text tokens, but check for hard line break pattern.
                // In LinkDefinition context, stop at whitespace to separate destination from title.
                WHS => {
                    if matches!(context, MarkdownLexContext::LinkDefinition) {
                        // In link definition context, whitespace ends the text token
                        break;
                    } else if byte == b' ' {
                        if self.is_at_trailing_hash_closing_whitespace() {
                            break;
                        }
                        // Look ahead to check if this could be the start of a hard line break
                        // (2+ spaces followed by newline)
                        if self.is_potential_hard_line_break() {
                            break;
                        }
                        self.advance(1);
                    } else if byte == b'\t' {
                        if self.is_at_trailing_hash_closing_whitespace() {
                            break;
                        }
                        self.advance(1);
                    } else {
                        // Stop at newlines (\n, \r)
                        break;
                    }
                }
                // Stop at characters that could be markdown syntax
                MUL  // *
                | MIN  // -
                | PLS  // +
                | HAS  // #
                | TPL  // `
                | TLD  // ~
                | LSS  // <
                | MOR  // >
                | EXL  // !
                | BTO  // [
                | BTC  // ]
                | PNO  // (
                | PNC  // )
                | BSL  // \
                | AMP  // & (entity references)
                => break,
                // IDT includes A-Z, a-z, and _ - only _ is special for markdown
                IDT => {
                    if byte == b'_' {
                        break;
                    }
                    self.advance_char_unchecked();
                }
                // All other characters are regular text
                _ => {
                    self.advance_char_unchecked();
                }
            }
        }

        MD_TEXTUAL_LITERAL
    }

    fn is_ordered_list_marker_at(&self, idx: usize) -> bool {
        if idx >= self.source.len() {
            return false;
        }

        let bytes = self.source.as_bytes();
        let mut pos = idx;
        let mut digit_count = 0;

        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            digit_count += 1;
            if digit_count > 9 {
                return false;
            }
            pos += 1;
        }

        if digit_count == 0 {
            return false;
        }

        if pos >= bytes.len() || !(bytes[pos] == b'.' || bytes[pos] == b')') {
            return false;
        }
        pos += 1;

        if pos >= bytes.len() {
            return true;
        }

        matches!(bytes[pos], b' ' | b'\t' | b'\n' | b'\r')
    }

    /// Returns true if the current whitespace is the start of a closing ATX hash sequence.
    ///
    /// Pattern: spaces/tabs + one or more `#` + optional spaces/tabs + newline/EOF.
    fn is_at_trailing_hash_closing_whitespace(&self) -> bool {
        let mut i = self.position;
        let bytes = self.source.as_bytes();
        let len = bytes.len();

        let mut saw_ws = false;
        while i < len {
            let b = bytes[i];
            if b == b' ' || b == b'\t' {
                saw_ws = true;
                i += 1;
            } else {
                break;
            }
        }

        if !saw_ws {
            return false;
        }

        if i >= len || bytes[i] != b'#' {
            return false;
        }

        while i < len && bytes[i] == b'#' {
            i += 1;
        }

        while i < len {
            let b = bytes[i];
            if b == b' ' || b == b'\t' {
                i += 1;
            } else {
                break;
            }
        }

        if i >= len {
            return true;
        }

        matches!(bytes[i], b'\n' | b'\r')
    }

    /// Check if current position starts a potential hard line break pattern.
    /// Returns true if there are 2+ spaces followed by a newline.
    fn is_potential_hard_line_break(&self) -> bool {
        // Must have at least one space at current position (already checked by caller)
        let mut offset = 0;
        let mut space_count = 0;

        // Count consecutive spaces
        while let Some(b' ') = self.byte_at(offset) {
            space_count += 1;
            offset += 1;
        }

        // Check if followed by newline
        if space_count >= 2
            && let Some(next) = self.byte_at(offset)
        {
            return next == b'\n' || next == b'\r';
        }

        false
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn consume_byte(&mut self, tok: MarkdownSyntaxKind) -> MarkdownSyntaxKind {
        self.advance(1);
        tok
    }
}

impl<'src> ReLexer<'src> for MarkdownLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let lex_context = match context {
            MarkdownReLexContext::Regular => MarkdownLexContext::Regular,
            MarkdownReLexContext::LinkDefinition => MarkdownLexContext::LinkDefinition,
            MarkdownReLexContext::EmphasisInline => MarkdownLexContext::EmphasisInline,
        };

        let re_lexed_kind = match self.current_byte() {
            Some(current) => self.consume_token(current, lex_context),
            None => EOF,
        };

        let new_position = self.position;
        if self.current() == re_lexed_kind && new_position == old_position {
            // Didn't re-lex anything. Return existing token again.
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for MarkdownLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            after_whitespace: false,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}
