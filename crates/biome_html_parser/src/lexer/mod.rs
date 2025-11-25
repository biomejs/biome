mod tests;

use crate::token_source::{
    HtmlEmbeddedLanguage, HtmlLexContext, HtmlReLexContext, TextExpressionKind,
};
use biome_html_syntax::HtmlSyntaxKind::{
    ATTACH_KW, COMMENT, CONST_KW, DEBUG_KW, DOCTYPE_KW, ELSE_KW, EOF, ERROR_TOKEN, HTML_KW,
    HTML_LITERAL, HTML_STRING_LITERAL, IF_KW, KEY_KW, NEWLINE, RENDER_KW, SVELTE_IDENT, TOMBSTONE,
    UNICODE_BOM, WHITESPACE,
};
use biome_html_syntax::{HtmlSyntaxKind, T, TextLen, TextSize};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags};
use biome_rowan::SyntaxKind;
use biome_unicode_table::Dispatch::{BSL, QOT, UNI};
use biome_unicode_table::lookup_byte;
use std::ops::{Add, AddAssign};

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
}

enum IdentifierContext {
    None,
    Doctype,
    Svelte,
    Vue,
    VueDirectiveArgument,
}

impl IdentifierContext {
    const fn is_doctype(&self) -> bool {
        matches!(self, Self::Doctype)
    }

