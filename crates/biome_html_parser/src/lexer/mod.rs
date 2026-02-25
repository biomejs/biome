mod tests;

use crate::token_source::{
    HtmlEmbeddedLanguage, HtmlLexContext, HtmlReLexContext, RestrictedExpressionStopAt,
    TextExpressionKind,
};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T, TextLen, TextSize};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags};
use biome_rowan::SyntaxKind;
use biome_unicode_table::{Dispatch::*, lookup_byte};
use std::ops::Add;

pub(crate) struct HtmlLexer<'src> {
    /// Source text
    source: &'src str,
    /// The start byte position in the source text of the next token.
    position: usize,
    current_kind: HtmlSyntaxKind,
    current_start: TextSize,
    diagnostics: Vec<ParseDiagnostic>,
    current_flags: TokenFlags,
    preceding_line_break: bool,
    after_newline: bool,
    unicode_bom_length: usize,
    /// Set to `true` after the Astro frontmatter closing fence (`---`) has been
    /// consumed. Once set, the `Regular` context will no longer treat `---` as a
    /// `FENCE` token, allowing `---` to appear as plain text in HTML content.
    after_frontmatter: bool,
}

enum IdentifierContext {
    None,
    Doctype,
    Vue,
    VueDirectiveArgument,
    Astro,
}

impl IdentifierContext {
    const fn is_doctype(&self) -> bool {
        matches!(self, Self::Doctype)
    }

    const fn is_astro(&self) -> bool {
        matches!(self, Self::Astro)
    }
}

impl<'src> HtmlLexer<'src> {
    pub fn from_str(string: &'src str) -> Self {
        Self {
            source: string,
            position: 0,
            diagnostics: vec![],
            current_start: TextSize::from(0),
            current_kind: TOMBSTONE,
            preceding_line_break: false,
            after_newline: false,
            current_flags: TokenFlags::empty(),
            unicode_bom_length: 0,
            after_frontmatter: false,
        }
    }

    /// Sets the `after_frontmatter` flag. When `true`, `---` in the `Regular`
    /// context is treated as plain HTML text rather than a `FENCE` token.
    pub fn set_after_frontmatter(&mut self, value: bool) {
        self.after_frontmatter = value;
    }

    /// Consume a token in the [HtmlLexContext::InsideTag] context.
    fn consume_token_inside_tag(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => self.consume_byte(T![/]),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            BEO if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            BEO => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            BEC => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            QOT => self.consume_string_literal(current),
            _ if self.current_kind == T![<] && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                self.consume_tag_name(current)
            }
            _ if self.current_kind != T![<] && is_attribute_name_byte(current) => {
                self.consume_identifier(current, IdentifierContext::None)
            }
            IDT => self
                .consume_language_identifier(current)
                .unwrap_or_else(|| self.consume_unexpected_character()),
            _ => {
                if self.position == 0
                    && let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM)
                {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_unexpected_character()
            }
        }
    }

    /// Consume a token in the [HtmlLexContext::InsideTagAstro] context.
    /// This context is used for Astro files with Astro-specific directives (client:, set:, etc.)
    /// It handles colons as separate tokens to enable directive parsing.
    fn consume_token_inside_tag_astro(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => self.consume_byte(T![/]),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            // Handle colons as separate tokens for Astro directives
            COL => self.consume_byte(T![:]),
            BEO if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            BEO => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            BEC => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            QOT => self.consume_string_literal(current),
            _ if self.current_kind == T![<] && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                self.consume_tag_name(current)
            }
            _ if self.current_kind != T![<] && is_attribute_name_byte(current) => {
                self.consume_identifier(current, IdentifierContext::Astro)
            }
            IDT => self
                .consume_language_identifier(current)
                .unwrap_or_else(|| self.consume_unexpected_character()),
            _ => {
                if self.position == 0
                    && let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM)
                {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_unexpected_character()
            }
        }
    }

    /// Consume a token in the [HtmlLexContext::InsideTagWithDirectives] context.
    /// This context is used for Vue files with Vue-specific directives.
    /// When `svelte` is `true`, also handles `//` and `/* */` as JS-style comments.
    fn consume_token_inside_tag_directives(&mut self, current: u8, svelte: bool) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => {
                if svelte {
                    match self.byte_at(1).map(lookup_byte) {
                        Some(SLH) => return self.consume_js_line_comment(),
                        Some(MUL) => return self.consume_js_block_comment(),
                        _ => {}
                    }
                }
                self.consume_byte(T![/])
            }
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            BEO => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            BEC => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            // Vue and Svelte directives
            COL => self.consume_byte(T![:]),

            // these are used in Vue directives
            AT_ => self.consume_byte(T![@]),
            PRD => self.consume_byte(T![.]),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            HAS => self.consume_byte(T![#]),

            QOT => self.consume_string_literal(current),
            _ if self.current_kind == T![<] && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                self.consume_tag_name(current)
            }
            _ if (self.current_kind == T![@] && is_attribute_name_byte_vue(current)) => {
                self.consume_identifier(current, IdentifierContext::VueDirectiveArgument)
            }
            _ if (self.current_kind != T![<] && is_attribute_name_byte_vue(current)) => {
                self.consume_identifier(current, IdentifierContext::Vue)
            }
            _ => self.consume_unexpected_character(),
        }
    }

    /// Consume a token in the [HtmlLexContext::VueDirectiveArgument] context.
    fn consume_token_vue_directive_argument(&mut self) -> HtmlSyntaxKind {
        let start = self.text_position();
        let mut brackets_stack = 0;
        let mut quotes_seen = QuotesSeen::new();

        while let Some(byte) = self.current_byte() {
            quotes_seen.check_byte(byte);
            let char = biome_unicode_table::lookup_byte(byte);
            use biome_unicode_table::Dispatch::*;

            if quotes_seen.is_empty() {
                match char {
                    BTO => {
                        brackets_stack += 1;
                    }
                    BTC => {
                        if brackets_stack == 0 {
                            break;
                        }
                        brackets_stack -= 1;
                    }
                    _ => {}
                }
            }

            self.advance_byte_or_char(byte);
        }

        if self.text_position() != start {
            HTML_LITERAL
        } else {
            ERROR_TOKEN
        }
    }

    /// Consume a token in the [HtmlLexContext::InsideTagSvelte] context.
    /// This context is used for Svelte files with JS-style comment support.
    fn consume_token_inside_tag_svelte(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        if dispatched == SLH {
            match self.byte_at(1).map(lookup_byte) {
                Some(SLH) => return self.consume_js_line_comment(),
                Some(MUL) => return self.consume_js_block_comment(),
                _ => {}
            }
        }
        self.consume_token_inside_tag(current)
    }

    /// Consume a token in the [HtmlLexContext::Regular] context.
    fn consume_token(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            EXL if self.current() == T![<] => self.consume_byte(T![!]),
            SLH if self.current() == T![<] => self.consume_byte(T![/]),
            COM if self.current() == T![<] => self.consume_byte(T![,]),
            MIN if !self.after_frontmatter && self.at_frontmatter_edge() => {
                self.consume_frontmatter_edge()
            }
            BEO if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            BEO => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            BEC => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            LSS => {
                // if this truly is the start of a tag, it *must* be immediately followed by a tag name. Whitespace is not allowed.
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                if self
                    .peek_byte()
                    .is_some_and(|b| is_tag_start_byte(b) || b == b'!' || b == b'/' || b == b'>')
                {
                    self.consume_l_angle()
                } else {
                    self.push_diagnostic(
                        ParseDiagnostic::new(
                            "Unescaped `<` bracket character. Expected a tag or escaped character.",
                            self.text_position()..self.text_position() + TextSize::from(1),
                        )
                        .with_hint("Replace this character with `&lt;` to escape it."),
                    );
                    self.consume_byte(HTML_LITERAL)
                }
            }
            IDT => self
                .consume_language_identifier(current)
                .unwrap_or_else(|| self.consume_html_text(current)),
            _ => {
                if self.position == 0
                    && let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM)
                {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_html_text(current)
            }
        }
    }

    /// Consume a token in the [HtmlLexContext::AttributeValue] context.
    fn consume_token_attribute_value(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_byte(T![<]),
            MOR => self.consume_byte(T![>]),
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            QOT => self.consume_string_literal(current),
            _ => self.consume_unquoted_string_literal(),
        }
    }

    /// Consume a token in the [HtmlLexContext::Doctype] context.
    fn consume_token_doctype(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_byte(T![<]),
            MOR => self.consume_byte(T![>]),
            EXL => self.consume_byte(T![!]),
            QOT => self.consume_string_literal(current),
            _ if is_tag_name_byte(current) || is_attribute_name_byte(current) => {
                self.consume_identifier(current, IdentifierContext::Doctype)
            }
            _ => self.consume_unexpected_character(),
        }
    }

    /// Consume an embedded language in its entirety. Stops immediately:
    /// - before the closing tag
    /// - before the Astro fence, if applicable
    fn consume_token_embedded_language(
        &mut self,
        _current: u8,
        lang: HtmlEmbeddedLanguage,
        context: HtmlLexContext,
    ) -> HtmlSyntaxKind {
        let start = self.text_position();
        let end_tag = lang.end_tag();
        // double, single, template
        let mut quotes_seen = QuotesSeen::new();
        self.assert_current_char_boundary();
        while let Some(byte) = self.current_byte() {
            if context == HtmlLexContext::AstroFencedCodeBlock {
                quotes_seen.check_byte(byte);
            }

            let end = self.position + end_tag.len();
            let both_ends_at_char_boundaries =
                self.source.is_char_boundary(self.position) && self.source.is_char_boundary(end);
            if both_ends_at_char_boundaries
                && self.source[self.position..end].eq_ignore_ascii_case(end_tag)
            {
                break;
            }
            self.advance_byte_or_char(byte);

            if context == HtmlLexContext::AstroFencedCodeBlock
                && self.at_frontmatter_edge()
                && quotes_seen.is_empty()
            {
                return HTML_LITERAL;
            }
        }

        if self.text_position() != start {
            HTML_LITERAL
        } else {
            // if the element is empty, we will immediately hit the closing tag.
            // we HAVE to consume something, so we start consuming the closing tag.
            self.consume_byte(T![<])
        }
    }

    /// Consumes tokens within a double text expression ('{{...}}') until the closing
    /// delimiter is reached. Returns HTML_LITERAL for the expression content.
    fn consume_double_text_expression(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            BEC if self.at_closing_double_text_expression() => {
                self.consume_r_double_text_expression()
            }
            LSS => self.consume_byte(T![<]),
            _ => {
                while let Some(current) = self.current_byte() {
                    if current == b'}' && self.at_closing_double_text_expression() {
                        break;
                    }
                    self.advance(1);
                }
                HTML_LITERAL
            }
        }
    }

    /// Consumes tokens within a single text expression ('{...}') while tracking nested
    /// brackets until the matching closing bracket is found.
    fn consume_single_text_expression(&mut self) -> HtmlSyntaxKind {
        let mut brackets_stack = 0;
        while let Some(current) = self.current_byte() {
            match current {
                b'}' => {
                    if brackets_stack == 0 {
                        break;
                    } else {
                        brackets_stack -= 1;
                        self.advance(1);
                    }
                }
                b'{' => {
                    brackets_stack += 1;
                    self.advance(1);
                }

                _ => {
                    self.advance(1);
                }
            }
        }

        HTML_LITERAL
    }

    /// Consumes a restricted single text expression that stops at specific keywords
    /// (e.g., 'as' in Svelte #each blocks). Tracks nested brackets and stops when
    /// encountering a keyword at the top level.
    ///
    /// Finding a `{` at the top level will emit an error token.
    fn consume_restricted_single_text_expression(
        &mut self,
        kind: RestrictedExpressionStopAt,
    ) -> HtmlSyntaxKind {
        let start_pos = self.position;
        let mut brackets_stack = 0;

        let is_opening_paren = |byte: u8| byte == b'(' || byte == b'[' || byte == b'{';
        let is_closing_paren = |byte: u8| byte == b')' || byte == b']' || byte == b'}';

        while let Some(current) = self.current_byte() {
            match current {
                // That's usually the case where we find a tag
                _ if current == b'<' && brackets_stack == 0 => break,
                _ if is_opening_paren(current) && !kind.matches_punct(current) => {
                    brackets_stack += 1;
                    self.advance(1)
                }
                _ if is_closing_paren(current) && !kind.matches_punct(current) => {
                    if brackets_stack == 0 {
                        // Reached the closing brace
                        break;
                    } else {
                        brackets_stack -= 1;
                        self.advance(1)
                    }
                }

                _ if brackets_stack == 0 && !is_at_start_identifier(current) => {
                    let should_stop = kind.matches_punct(current);
                    if should_stop {
                        break;
                    }
                    self.advance(1);
                }
                _ if brackets_stack == 0 && is_at_start_identifier(current) => {
                    // Check if we're at a stop keyword
                    let checkpoint_pos = self.position;
                    let prev_byte = self.prev_byte();
                    if let Some(keyword_kind) = self.consume_language_identifier(current) {
                        // Check if this keyword is in our stop list
                        let should_stop =
                            kind.matches_keyword(keyword_kind) && prev_byte == Some(b' ');

                        if should_stop {
                            // Rewind - don't consume the keyword
                            self.position = checkpoint_pos;
                            break;
                        }
                        // Not a stop keyword, continue (position already advanced by consume_language_identifier)
                    } else {
                        // Not a keyword, advance one byte (position was reset by consume_language_identifier)
                        self.advance_byte_or_char(current);
                    }
                }
                _ => {
                    self.advance(1);
                }
            }
        }

        if self.position > start_pos {
            HTML_LITERAL
        } else {
            ERROR_TOKEN
        }
    }

    /// Consumes an HTML comment starting with '<!--' until the closing '-->' is found.
    /// Returns COMMENT token type.
    fn consume_comment(&mut self) -> HtmlSyntaxKind {
        // eat <!--
        self.advance(4);

        while let Some(char) = self.current_byte() {
            if self.at_end_comment() {
                // eat -->
                self.advance(3);
                return COMMENT;
            }
            self.advance_byte_or_char(char);
        }

        COMMENT
    }

    /// Consumes a `//` single-line comment, returning COMMENT.
    /// Does NOT consume the terminating newline — it must be emitted as a
    /// separate NEWLINE trivia token to preserve leading/trailing trivia boundaries.
    fn consume_js_line_comment(&mut self) -> HtmlSyntaxKind {
        self.advance(2);
        while let Some(chr) = self.current_byte() {
            match chr {
                b'\n' | b'\r' => break,
                _ if chr.is_ascii() => self.advance(1),
                _ => {
                    let c = self.current_char_unchecked();
                    if is_linebreak(c) {
                        break;
                    }
                    self.advance(c.len_utf8());
                }
            }
        }
        COMMENT
    }

    /// Consumes a `/* */` block comment, returning COMMENT.
    fn consume_js_block_comment(&mut self) -> HtmlSyntaxKind {
        self.advance(2);
        while let Some(chr) = self.current_byte() {
            let dispatched = lookup_byte(chr);
            match dispatched {
                MUL if self.byte_at(1).map(lookup_byte) == Some(SLH) => {
                    self.advance(2);
                    return COMMENT;
                }
                IDT | ZER | DIG | WHS | COL | SLH | MIN | MUL => self.advance(1),
                _ if chr.is_ascii() => self.advance(1),
                _ => self.advance(self.current_char_unchecked().len_utf8()),
            }
        }
        self.push_diagnostic(ParseDiagnostic::new(
            "Unterminated block comment, expected `*/`",
            self.current_start..self.text_position(),
        ));
        COMMENT
    }

    /// Consume a token in the [HtmlLexContext::CdataSection] context.
    fn consume_inside_cdata(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            LSS if self.at_start_cdata() => self.consume_cdata_start(),
            BTC if self.at_end_cdata() => self.consume_cdata_end(),
            _ => {
                while let Some(char) = self.current_byte() {
                    if self.at_end_cdata() {
                        // eat ]]>
                        break;
                    }
                    self.advance_byte_or_char(char);
                }
                HTML_LITERAL
            }
        }
    }
    /// Consume a token in the [HtmlLexContext::AstroFencedCodeBlock] context until
    /// either the closing `---` fence is reached or a script tag is encountered.
    fn consume_astro_frontmatter(
        &mut self,
        current: u8,
        context: HtmlLexContext,
    ) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS if self.at_start_cdata() => self.consume_cdata_start(),
            LSS => self.consume_byte(T![<]),
            MIN => {
                debug_assert!(self.at_frontmatter_edge());
                self.advance(3);
                self.after_frontmatter = true;
                T![---]
            }
            _ => {
                self.consume_token_embedded_language(current, HtmlEmbeddedLanguage::Script, context)
            }
        }
    }

    fn consume_svelte(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            BEC => self.consume_byte(T!['}']),
            PRD if self.is_at_three_dots() => self.consume_dot3(),
            COM => self.consume_byte(T![,]),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BEO if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            BEO => self.consume_byte(T!['{']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            COL => self.consume_byte(T![:]),
            EQL => self.consume_byte(T![=]),
            PIP => self.consume_byte(T![|]),
            IDT => self
                .consume_language_identifier(current)
                .unwrap_or_else(|| self.consume_svelte_identifier(current)),
            _ => self.consume_single_text_expression(),
        }
    }

    fn consume_svelte_literal(&mut self) -> HtmlSyntaxKind {
        while let Some(current) = self.current_byte() {
            let dispatched = lookup_byte(current);
            if dispatched == WHS || dispatched == EQL || dispatched == MOR {
                break;
            }
            self.advance(1);
        }

        HTML_LITERAL
    }

    /// Consumes a Svelte identifier (alphanumeric + underscore only)
    fn consume_svelte_identifier(&mut self, first: u8) -> HtmlSyntaxKind {
        self.assert_current_char_boundary();
        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            if is_at_continue_identifier(byte) {
                self.advance(1);
            } else {
                break;
            }
        }

        IDENT
    }
    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: HtmlSyntaxKind) -> HtmlSyntaxKind {
        self.advance(1);
        tok
    }

    fn consume_unexpected_character(&mut self) -> HtmlSyntaxKind {
        self.assert_at_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("Unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());

        ERROR_TOKEN
    }

    /// Asserts that the lexer is at a UTF8 char boundary
    #[inline]
    fn assert_at_char_boundary(&self) {
        debug_assert!(self.source.is_char_boundary(self.position));
    }

    /// Attempts to consume HTML-ish languages identifiers. If none is found, the function
    /// restores the position of the lexer and returns [None].
    fn consume_language_identifier(&mut self, first: u8) -> Option<HtmlSyntaxKind> {
        self.assert_current_char_boundary();
        let starting_position = self.position;
        const BUFFER_SIZE: usize = 14;
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0] = first;
        let mut len = 1;

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            if is_at_continue_identifier(byte) {
                if len < BUFFER_SIZE {
                    buffer[len] = byte;
                    len += 1;
                }
                self.advance(1)
            } else {
                break;
            }
        }

        Some(match &buffer[..len] {
            b"debug" => DEBUG_KW,
            b"attach" => ATTACH_KW,
            b"const" => CONST_KW,
            b"render" => RENDER_KW,
            b"html" => HTML_KW,
            b"key" => KEY_KW,
            b"if" => IF_KW,
            b"else" => ELSE_KW,
            b"each" => EACH_KW,
            b"as" => AS_KW,
            b"await" => AWAIT_KW,
            b"then" => THEN_KW,
            b"catch" => CATCH_KW,
            b"snippet" => SNIPPET_KW,
            b"bind" => BIND_KW,
            b"transition" => TRANSITION_KW,
            b"animate" => ANIMATE_KW,
            b"in" => IN_KW,
            b"out" => OUT_KW,
            b"use" => USE_KW,
            b"style" => STYLE_KW,
            b"class" => CLASS_KW,

            _ => {
                self.position = starting_position;
                return None;
            }
        })
    }

    fn consume_identifier(&mut self, first: u8, context: IdentifierContext) -> HtmlSyntaxKind {
        self.assert_current_char_boundary();

        const BUFFER_SIZE: usize = 14;
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0] = first;
        let mut len = 1;

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            match context {
                IdentifierContext::Doctype | IdentifierContext::None => {
                    if is_attribute_name_byte(byte) {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte;
                            len += 1;
                        }

                        self.advance(1)
                    } else {
                        break;
                    }
                }
                IdentifierContext::Astro => {
                    if byte == b':' && is_astro_directive_keyword_bytes(&buffer[..len]) {
                        break;
                    }

                    if is_attribute_name_byte_astro(byte) {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte;
                            len += 1;
                        }

                        self.advance(1)
                    } else {
                        break;
                    }
                }
                IdentifierContext::Vue => {
                    if byte == b':' && is_vue_directive_prefix_bytes(&buffer[..len]) {
                        break;
                    }

                    if is_attribute_name_byte_vue(byte) || byte == b':' {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte;
                            len += 1;
                        }

                        self.advance(1)
                    } else {
                        break;
                    }
                }
                IdentifierContext::VueDirectiveArgument => {
                    if is_attribute_name_byte_vue(byte) || byte == b':' {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte;
                            len += 1;
                        }

                        self.advance(1)
                    } else {
                        break;
                    }
                }
            }
        }

        match &buffer[..len] {
            b"doctype" | b"DOCTYPE" => DOCTYPE_KW,
            b"html" | b"HTML" if context.is_doctype() => HTML_KW,
            b"client" if context.is_astro() && self.current_byte() == Some(b':') => CLIENT_KW,
            b"set" if context.is_astro() && self.current_byte() == Some(b':') => SET_KW,
            b"class" if context.is_astro() && self.current_byte() == Some(b':') => CLASS_KW,
            b"is" if context.is_astro() && self.current_byte() == Some(b':') => IS_KW,
            b"server" if context.is_astro() && self.current_byte() == Some(b':') => SERVER_KW,
            b"define" if context.is_astro() && self.current_byte() == Some(b':') => DEFINE_KW,
            _ => HTML_LITERAL,
        }
    }

    /// Consumes an HTML tag name token starting with the given byte.
    /// Tag names can contain alphanumeric characters, hyphens, colons and dots.
    /// Consumes an HTML tag name token starting with the given byte.
    /// Tag names can contain alphanumeric characters, hyphens, and colons.
    /// In component contexts (Vue/Svelte/Astro), dots are excluded and lexed separately.
    fn consume_tag_name(&mut self, first: u8) -> HtmlSyntaxKind {
        self.assert_current_char_boundary();

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            if is_tag_name_byte(byte) {
                self.advance(1)
            } else {
                break;
            }
        }

        HTML_LITERAL
    }

    /// Consumes a quoted string literal token, handling escaped characters and unicode sequences.
    /// Returns ERROR_TOKEN if the string is not properly terminated or contains invalid escapes.
    fn consume_string_literal(&mut self, quote: u8) -> HtmlSyntaxKind {
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

                        Some(b'u') => match (self.consume_unicode_escape(), state) {
                            (Ok(_), _) => {}
                            (Err(err), LexStringState::InString) => {
                                self.diagnostics.push(err);
                                state = LexStringState::InvalidEscapeSequence;
                            }
                            (Err(_), _) => {}
                        },

                        Some(chr) => {
                            self.advance_byte_or_char(chr);
                        }

                        None => {}
                    }
                }
                // we don't need to handle IDT because it's always len 1.
                UNI => self.advance_char_unchecked(),

                _ => self.advance(1),
            }
        }

        match state {
            LexStringState::Terminated => HTML_STRING_LITERAL,
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

    /// Consume an attribute value that is not quoted.
    ///
    /// See: https://html.spec.whatwg.org/#attributes-2 under "Unquoted attribute value syntax"
    fn consume_unquoted_string_literal(&mut self) -> HtmlSyntaxKind {
        let mut content_started = false;
        let mut encountered_invalid = false;
        while let Some(current) = self.current_byte() {
            match current {
                // these characters safely terminate an unquoted attribute value
                b'\n' | b'\r' | b'\t' | b' ' | b'>' => break,
                // these characters are absolutely invalid in an unquoted attribute value
                b'?' | b'\'' | b'"' | b'=' | b'<' | b'`' => {
                    encountered_invalid = true;
                    break;
                }
                _ if current.is_ascii() => {
                    self.advance(1);
                    content_started = true;
                }
                _ => break,
            }
        }

        if content_started && !encountered_invalid {
            HTML_STRING_LITERAL
        } else {
            let char = self.current_char_unchecked();
            self.push_diagnostic(ParseDiagnostic::new(
                "Unexpected character in unquoted attribute value",
                self.text_position()..self.text_position() + char.text_len(),
            ));
            self.consume_unexpected_character()
        }
    }

    /// Consumes a left angle bracket '<' token, which may start a comment, CDATA section,
    /// or regular tag opening.
    fn consume_l_angle(&mut self) -> HtmlSyntaxKind {
        self.assert_byte(b'<');

        if self.at_start_comment() {
            self.consume_comment()
        } else if self.at_start_cdata() {
            self.consume_cdata_start()
        } else {
            self.consume_byte(T![<])
        }
    }

    /// Consumes an opening double text expression '{{' token used for interpolation.
    fn consume_l_double_text_expression(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_opening_double_text_expression());
        self.advance(2);
        T!["{{"]
    }

    /// Consumes a Svelte opening block token starting with '{' followed by @, #, : or /.
    fn consume_svelte_opening_block(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_svelte_opening_block());
        let next_byte = self.byte_at(1);
        let token = match next_byte {
            Some(b'@') => T!["{@"],
            Some(b'#') => T!["{#"],
            Some(b':') => T!["{:"],
            Some(b'/') => T!["{/"],
            _ => unimplemented!(
                "Svelte block not correctly lexed. Char not expected {:?}",
                next_byte
            ),
        };
        self.advance(2);
        token
    }

    #[inline(always)]
    fn consume_dot3(&mut self) -> HtmlSyntaxKind {
        self.assert_byte(b'.');
        self.advance(3);
        T![...]
    }

    /// Consumes a closing double text expression '}}' token used for interpolation.
    fn consume_r_double_text_expression(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_closing_double_text_expression());
        self.advance(2);
        T!["}}"]
    }

    /// Consumes a frontmatter fence '---' token that delimits Astro frontmatter blocks.
    fn consume_frontmatter_edge(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_frontmatter_edge());
        self.advance(3);
        T![---]
    }

    #[inline(always)]
    fn at_start_comment(&self) -> bool {
        self.current_byte() == Some(b'<')
            && self.byte_at(1) == Some(b'!')
            && self.byte_at(2) == Some(b'-')
            && self.byte_at(3) == Some(b'-')
    }

    #[inline(always)]
    fn at_end_comment(&self) -> bool {
        self.current_byte() == Some(b'-')
            && self.byte_at(1) == Some(b'-')
            && self.byte_at(2) == Some(b'>')
    }

    #[inline(always)]
    fn at_start_cdata(&self) -> bool {
        self.current_byte() == Some(b'<')
            && self.byte_at(1) == Some(b'!')
            && self.byte_at(2) == Some(b'[')
            && self.byte_at(3) == Some(b'C')
            && self.byte_at(4) == Some(b'D')
            && self.byte_at(5) == Some(b'A')
            && self.byte_at(6) == Some(b'T')
            && self.byte_at(7) == Some(b'A')
            && self.byte_at(8) == Some(b'[')
    }
    #[inline(always)]
    fn at_end_cdata(&self) -> bool {
        self.current_byte() == Some(b']')
            && self.byte_at(1) == Some(b']')
            && self.byte_at(2) == Some(b'>')
    }
    #[inline(always)]
    fn at_frontmatter_edge(&self) -> bool {
        self.current_byte() == Some(b'-')
            && self.byte_at(1) == Some(b'-')
            && self.byte_at(2) == Some(b'-')
    }

    #[inline(always)]
    fn at_opening_double_text_expression(&self) -> bool {
        self.current_byte() == Some(b'{') && self.byte_at(1) == Some(b'{')
    }

    #[inline(always)]
    fn at_svelte_opening_block(&self) -> bool {
        self.current_byte() == Some(b'{')
            && (self.byte_at(1) == Some(b'@')
                || self.byte_at(1) == Some(b'#')
                || self.byte_at(1) == Some(b':')
                || self.byte_at(1) == Some(b'/'))
    }

    #[inline(always)]
    fn is_at_three_dots(&self) -> bool {
        self.current_byte() == Some(b'.')
            && self.byte_at(1) == Some(b'.')
            && self.byte_at(2) == Some(b'.')
    }

    #[inline(always)]
    fn at_closing_double_text_expression(&self) -> bool {
        self.current_byte() == Some(b'}') && self.byte_at(1) == Some(b'}')
    }

    /// Consumes the opening CDATA section marker '<![CDATA[' token.
    fn consume_cdata_start(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_start_cdata());

        self.advance(9);
        T!["<![CDATA["]
    }

    /// Consumes the closing CDATA section marker ']]>' token.
    fn consume_cdata_end(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_end_cdata());

        self.advance(3);
        T!["]]>"]
    }

    /// Lexes a `\u0000` escape sequence. Assumes that the lexer is positioned at the `u` token.
    ///
    /// A unicode escape sequence must consist of 4 hex characters.
    fn consume_unicode_escape(&mut self) -> Result<(), ParseDiagnostic> {
        self.assert_byte(b'u');
        self.assert_current_char_boundary();

        let start = self.text_position();

        let start = start
            // Subtract 1 to get position of `\`
            .checked_sub(TextSize::from(1))
            .unwrap_or(start);

        self.advance(1); // Advance over `u`

        for _ in 0..4 {
            match self.current_byte() {
                Some(byte) if byte.is_ascii_hexdigit() => self.advance(1),
                Some(_) => {
                    let char = self.current_char_unchecked();
                    // Reached a non-hex digit which is invalid
                    return Err(ParseDiagnostic::new(

                        "Invalid unicode sequence",
                        start..self.text_position(),
                    )
                        .with_detail(self.text_position()..self.text_position().add(char.text_len()), "Non hexadecimal number")
                        .with_hint("A unicode escape sequence must consist of 4 hexadecimal numbers: `\\uXXXX`, e.g. `\\u002F' for '/'."));
                }
                None => {
                    // Reached the end of the file before processing 4 hex digits
                    return Err(ParseDiagnostic::new(
                        "Unicode escape sequence with two few hexadecimal numbers.",
                        start..self.text_position(),
                    )
                        .with_detail(
                            self.text_position()..self.text_position(),
                            "reached the end of the file",
                        )
                        .with_hint("A unicode escape sequence must consist of 4 hexadecimal numbers: `\\uXXXX`, e.g. `\\u002F' for '/'."));
                }
            }
        }

        Ok(())
    }

    /// Consume a single block of HTML text outside of tags.
    ///
    /// We consider a "block" of text to be a sequence of words, with whitespace
    /// separating them. A block ends when there is 2 newlines, or when a special
    /// character (eg. `<`) is found.
    ///
    /// Spaces between words are treated the same as newlines between words in HTML,
    /// and we don't end a block when we encounter a newline. However, we do not
    /// include leading or trailing whitespace in the block, letting the lexer
    /// consume that whitespace as trivia.
    ///
    /// This makes it easier for users to suppress formatting for specific blocks
    /// of text instead of needing to suppress the entire parent element, which may
    /// not even be present if the text is at the root level.
    ///
    /// - See: <https://html.spec.whatwg.org/#space-separated-tokens>
    /// - See: <https://infra.spec.whatwg.org/#strip-leading-and-trailing-ascii-whitespace>
    fn consume_html_text(&mut self, current: u8) -> HtmlSyntaxKind {
        let mut whitespace_started = None;
        let mut seen_newlines = 0;

        let mut closing_expression = None;
        let mut was_escaped = false;

        let dispatched = lookup_byte(current);

        match dispatched {
            BEO => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            BEC => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            _ => {
                while let Some(current) = self.current_byte() {
                    let dispatched = lookup_byte(current);

                    match dispatched {
                        BEO => {
                            if was_escaped {
                                self.advance(1);
                            } else {
                                break;
                            }
                        }
                        BSL => {
                            was_escaped = true;
                            whitespace_started = None;
                            self.advance(1);
                        }
                        BEC => {
                            if was_escaped {
                                self.advance(1);
                            } else {
                                if let Some(checkpoint) = closing_expression {
                                    self.rewind(checkpoint);
                                    break;
                                }
                                closing_expression = Some(self.checkpoint());
                                whitespace_started = None;
                                self.advance(1);
                            }
                        }

                        LSS => {
                            break;
                        }
                        WHS if current == b'\n' || current == b'\r' => {
                            if whitespace_started.is_none() {
                                whitespace_started = Some(self.checkpoint());
                            }
                            self.after_newline = true;
                            seen_newlines += 1;
                            if seen_newlines > 1 {
                                break;
                            }
                            self.advance(1);
                        }
                        WHS => {
                            if was_escaped {
                                was_escaped = false;
                            }
                            if whitespace_started.is_none() {
                                whitespace_started = Some(self.checkpoint());
                            }
                            closing_expression = None;
                            self.advance(1);
                        }
                        _ => {
                            self.advance(1);
                            whitespace_started = None;
                            seen_newlines = 0;
                        }
                    }
                }

                if let Some(checkpoint) = whitespace_started {
                    // avoid treating the trailing whitespace as part of the token if there is any
                    self.rewind(checkpoint);
                }

                HTML_LITERAL
            }
        }
    }
}