    const fn is_svelte(&self) -> bool {
        matches!(self, Self::Svelte)
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
        }
    }

    /// Consume a token in the [HtmlLexContext::InsideTag] context.
    fn consume_token_inside_tag(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_l_angle(),
            b'>' => self.consume_byte(T![>]),
            b'/' => self.consume_byte(T![/]),
            b'=' => self.consume_byte(T![=]),
            b'!' => self.consume_byte(T![!]),
            b'{' if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            b'{' => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            b'}' => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            b'\'' | b'"' => self.consume_string_literal(current),
            _ if self.current_kind == T![<] && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                self.consume_tag_name(current)
            }
            _ if self.current_kind != T![<] && is_attribute_name_byte(current) => {
                self.consume_identifier(current, IdentifierContext::None)
            }
            _ if is_at_svelte_start_identifier(current) => {
                self.consume_identifier(current, IdentifierContext::Svelte)
            }
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

    /// Consume a token in the [HtmlLexContext::InsideTagVue] context.
    fn consume_token_inside_tag_vue(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_l_angle(),
            b'>' => self.consume_byte(T![>]),
            b'/' => self.consume_byte(T![/]),
            b'=' => self.consume_byte(T![=]),
            b'!' => self.consume_byte(T![!]),
            b'{' => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            b'}' => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }

            // these are used in Vue directives
            b':' => self.consume_byte(T![:]),
            b'@' => self.consume_byte(T![@]),
            b'.' => self.consume_byte(T![.]),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'#' => self.consume_byte(T![#]),

            b'\'' | b'"' => self.consume_string_literal(current),
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

    /// Consume a token in the [HtmlLexContext::Regular] context.
    fn consume_token(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'!' if self.current() == T![<] => self.consume_byte(T![!]),
            b'/' if self.current() == T![<] => self.consume_byte(T![/]),
            b',' if self.current() == T![<] => self.consume_byte(T![,]),
            b'-' if self.at_frontmatter_edge() => self.consume_frontmatter_edge(),
            b'{' if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            b'{' => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            b'}' => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            b'<' => {
                // if this truly is the start of a tag, it *must* be immediately followed by a tag name. Whitespace is not allowed.
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                if self
                    .peek_byte()
                    .is_some_and(|b| is_tag_name_byte(b) || b == b'!' || b == b'/' || b == b'>')
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
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_byte(T![<]),
            b'>' => self.consume_byte(T![>]),
            b'{' => self.consume_byte(T!['{']),
            b'}' => self.consume_byte(T!['}']),
            b'\'' | b'"' => self.consume_string_literal(current),
            _ => self.consume_unquoted_string_literal(),
        }
    }

    /// Consume a token in the [HtmlLexContext::Doctype] context.
    fn consume_token_doctype(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' => self.consume_byte(T![<]),
            b'>' => self.consume_byte(T![>]),
            b'!' => self.consume_byte(T![!]),
            b'\'' | b'"' => self.consume_string_literal(current),
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
        match current {
            b'}' if self.at_closing_double_text_expression() => {
                self.consume_r_double_text_expression()
            }
            b'<' => self.consume_byte(T![<]),
            _ => {
                while let Some(current) = self.current_byte() {
                    match current {
                        b'}' if self.at_closing_double_text_expression() => break,
                        _ => {
                            self.advance(1);
                        }
                    }
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

    /// Consume a token in the [HtmlLexContext::CdataSection] context.
    fn consume_inside_cdata(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'<' if self.at_start_cdata() => self.consume_cdata_start(),
            b']' if self.at_end_cdata() => self.consume_cdata_end(),
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
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'<' if self.at_start_cdata() => self.consume_cdata_start(),
            b'<' => self.consume_byte(T![<]),
            b'-' => {
                debug_assert!(self.at_frontmatter_edge());
                self.advance(3);
                T![---]
            }
            _ => {
                self.consume_token_embedded_language(current, HtmlEmbeddedLanguage::Script, context)
            }
        }
    }

    fn consume_svelte(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'\n' | b'\r' | b'\t' | b' ' => self.consume_newline_or_whitespaces(),
            b'}' => self.consume_byte(T!['}']),
            b',' => self.consume_byte(T![,]),
            b'{' if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            b'{' => self.consume_byte(T!['{']),
            _ if is_at_svelte_start_identifier(current) => {
                self.consume_identifier(current, IdentifierContext::Svelte)
            }
            _ => self.consume_single_text_expression(),
        }
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
                IdentifierContext::Svelte => {
                    if is_at_svelte_continue_identifier(byte) {
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
                    if is_attribute_name_byte_vue(byte) {
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
            b"doctype" | b"DOCTYPE" if !context.is_svelte() => DOCTYPE_KW,
            b"html" | b"HTML" if context.is_doctype() => HTML_KW,
            buffer if context.is_svelte() => match buffer {
                b"debug" => DEBUG_KW,
                b"attach" => ATTACH_KW,
                b"const" => CONST_KW,
                b"render" => RENDER_KW,
                b"html" => HTML_KW,
                b"key" => KEY_KW,
                b"if" => IF_KW,
                b"else" => ELSE_KW,
                _ => SVELTE_IDENT,
            },
            _ => HTML_LITERAL,
        }
    }

    /// Consumes an HTML tag name token starting with the given byte.
    /// Tag names can contain alphanumeric characters, hyphens, colons and dots.
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

        match current {
            b'{' => {
                if self.at_opening_double_text_expression() {
                    self.consume_l_double_text_expression()
                } else {
                    self.consume_byte(T!['{'])
                }
            }
            b'}' => {
                if self.at_closing_double_text_expression() {
                    self.consume_r_double_text_expression()
                } else {
                    self.consume_byte(T!['}'])
                }
            }
            _ => {
                while let Some(current) = self.current_byte() {
                    match current {
                        b'{' => {
                            if was_escaped {
                                self.advance(1);
                            } else {
                                break;
                            }
                        }
                        b'\\' => {
                            was_escaped = true;
                            whitespace_started = None;
                            self.advance(1);
                        }
                        b'}' => {
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

                        b'<' => {
                            break;
                        }
                        b'\n' | b'\r' => {
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
                        b' ' | b'\t' => {
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
                    HtmlLexContext::InsideTagVue => self.consume_token_inside_tag_vue(current),
                    HtmlLexContext::AttributeValue => self.consume_token_attribute_value(current),
                    HtmlLexContext::Doctype => self.consume_token_doctype(current),
                    HtmlLexContext::EmbeddedLanguage(lang) => {
                        self.consume_token_embedded_language(current, lang, context)
                    }
                    HtmlLexContext::TextExpression(kind) => match kind {
                        TextExpressionKind::Double => self.consume_double_text_expression(current),
                        TextExpressionKind::Single => self.consume_single_text_expression(),
                    },
                    HtmlLexContext::CdataSection => self.consume_inside_cdata(current),
                    HtmlLexContext::AstroFencedCodeBlock => {
                        self.consume_astro_frontmatter(current, context)
                    }
                    HtmlLexContext::Svelte => self.consume_svelte(current),
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
                HtmlReLexContext::SingleTextExpression => self.consume_single_text_expression(),
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

fn is_tag_name_byte(byte: u8) -> bool {
    // Canonical HTML tag names are specified to be case-insensitive and alphanumeric.
    // https://html.spec.whatwg.org/#elements-2
    // https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
    // However, custom tag names must start with a lowercase letter, but they can be followed by pretty much anything else.
    // https://html.spec.whatwg.org/#valid-custom-element-name

    // The extra characters allowed here `-`, `:`, and `.` are not usually allowed in the HTML tag name.
    // However, Prettier considers them to be valid characters in tag names, so we allow them to remain compatible.

    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b':' || byte == b'.'
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

fn is_attribute_name_byte_vue(byte: u8) -> bool {
    is_attribute_name_byte(byte) && byte != b':' && byte != b'.' && byte != b']' && byte != b'['
}

/// Identifiers can contain letters, numbers and `_`
fn is_at_svelte_continue_identifier(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Identifiers should start with letters or `_`
fn is_at_svelte_start_identifier(byte: u8) -> bool {
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
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

struct QuotesSeen {
    single: u16,
    double: u16,
    template: u16,
    inside_comment: bool,
    prev_byte: Option<u8>,
}

impl QuotesSeen {
    fn new() -> Self {
        Self {
            single: 0,
            double: 0,
            template: 0,
            inside_comment: false,
            prev_byte: None,
        }
    }

    /// It checks the given byte. If it's a quote, it's tracked
    fn check_byte(&mut self, byte: u8) {
        // Check for comment exit first
        if self.inside_comment {
            if byte == b'\n' {
                // Exit single-line comment
                self.inside_comment = false;
            } else if self.prev_byte == Some(b'*') && byte == b'/' {
                // Exit multi-line comment
                self.inside_comment = false;
            }
            self.prev_byte = Some(byte);
            return; // Don't track quotes inside comments
        }

        // Check for comment entry - but only if we're not inside quotes
        if self.prev_byte == Some(b'/')
            && (byte == b'/' || byte == b'*')
            && self.single == 0
            && self.double == 0
            && self.template == 0
        {
            self.inside_comment = true;
            self.prev_byte = Some(byte);
            return;
        }

        // Normal quote tracking
        match byte {
            b'"' => self.track_double(),
            b'\'' => self.track_single(),
            b'`' => self.track_template(),
            _ => {}
        }

        self.prev_byte = Some(byte);
    }

    /// It adds a single quote if single quotes are zero and the others are greater than zero. It removes it otherwise
    fn track_single(&mut self) {
        if (self.single == 0 && (self.double > 0 || self.template > 0))
            || (self.single == 0 && self.double == 0 && self.template == 0)
        {
            self.single.add_assign(1);
        } else {
            self.single = self.single.saturating_sub(1);
        }
    }
    /// It adds a double quote if double quotes are zero and the others are greater than zero. It removes it otherwise
    fn track_double(&mut self) {
        if (self.double == 0 && (self.single > 0 || self.template > 0))
            || (self.double == 0 && self.single == 0 && self.template == 0)
        {
            self.double.add_assign(1);
        } else {
            self.double = self.double.saturating_sub(1);
        }
    }

    /// It adds a template quote if template quotes are zero and the others are greater than zero. It removes it otherwise
    fn track_template(&mut self) {
        if (self.template == 0 && (self.single > 0 || self.double > 0))
            || (self.template == 0 && self.single == 0 && self.double == 0)
        {
            self.template.add_assign(1);
        } else {
            self.template = self.template.saturating_sub(1);
        }
    }

    fn is_empty(&self) -> bool {
        self.single == 0 && self.double == 0 && self.template == 0
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

        let source = r#"// Don't want to use any of this? Delete everything in this file, the `assets`, `components`, and `layouts` directories, and start fresh."#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
    }

    #[test]
    fn empty_inside_comments() {
        let source = r#"// Don't want to use any of this? Delete everything in this file, the `assets`, `components`, and `layouts` directories, and start fresh."#;
        let mut quotes_seen = QuotesSeen::new();
        track(source, &mut quotes_seen);
        assert!(quotes_seen.is_empty());
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
}