impl<'src> Lexer<'src> for HtmlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;
    type Kind = HtmlSyntaxKind;
    type LexContext = HtmlLexContext;
    type ReLexContext = HtmlReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() {
            EOF
        } else {
            match self.current_byte() {
                Some(current) => match context {
                    HtmlLexContext::Regular => self.consume_token(current),
                    HtmlLexContext::InsideTag => self.consume_token_inside_tag(current),
                    HtmlLexContext::InsideTagWithDirectives { svelte } => {
                        self.consume_token_inside_tag_directives(current, svelte)
                    }
                    HtmlLexContext::InsideTagAstro => self.consume_token_inside_tag_astro(current),
                    HtmlLexContext::InsideTagSvelte => {
                        self.consume_token_inside_tag_svelte(current)
                    }
                    HtmlLexContext::VueDirectiveArgument => {
                        self.consume_token_vue_directive_argument()
                    }
                    HtmlLexContext::AttributeValue => self.consume_token_attribute_value(current),
                    HtmlLexContext::Doctype => self.consume_token_doctype(current),
                    HtmlLexContext::EmbeddedLanguage(lang) => {
                        self.consume_token_embedded_language(current, lang, context)
                    }
                    HtmlLexContext::TextExpression(kind) => match kind {
                        TextExpressionKind::Double => self.consume_double_text_expression(current),
                        TextExpressionKind::Single => self.consume_single_text_expression(),
                    },
                    HtmlLexContext::RestrictedSingleExpression(kind) => {
                        self.consume_restricted_single_text_expression(kind)
                    }
                    HtmlLexContext::CdataSection => self.consume_inside_cdata(current),
                    HtmlLexContext::AstroFencedCodeBlock => {
                        self.consume_astro_frontmatter(current, context)
                    }
                    HtmlLexContext::Svelte => self.consume_svelte(current),
                    HtmlLexContext::SvelteBindingLiteral => self.consume_svelte_literal(),
                },
                None => EOF,
            }
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
        self.preceding_line_break
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

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }
}

impl<'src> ReLexer<'src> for HtmlLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match self.current_byte() {
            Some(current) => match context {
                HtmlReLexContext::Svelte => self.consume_svelte(current),
                HtmlReLexContext::HtmlText => self.consume_html_text(current),
                HtmlReLexContext::InsideTag => self.consume_token_inside_tag(current),
                HtmlReLexContext::InsideTagAstro => self.consume_token_inside_tag_astro(current),
            },
            None => EOF,
        };

        if self.current() == re_lexed_kind && self.position == old_position {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

fn is_tag_name_byte(byte: u8) -> bool {
    // Canonical HTML tag names are specified to be case-insensitive and alphanumeric.
    // https://html.spec.whatwg.org/#elements-2
    // https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
    // However, custom tag names must start with a lowercase letter, but they can be followed by pretty much anything else.
    // https://html.spec.whatwg.org/#valid-custom-element-name

    // The extra characters allowed here `-` and `:` are not usually allowed in the HTML tag name.
    // However, Prettier considers them to be valid characters in tag names, so we allow them to remain compatible.

    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b':'
}

fn is_tag_start_byte(byte: u8) -> bool {
    // Tag names must start with an ASCII letter (not a digit)
    // https://html.spec.whatwg.org/#valid-custom-element-name
    byte.is_ascii_alphabetic()
}

fn is_attribute_name_byte(byte: u8) -> bool {
    // https://html.spec.whatwg.org/#attributes-2
    byte.is_ascii()
        && !byte.is_ascii_control()
        && !matches!(
            byte,
            b' ' | b'\t' | b'\n' | b'"' | b'\'' | b'>' | b'<' | b'/' | b'='
        )
}

fn is_attribute_name_byte_astro(byte: u8) -> bool {
    is_attribute_name_byte(byte)
}

fn is_attribute_name_byte_vue(byte: u8) -> bool {
    is_attribute_name_byte(byte) && byte != b':' && byte != b'.' && byte != b']' && byte != b'['
}

fn is_astro_directive_keyword_bytes(bytes: &[u8]) -> bool {
    matches!(
        bytes,
        b"client" | b"set" | b"class" | b"is" | b"server" | b"define"
    )
}

fn is_vue_directive_prefix_bytes(bytes: &[u8]) -> bool {
    bytes.starts_with(b"v-")
}

/// Check if a char is a linebreak (for JS-style comments in Svelte)
fn is_linebreak(chr: char) -> bool {
    matches!(chr, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// Identifiers can contain letters, numbers and `_`
fn is_at_continue_identifier(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Identifiers should start with letters or `_`
fn is_at_start_identifier(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
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

impl<'src> LexerWithCheckpoint<'src> for HtmlLexer<'src> {
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

/// Tracks whether the lexer is currently inside an open string literal while
/// scanning Astro frontmatter. Used to determine whether a `---` sequence is
/// a genuine closing fence or merely three dashes that appear inside a string.
///
/// ## Design
///
/// The tracker maintains:
/// - The **currently open quote character** (`current_quote`): `None` when not
///   inside any string, or `Some(b'"')` / `Some(b'\'')` / `Some(b'`')` when
///   inside a string. A quote opens a string only when no other string is
///   already open; it closes the string only when it **matches** the opening
///   quote. For example, a `'` inside a `"…"` string is treated as a literal
///   character, not as a new string opener.
/// - The **comment state** (`comment`): distinguishes single-line (`//`) from
///   multi-line (`/* … */`) comments, so that quote characters inside comments
///   are not counted as string delimiters.
/// - The **escape flag** (`escaped`): set when the previous byte was an
///   unescaped `\`, so that the immediately following quote character is not
///   treated as a string delimiter. A double backslash resets the flag because
///   the backslash itself is escaped.
struct QuotesSeen {
    /// The quote character that opened the current string, if any.
    current_quote: Option<u8>,
    /// Current comment state.
    comment: QuotesSeenComment,
    /// Whether the previous byte was an unescaped backslash.
    escaped: bool,
    /// The previous byte, needed to detect `//` and `/* */` comment markers
    /// and the `*/` block-comment terminator.
    prev_byte: Option<u8>,
}

/// Distinguishes the kind of comment the lexer is currently inside.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuotesSeenComment {
    /// Not inside any comment.
    None,
    /// Inside a `//` single-line comment; exits on `\n`.
    SingleLine,
    /// Inside a `/* … */` multi-line comment; exits on `*/`.
    MultiLine,
}

impl QuotesSeen {
    fn new() -> Self {
        Self {
            current_quote: None,
            comment: QuotesSeenComment::None,
            escaped: false,
            prev_byte: None,
        }
    }

    /// Processes one byte of frontmatter source and updates the tracking state.
    fn check_byte(&mut self, byte: u8) {
        match self.comment {
            QuotesSeenComment::SingleLine => {
                // Single-line comment ends at the newline.
                if byte == b'\n' {
                    self.comment = QuotesSeenComment::None;
                }
                self.prev_byte = Some(byte);
                // Quotes inside comments are ignored.
                return;
            }
            QuotesSeenComment::MultiLine => {
                // Multi-line comment ends at `*/`.
                if self.prev_byte == Some(b'*') && byte == b'/' {
                    self.comment = QuotesSeenComment::None;
                }
                self.prev_byte = Some(byte);
                // Quotes inside comments are ignored.
                return;
            }
            QuotesSeenComment::None => {}
        }

        // Handle escape sequences: a `\` that is not itself escaped toggles the
        // escape flag for the next character.
        if byte == b'\\' {
            self.escaped = !self.escaped;
            self.prev_byte = Some(byte);
            return;
        }

        // If the current byte is escaped, it cannot act as a string delimiter
        // or comment opener.
        let was_escaped = self.escaped;
        self.escaped = false;

        if was_escaped {
            self.prev_byte = Some(byte);
            return;
        }

        // Detect comment openers — only valid outside of open strings.
        if self.current_quote.is_none() && self.prev_byte == Some(b'/') {
            match byte {
                b'/' => {
                    self.comment = QuotesSeenComment::SingleLine;
                    self.prev_byte = Some(byte);
                    return;
                }
                b'*' => {
                    self.comment = QuotesSeenComment::MultiLine;
                    self.prev_byte = Some(byte);
                    return;
                }
                _ => {}
            }
        }

        // Track string delimiters.
        match byte {
            b'"' | b'\'' | b'`' => {
                match self.current_quote {
                    None => {
                        // No string is open; this quote opens one.
                        self.current_quote = Some(byte);
                    }
                    Some(open) if open == byte => {
                        // The same quote that opened this string closes it.
                        self.current_quote = None;
                    }
                    Some(_) => {
                        // A different quote character inside an open string is
                        // just a literal character — ignore it.
                    }
                }
            }
            _ => {}
        }

        self.prev_byte = Some(byte);
    }

    /// Returns `true` when the tracker is not currently inside an open string literal
    /// or a comment. Both states must be absent for a `---` fence to be a valid
    /// frontmatter closing delimiter.
    fn is_empty(&self) -> bool {
        self.current_quote.is_none() && self.comment == QuotesSeenComment::None
    }
}

#[cfg(test)]
mod quotes_seen {
    use crate::lexer::QuotesSeen;

    fn track(source: &str, quotes_seen: &mut QuotesSeen) {
        for char in source.as_bytes() {
            quotes_seen.check_byte(*char);
        }
    }

    #[test]
    fn is_not_empty() {
        let source = r#"'"`"#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(!quotes_seen.is_empty());

        let source = r#"`'""'"#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(!quotes_seen.is_empty());
    }
    #[test]
    fn is_empty() {
        let source = r#" '"``"' "#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
    }

    /// A single-line comment that has not yet been terminated (no trailing newline)
    /// leaves the tracker inside the comment, so `is_empty()` returns `false`.
    /// A `---` encountered in this state must not be treated as a closing fence.
    #[test]
    fn not_empty_inside_unterminated_single_line_comment() {
        let source = r#"// Don't want to use any of this? Delete everything in this file, the `assets`, `components`, and `layouts` directories, and start fresh."#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(!quotes_seen.is_empty());
    }

    #[test]
    fn empty_with_comments_outside_comments_1() {
        let source = r#"// Don't want to use any of this? Delete everything in this file, the `assets`, `components`, and `layouts` directories, and start fresh.
const f = "something" "#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
    }

    #[test]
    fn empty_with_comments_outside_comments_2() {
        let source = r#"/* Don't want to use any of this? Delete everything in this file, the `assets`, `components`, and `layouts` directories, and start fresh. */
const f = "something" "#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
    }

    #[test]
    fn empty_with_comments_outside_comments_3() {
        let source = r#"/* "safe"
 'safe'
 `safe`*/
const f = "something" "#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
    }

    // --- Tests for issue #9108: mixed quote types inside a string ---

    /// A double-quoted string containing a single quote should be considered closed.
    /// The apostrophe is a literal character inside the double-quoted string and
    /// must not be treated as an opening single-quote delimiter.
    #[test]
    fn issue_9108_double_quoted_with_apostrophe() {
        // "test'"  — double-quoted string that contains an apostrophe
        let source = r#""test'""#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "double-quoted string containing apostrophe must be treated as closed"
        );
    }

    /// A single-quoted string containing a double quote should be considered closed.
    #[test]
    fn issue_9108_single_quoted_with_double_quote() {
        // 'it"s'
        let source = r#"'it"s'"#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "single-quoted string containing double quote must be treated as closed"
        );
    }

    /// A template literal containing both single and double quotes should close cleanly.
    #[test]
    fn issue_9108_template_with_mixed_quotes() {
        // `it's a "test"`
        let source = "`it's a \"test\"`";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "template literal containing single and double quotes must be treated as closed"
        );
    }

    /// Multiple complete strings with different quote types on the same line.
    #[test]
    fn issue_9108_multiple_strings_with_mixed_quotes() {
        // const a = "it's"; const b = 'say "hi"';
        let source = r#"const a = "it's"; const b = 'say "hi"';"#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "multiple complete string literals with mixed quote types must all be closed"
        );
    }

    // --- Tests for issue #8882: multi-line block comments with quotes ---

    /// A multi-line block comment containing an apostrophe must not leave the tracker
    /// in a non-empty state. Quote characters inside block comments are not string
    /// delimiters and must be ignored across all lines of the comment.
    #[test]
    fn issue_8882_multiline_block_comment_with_apostrophe() {
        let source = "/*\n * Doesn't that stink?\n */";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "multi-line block comment with apostrophe must not leave an open quote"
        );
    }

    /// JSDoc-style comment followed by real code — mirrors the exact pattern from #8882.
    #[test]
    fn issue_8882_jsdoc_comment_then_code() {
        let source = "/**\n * In this comment, if you add any string opening or closing, such as an apostrophe, the file will show \n * a bunch of errors. Doesn't (remove the apostrophe in the previous word to fix) that stink? \n */\nimport type { HTMLAttributes } from \"astro/types\";\nconst { class: className } = Astro.props;";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "JSDoc comment with apostrophes followed by real code must leave tracker empty"
        );
    }

    /// Multi-line block comment with quotes on every line.
    #[test]
    fn issue_8882_multiline_block_comment_quotes_every_line() {
        let source = "/* line one 'unclosed\n   line two `unclosed\n   line three \"unclosed */\nconst x = 1;";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "block comment with unclosed quote chars on every line must not affect tracker"
        );
    }

    // --- Tests for fence-inside-comment cases ---

    /// A `---` sequence that appears after `//` on the same line must not close
    /// the frontmatter, because it is inside a single-line comment.
    #[test]
    fn fence_inside_single_line_comment_is_not_empty() {
        // "// ---" — the dashes are inside a line comment, not a real fence
        let source = "// ---";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            !quotes_seen.is_empty(),
            "tracker must be non-empty while inside a single-line comment"
        );
    }

    /// A `---` sequence that appears inside a `/* */` block comment must not
    /// close the frontmatter.
    #[test]
    fn fence_inside_block_comment_is_not_empty() {
        // "/*\n---\n" — the dashes are inside a block comment that has not yet closed
        let source = "/*\n---\n";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            !quotes_seen.is_empty(),
            "tracker must be non-empty while inside a multi-line block comment"
        );
    }

    // --- Tests for escape sequence handling ---

    /// An escaped quote inside a double-quoted string must not close the string early.
    /// In `"\""`, the inner `\"` is escaped, so only the final `"` closes the string.
    #[test]
    fn escaped_double_quote_inside_double_string() {
        // "\""  — the backslash escapes the inner double quote
        let source = "\"\\\"\"";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "escaped double quote inside double-quoted string must not close it prematurely"
        );
    }

    /// An escaped single quote inside a single-quoted string must not close it early.
    /// In `'\''`, the inner `\'` is escaped, so only the final `'` closes the string.
    #[test]
    fn escaped_single_quote_inside_single_string() {
        // '\''  — open single quote, escaped single quote, closing single quote
        let source = r"'\''";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "escaped single quote inside single-quoted string must not close it prematurely"
        );
    }

    /// An escaped backtick inside a template literal must not close it early.
    #[test]
    fn escaped_backtick_inside_template() {
        // `\``
        let source = "`\\``";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "escaped backtick inside template literal must not close it prematurely"
        );
    }

    /// A double backslash before a closing quote means the backslash is itself escaped,
    /// so the quote is a genuine string delimiter. `"\\"` opens with the first `"`,
    /// contains an escaped backslash `\\`, and closes with the final `"`.
    #[test]
    fn double_backslash_then_quote() {
        // "\\"  — opening quote, escaped backslash, closing quote
        let source = "\"\\\\\"";
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(
            quotes_seen.is_empty(),
            "double backslash followed by closing quote must close the string"
        );
    }
}
