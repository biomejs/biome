mod tests;

use crate::token_source::{
    HtmlEmbeddedLanguage, HtmlFramework, HtmlLexContext, HtmlReLexContext,
    RestrictedExpressionStopAt, TextExpressionKind,
};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HTML_TAG_NAMES, HtmlSyntaxKind, T, TextLen, TextSize, VOID_ELEMENTS};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags};
use biome_rowan::SyntaxKind;
use biome_unicode_table::{Dispatch::*, lookup_byte};
use smallvec::SmallVec;

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
    /// Parse options, not state. The lexer cannot read them from the lex
    /// context, because `bump()` passes [HtmlLexContext::default].
    options: HtmlLexerOptions,
}

/// The parse options the lexer needs. Fixed for the whole file, unlike the
/// lexer's position and flags.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct HtmlLexerOptions {
    pub(crate) framework: HtmlFramework,
}

enum IdentifierContext {
    None,
    Doctype,
    Vue,
    VueDirectiveArgument,
    Astro,
    VueVForValue,
    Angular,
}

/// Controls how [`HtmlLexer::consume_tag_name`] classifies a tag-name token.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TagNameMode {
    /// Plain HTML: tag-name keyword lookup, no component names.
    Html,
    /// Framework file (Vue/Svelte/Astro): tag-name keyword lookup, but a PascalCase
    /// name (or a name followed by `.`) is a component.
    HtmlOrComponent,
    /// Component/member-name context: always emit `HTML_COMPONENT_LITERAL`.
    ComponentOnly,
}

impl TagNameMode {
    fn for_inside_tag(framework: HtmlFramework) -> Self {
        if framework.supports_components() {
            Self::HtmlOrComponent
        } else {
            Self::Html
        }
    }

    const fn allows_components(self) -> bool {
        matches!(self, Self::HtmlOrComponent | Self::ComponentOnly)
    }
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
            options: HtmlLexerOptions::default(),
        }
    }

    pub(crate) fn with_options(self, options: HtmlLexerOptions) -> Self {
        Self { options, ..self }
    }

    /// Sets the `after_frontmatter` flag. When `true`, `---` in the `Regular`
    /// context is treated as plain HTML text rather than a `FENCE` token.
    pub fn set_after_frontmatter(&mut self, value: bool) {
        self.after_frontmatter = value;
    }

    /// Consume a token in the [HtmlLexContext::InsideTag] context.
    fn consume_token_inside_tag(
        &mut self,
        current: u8,
        mode: TagNameMode,
        double_text_expressions: bool,
    ) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QST if self.at_pi_end() => self.consume_pi_end(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => self.consume_byte(T![/]),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            BEO if self.at_svelte_opening_block() => self.consume_svelte_opening_block(),
            BEO => {
                if double_text_expressions && self.at_opening_double_text_expression() {
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
            // A tag name immediately follows the `<` of an opening tag or the `/`
            // of a closing tag (`</div>`), so classify it in both cases.
            _ if matches!(self.current_kind, T![<] | T![/]) && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                self.consume_tag_name(current, mode)
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
    fn consume_token_inside_tag_astro(&mut self, current: u8, mode: TagNameMode) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => match self.consume_js_comment_in_tag() {
                Some(comment) => comment,
                None => self.consume_byte(T![/]),
            },
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            // Handle colons as separate tokens for Astro directives
            COL => self.consume_byte(T![:]),
            PRD => self.consume_byte(T![.]),
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
            // A tag name immediately follows the `<` of an opening tag or the `/`
            // of a closing tag (`</div>`), so classify it in both cases.
            _ if matches!(self.current_kind, T![<] | T![/]) && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                self.consume_tag_name(current, mode)
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

    /// Consume a token in the [HtmlLexContext::InsideTagAngular] context.
    /// This context is used for Angular templates with Angular-specific attribute syntax.
    fn consume_token_inside_tag_angular(
        &mut self,
        current: u8,
        mode: TagNameMode,
    ) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => self.consume_byte(T![/]),
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            BTO if self.at_angular_two_way_binding_start() => {
                self.advance(2);
                T!["[("]
            }
            PNC if self.at_angular_two_way_binding_end() => {
                self.advance(2);
                T![")]"]
            }
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            HAS => self.consume_byte(T![#]),
            MUL => self.consume_byte(T![*]),
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
                self.consume_tag_name(current, mode)
            }
            _ if self.current_kind != T![<] && is_attribute_name_byte(current) => {
                self.consume_identifier(current, IdentifierContext::Angular)
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
    ///
    /// `svelte` controls brace handling only. `//` and `/* */` comments follow
    /// [`HtmlLexerOptions::framework`], since Astro accepts them too.
    fn consume_token_inside_tag_directives(
        &mut self,
        current: u8,
        svelte: bool,
        mode: TagNameMode,
    ) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS => self.consume_l_angle(),
            MOR => self.consume_byte(T![>]),
            SLH => match self.consume_js_comment_in_tag() {
                Some(comment) => comment,
                None => self.consume_byte(T![/]),
            },
            EQL => self.consume_byte(T![=]),
            EXL => self.consume_byte(T![!]),
            BEO => {
                if !svelte && self.at_opening_double_text_expression() {
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
            // A tag name immediately follows the `<` of an opening tag or the `/`
            // of a closing tag (`</div>`), so classify it in both cases.
            _ if matches!(self.current_kind, T![<] | T![/]) && is_tag_name_byte(current) => {
                // tag names must immediately follow a `<`
                // https://html.spec.whatwg.org/multipage/syntax.html#start-tags
                self.consume_tag_name(current, mode)
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
        let length = JsScanner::argument_length(&self.source.as_bytes()[self.position..]);
        self.advance(length);

        if length > 0 {
            HTML_LITERAL
        } else {
            ERROR_TOKEN
        }
    }

    /// Consume a token in the [HtmlLexContext::InsideTagSvelte] context.
    /// This context is used for Svelte files with JS-style comment support.
    fn consume_token_inside_tag_svelte(
        &mut self,
        current: u8,
        mode: TagNameMode,
    ) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            SLH if let Some(comment) = self.consume_js_comment_in_tag() => return comment,
            PRD => return self.consume_byte(T![.]),
            _ => {}
        }
        self.consume_token_inside_tag(current, mode, false)
    }

    fn consume_token_vue_v_for_value(&mut self, current: u8) -> HtmlSyntaxKind {
        match lookup_byte(current) {
            WHS => self.consume_newline_or_whitespaces(),
            QOT if current == b'\'' => self.consume_byte(T!["'"]),
            QOT => self.consume_byte(T!['"']),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            COM => self.consume_byte(T![,]),
            COL => self.consume_byte(T![:]),
            BEO => self.consume_byte(T!['{']),
            BEC => self.consume_byte(T!['}']),
            BTO => self.consume_byte(T!['[']),
            BTC => self.consume_byte(T![']']),
            PRD if self.is_at_three_dots() => self.consume_dot3(),
            IDT if self.at_vue_v_for_in_keyword() => self.consume_vue_v_for_in_keyword(),
            IDT if self.at_vue_v_for_of_keyword() => self.consume_vue_v_for_of_keyword(),
            _ if is_at_start_identifier(current) => {
                self.consume_identifier(current, IdentifierContext::VueVForValue)
            }
            // Handle numbers in v-for expressions (like "n in 10")
            ZER | DIG => self.consume_vue_v_for_number(),
            _ => self.consume_vue_v_for_literal(),
        }
    }

    fn consume_token_vue_v_for_expression(
        &mut self,
        current: u8,
        quote: HtmlSyntaxKind,
    ) -> HtmlSyntaxKind {
        if (quote == T!['"'] && current == b'"') || (quote == T!["'"] && current == b'\'') {
            return self.consume_byte(quote);
        }

        while let Some(byte) = self.current_byte() {
            if (quote == T!['"'] && byte == b'"') || (quote == T!["'"] && byte == b'\'') {
                break;
            }

            self.advance_byte_or_char(byte);
        }

        HTML_LITERAL
    }

    /// Consume a token in the [HtmlLexContext::Regular] context.
    fn consume_token(&mut self, current: u8, double_text_expressions: bool) -> HtmlSyntaxKind {
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
                if double_text_expressions && self.at_opening_double_text_expression() {
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
                if self.peek_byte().is_some_and(|b| {
                    is_tag_start_byte(b) || b == b'!' || b == b'/' || b == b'>' || b == b'?'
                }) {
                    self.consume_l_angle()
                } else {
                    // Astro keeps the HTML5 reading, where a `<` that cannot open
                    // a tag is text and so needs no escaping.
                    if self.options.framework != HtmlFramework::Astro {
                        self.push_diagnostic(
                            ParseDiagnostic::new(
                                "Unescaped `<` bracket character. Expected a tag or escaped character.",
                                self.text_position()..self.text_position() + TextSize::from(1),
                            )
                            .with_hint("Replace this character with `&lt;` to escape it."),
                        );
                    }
                    self.consume_byte(HTML_LITERAL)
                }
            }
            IDT => self
                .consume_language_identifier(current)
                .unwrap_or_else(|| self.consume_html_text(current, double_text_expressions)),
            _ => {
                if self.position == 0
                    && let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM)
                {
                    self.unicode_bom_length = bom_size;
                    return bom;
                }
                self.consume_html_text(current, double_text_expressions)
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
            TPL if self.options.framework == HtmlFramework::Astro => {
                self.consume_template_literal_attribute_value()
            }
            _ => self.consume_unquoted_string_literal(),
        }
    }

    fn consume_token_svelte_attribute_value(&mut self, current: u8) -> HtmlSyntaxKind {
        match current {
            b'"' => self.consume_byte(T!['"']),
            b'\'' => self.consume_byte(T!["'"]),
            _ => self.consume_token_attribute_value(current),
        }
    }

    /// Consume a token in the [HtmlLexContext::SvelteTemplateChunk] context.
    fn consume_token_svelte_template_chunk(&mut self, current: u8, quote: u8) -> HtmlSyntaxKind {
        if current == b'{' {
            self.consume_byte(T!['{'])
        } else if current == quote {
            self.advance(1);
            if quote == b'"' { T!['"'] } else { T!["'"] }
        } else {
            self.consume_svelte_template_chunk(quote)
        }
    }

    fn consume_svelte_template_chunk(&mut self, quote: u8) -> HtmlSyntaxKind {
        while let Some(current) = self.current_byte() {
            if current == b'{' || current == quote {
                break;
            }
            match lookup_byte(current) {
                UNI => self.advance_char_unchecked(),
                _ => self.advance(1),
            }
        }

        HTML_TEMPLATE_CHUNK
    }

    /// Consume a `` ` ``-delimited attribute value, which only Astro allows.
    ///
    /// The token kind matches a quoted string so the existing initializer path
    /// is reused; consumers that must tell the two apart check the leading byte.
    fn consume_template_literal_attribute_value(&mut self) -> HtmlSyntaxKind {
        let start = self.text_position();
        self.advance(1);
        while let Some(byte) = self.current_byte() {
            match byte {
                b'`' => {
                    self.advance(1);
                    return HTML_STRING_LITERAL;
                }
                b'\\' => {
                    self.advance(1);
                    if let Some(next) = self.current_byte() {
                        self.advance_byte_or_char(next);
                    }
                }
                _ => self.advance_byte_or_char(byte),
            }
        }
        self.diagnostics.push(
            ParseDiagnostic::new("Missing closing backtick", start..self.text_position())
                .with_detail(
                    self.source.text_len()..self.source.text_len(),
                    "file ends here",
                ),
        );
        HTML_STRING_LITERAL
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

    /// Whether the lexer sits at the closing tag `</name>`, ignoring case the
    /// way tag names are matched everywhere else.
    ///
    /// Whitespace is allowed before the `>`, as browsers accept it. The name
    /// has to end where the tag does, so that `</pre>` does not stop a
    /// `</prefetch>` from being ordinary text.
    fn is_at_closing_tag(&self, name: &str) -> bool {
        let Some(after_name) = self
            .source
            .get(self.position..)
            .and_then(|rest| rest.strip_prefix("</"))
            .and_then(|rest| {
                let candidate = rest.get(..name.len())?;
                candidate
                    .eq_ignore_ascii_case(name)
                    .then(|| &rest[name.len()..])
            })
        else {
            return false;
        };

        after_name
            .trim_start_matches([' ', '\t', '\n', '\r', '\x0C'])
            .starts_with('>')
    }

    /// Consume an embedded language in its entirety. Stops immediately before
    /// the closing tag.
    fn consume_token_embedded_language(
        &mut self,
        _current: u8,
        lang: HtmlEmbeddedLanguage,
    ) -> HtmlSyntaxKind {
        let start = self.text_position();
        let closing_tag_name = lang.closing_tag_name(self.source);
        self.assert_current_char_boundary();
        while let Some(byte) = self.current_byte() {
            if self.is_at_closing_tag(closing_tag_name) {
                break;
            }
            self.advance_byte_or_char(byte);
        }

        if self.text_position() != start {
            HTML_LITERAL
        } else {
            // if the element is empty, we will immediately hit the closing tag.
            // we HAVE to consume something, so we start consuming the closing tag.
            self.consume_byte(T![<])
        }
    }

    /// Consume the Astro frontmatter body up to its closing `---` fence. A
    /// closing tag does not end it: `</script>` inside frontmatter is ordinary
    /// JavaScript, as it is in Astro.
    fn consume_astro_frontmatter_body(&mut self) -> HtmlSyntaxKind {
        self.assert_current_char_boundary();
        let length = JsScanner::frontmatter_length(&self.source.as_bytes()[self.position..]);
        // The lexer must make progress even if a fence somehow sits at offset zero.
        self.advance(length.max(1));

        HTML_LITERAL
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
        // emit leading whitespace as trivia so it's not part of the expression literal
        if let Some(current) = self.current_byte()
            && lookup_byte(current) == WHS
        {
            return self.consume_newline_or_whitespaces();
        }

        let length = JsScanner::expression_length(
            &self.source.as_bytes()[self.position..],
            self.options.framework == HtmlFramework::Astro,
        );
        self.advance(length);

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
        // For `AsOrCommaSkipFirstAs`: tracks whether the first stop keyword has
        // already been skipped (i.e., the TypeScript `as` in `as const`).
        let mut first_stop_keyword_seen = false;

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
                _ if is_closing_paren(current) => {
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
                        let should_stop =
                            kind.matches_keyword(keyword_kind) && prev_byte == Some(b' ');

                        if should_stop {
                            if kind == RestrictedExpressionStopAt::AsOrCommaSkipFirstAs
                                && !first_stop_keyword_seen
                            {
                                // First `as` belongs to a TypeScript `as const` assertion;
                                // the parser determined this via lookahead. Skip it and
                                // continue scanning for the Svelte binding `as`.
                                first_stop_keyword_seen = true;
                            } else {
                                // Rewind — don't consume the keyword
                                self.position = checkpoint_pos;
                                break;
                            }
                        }
                        // Not a stop keyword (or skipped), continue
                        // (position already advanced by consume_language_identifier)
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

    /// Consumes a comment between attributes, which only Svelte and Astro
    /// accept; elsewhere a `/` inside a tag can only open a self-closing tag.
    fn consume_js_comment_in_tag(&mut self) -> Option<HtmlSyntaxKind> {
        if !matches!(
            self.options.framework,
            HtmlFramework::Svelte | HtmlFramework::Astro
        ) {
            return None;
        }

        match self.byte_at(1).map(lookup_byte) {
            Some(SLH) => Some(self.consume_js_line_comment()),
            Some(MUL) => Some(self.consume_js_block_comment()),
            _ => None,
        }
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
    /// the closing `---` fence is reached.
    fn consume_astro_frontmatter(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            LSS if self.at_start_cdata() => self.consume_cdata_start(),
            // Frontmatter never starts with markup, so this is a missing fence.
            LSS => self.consume_byte(T![<]),
            MIN if self.at_frontmatter_edge() => {
                self.advance(3);
                self.after_frontmatter = true;
                T![---]
            }
            _ => self.consume_astro_frontmatter_body(),
        }
    }

    fn consume_svelte(&mut self, current: u8) -> HtmlSyntaxKind {
        let dispatched = lookup_byte(current);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            BEC => self.consume_byte(T!['}']),
            PRD if self.is_at_three_dots() => self.consume_dot3(),
            PRD => self.consume_byte(T![.]),
            COM => self.consume_byte(T![,]),
            PNO => self.consume_byte(T!['(']),
            PNC => self.consume_byte(T![')']),
            BEO if self.at_svelte_block_start() => self.consume_svelte_opening_block(),
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
            b"in" => IN_KW,
            b"of" => OF_KW,
            b"await" => AWAIT_KW,
            b"then" => THEN_KW,
            b"catch" => CATCH_KW,
            b"snippet" => SNIPPET_KW,
            b"bind" => BIND_KW,
            b"transition" => TRANSITION_KW,
            b"animate" => ANIMATE_KW,
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
        let lowercase_buffer = matches!(
            &context,
            IdentifierContext::Doctype | IdentifierContext::None
        );
        let mut buffer = [0u8; BUFFER_SIZE];
        buffer[0] = if lowercase_buffer {
            first.to_ascii_lowercase()
        } else {
            first
        };
        let mut len = 1;

        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            match context {
                IdentifierContext::Doctype | IdentifierContext::None => {
                    if is_attribute_name_byte(byte) {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte.to_ascii_lowercase();
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
                IdentifierContext::VueVForValue => {
                    // Stop at comma, parens, space for v-for bindings
                    if is_vue_v_for_identifier_byte(byte) {
                        if len < BUFFER_SIZE {
                            buffer[len] = byte;
                            len += 1;
                        }

                        self.advance(1)
                    } else {
                        break;
                    }
                }
                IdentifierContext::Angular => {
                    if byte == b')' || byte == b']' {
                        break;
                    }

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
            }
        }

        match &buffer[..len] {
            b"doctype" => DOCTYPE_KW,
            b"html" if context.is_doctype() => HTML_KW,
            b"client" if context.is_astro() && self.current_byte() == Some(b':') => CLIENT_KW,
            b"set" if context.is_astro() && self.current_byte() == Some(b':') => SET_KW,
            b"class" if context.is_astro() && self.current_byte() == Some(b':') => CLASS_KW,
            b"is" if context.is_astro() && self.current_byte() == Some(b':') => IS_KW,
            b"server" if context.is_astro() && self.current_byte() == Some(b':') => SERVER_KW,
            b"define" if context.is_astro() && self.current_byte() == Some(b':') => DEFINE_KW,
            _ => HTML_LITERAL,
        }
    }

    /// Consumes a tag-name token starting with the given byte, classifying it
    /// into a specific token kind according to `mode`:
    ///
    /// - A known HTML/SVG tag name becomes its keyword kind (e.g. `div` -> `DIV_KW`,
    ///   `circle` -> `CIRCLE_KW`). HTML names are matched case-insensitively; SVG
    ///   camelCase names (e.g. `feGaussianBlur`) match by their exact spelling.
    /// - Any name containing `-` or `_` becomes `HTML_COMPONENT_LITERAL`.
    /// - In a framework file a PascalCase name, or any name immediately followed by
    ///   `.` (a member expression base), becomes `HTML_COMPONENT_LITERAL`.
    /// - In the component-name context (`ComponentOnly`) every name becomes
    ///   `HTML_COMPONENT_LITERAL`.
    /// - Anything else (custom elements, namespaced names, unknown tags) becomes
    ///   `HTML_UNKNOWN_TAG`.
    ///
    /// Tag names can contain alphanumeric characters, hyphens, underscores, and colons. In
    /// component contexts dots are excluded and lexed separately for member access.
    fn consume_tag_name(&mut self, first: u8, mode: TagNameMode) -> HtmlSyntaxKind {
        self.assert_current_char_boundary();

        // The longest known tag name is `feComponentTransfer` (19 bytes); anything
        // longer cannot be a known tag, so we only need to buffer up to this many
        // bytes to perform the lookup.
        const BUFFER_SIZE: usize = 24;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut len = 0;
        let mut overflow = false;
        let mut has_component_separator = false;

        // `consume_tag_name` is only entered when the current byte is a tag-name
        // byte, which is always ASCII, so buffering bytes verbatim is sound.
        buffer[0] = first;
        len += 1;
        self.advance_byte_or_char(first);

        while let Some(byte) = self.current_byte() {
            // In component-capable contexts a `.` separates member-expression parts
            // (`Foo.Bar`), so it ends the name. Otherwise (plain HTML / SVG) `.` is
            // a non-spec-compliant tag-name character we keep, matching Prettier.
            let is_name_byte =
                is_tag_name_byte(byte) || (!mode.allows_components() && byte == b'.');
            if is_name_byte {
                has_component_separator |= byte == b'-';
                if len < BUFFER_SIZE {
                    buffer[len] = byte;
                    len += 1;
                } else {
                    overflow = true;
                }
                self.advance(1)
            } else {
                break;
            }
        }

        // Component / member-expression detection.
        if (mode.allows_components() && has_component_separator)
            || mode == TagNameMode::ComponentOnly
        {
            return HTML_COMPONENT_LITERAL;
        }
        if mode.allows_components()
            && (first.is_ascii_uppercase() || self.current_byte() == Some(b'.'))
        {
            return HTML_COMPONENT_LITERAL;
        }

        // A name too long to be a known tag is always unknown.
        if overflow {
            return HTML_UNKNOWN_TAG;
        }

        // Tag names are keywords. Try the exact spelling first (so case-sensitive
        // SVG names like `feGaussianBlur` match), then a lower-cased fallback (so
        // HTML names like `<DIV>` match case-insensitively). Keywords that aren't
        // tag names (e.g. the Svelte `bind` keyword) are rejected and lex as
        // `HTML_UNKNOWN_TAG`.
        let lookup = |name: &str| {
            HtmlSyntaxKind::from_keyword(name).filter(|kind| HTML_TAG_NAMES.contains(*kind))
        };
        let name = &buffer[..len];
        let kind = std::str::from_utf8(name).ok().and_then(lookup).or_else(|| {
            let mut lower = [0u8; BUFFER_SIZE];
            lower[..len].copy_from_slice(name);
            lower[..len].make_ascii_lowercase();
            std::str::from_utf8(&lower[..len]).ok().and_then(lookup)
        });

        kind.unwrap_or(HTML_UNKNOWN_TAG)
    }

    /// Consumes a quoted string literal token.
    /// Returns ERROR_TOKEN if the string is not properly terminated.
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
                // HTML5 makes these a parse error but not a terminator, which is
                // the reading Astro takes.
                b'?' | b'\'' | b'"' | b'=' | b'`'
                    if self.options.framework == HtmlFramework::Astro =>
                {
                    self.advance(1);
                    content_started = true;
                }
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
        } else if self.at_pi_start() {
            self.consume_pi_start()
        } else {
            self.consume_byte(T![<])
        }
    }

    /// Consumes `<?`
    fn consume_pi_start(&mut self) -> HtmlSyntaxKind {
        self.assert_byte(b'<');

        self.advance(2);

        T![<?]
    }

    /// Consumes `<?`
    fn consume_pi_end(&mut self) -> HtmlSyntaxKind {
        self.assert_byte(b'?');

        self.advance(2);

        T![?>]
    }

    /// Consumes an opening double text expression '{{' token used for interpolation.
    fn consume_l_double_text_expression(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_opening_double_text_expression());
        self.advance(2);
        T!["{{"]
    }

    /// Consumes a Svelte opening block token starting with '{' followed by @, #, : or /.
    fn consume_svelte_opening_block(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_svelte_block_start());
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
    fn at_svelte_block_start(&self) -> bool {
        self.current_byte() == Some(b'{')
            && (self.byte_at(1) == Some(b'@')
                || self.byte_at(1) == Some(b'#')
                || self.byte_at(1) == Some(b':')
                || self.byte_at(1) == Some(b'/'))
    }

    fn at_svelte_opening_block(&self) -> bool {
        self.options.framework == HtmlFramework::Svelte && self.at_svelte_block_start()
    }

    #[inline(always)]
    fn at_pi_start(&self) -> bool {
        self.current_byte() == Some(b'<') && self.byte_at(1) == Some(b'?')
    }

    #[inline(always)]
    fn at_pi_end(&self) -> bool {
        self.current_byte() == Some(b'?') && self.byte_at(1) == Some(b'>')
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

    #[inline(always)]
    fn at_vue_v_for_in_keyword(&self) -> bool {
        self.current_byte() == Some(b'i')
            && self.byte_at(1) == Some(b'n')
            && !self.byte_at(2).is_some_and(is_at_continue_identifier)
    }

    fn consume_vue_v_for_in_keyword(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_vue_v_for_in_keyword());
        self.advance(2);
        T![in]
    }

    #[inline(always)]
    fn at_vue_v_for_of_keyword(&self) -> bool {
        self.current_byte() == Some(b'o')
            && self.byte_at(1) == Some(b'f')
            && !self.byte_at(2).is_some_and(is_at_continue_identifier)
    }

    fn consume_vue_v_for_of_keyword(&mut self) -> HtmlSyntaxKind {
        debug_assert!(self.at_vue_v_for_of_keyword());
        self.advance(2);
        T![of]
    }

    /// Consumes a number in v-for expressions (like "n in 10")
    fn consume_vue_v_for_number(&mut self) -> HtmlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            match lookup_byte(byte) {
                ZER | DIG => self.advance(1),
                _ => break,
            }
        }
        HTML_LITERAL
    }

    fn consume_vue_v_for_literal(&mut self) -> HtmlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            let dispatched = lookup_byte(byte);
            if matches!(
                dispatched,
                WHS | QOT | PNO | PNC | COM | BTO | BTC | BEO | BEC | IDT | ZER | DIG
            ) {
                break;
            }

            self.advance_byte_or_char(byte);
        }

        HTML_LITERAL
    }

    fn at_angular_two_way_binding_start(&self) -> bool {
        self.current_byte() == Some(b'[') && self.byte_at(1) == Some(b'(')
    }

    #[inline(always)]
    fn at_angular_two_way_binding_end(&self) -> bool {
        self.current_byte() == Some(b')') && self.byte_at(1) == Some(b']')
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
    fn consume_html_text(&mut self, current: u8, double_text_expressions: bool) -> HtmlSyntaxKind {
        let mut whitespace_started = None;
        let mut seen_newlines = 0;

        let mut closing_expression = None;
        let mut was_escaped = false;

        let dispatched = lookup_byte(current);

        match dispatched {
            BEO => {
                if double_text_expressions && self.at_opening_double_text_expression() {
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

    #[inline]
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
                    HtmlLexContext::Regular { framework } => {
                        self.consume_token(current, framework != HtmlFramework::Svelte)
                    }
                    HtmlLexContext::InsideTag { framework } => {
                        let mode = TagNameMode::for_inside_tag(framework);
                        match framework {
                            HtmlFramework::Plain => {
                                self.consume_token_inside_tag(current, mode, true)
                            }
                            HtmlFramework::Vue => {
                                self.consume_token_inside_tag_directives(current, false, mode)
                            }
                            HtmlFramework::Svelte => {
                                self.consume_token_inside_tag_svelte(current, mode)
                            }
                            HtmlFramework::Astro => {
                                self.consume_token_inside_tag_astro(current, mode)
                            }
                            HtmlFramework::Angular => {
                                self.consume_token_inside_tag_angular(current, mode)
                            }
                        }
                    }
                    HtmlLexContext::InsideTagWithDirectives { svelte } => self
                        .consume_token_inside_tag_directives(
                            current,
                            svelte,
                            TagNameMode::ComponentOnly,
                        ),
                    HtmlLexContext::VueDirectiveArgument => {
                        self.consume_token_vue_directive_argument()
                    }
                    HtmlLexContext::VueVForValue => self.consume_token_vue_v_for_value(current),
                    HtmlLexContext::VueVForExpression(quote) => {
                        self.consume_token_vue_v_for_expression(current, quote)
                    }
                    HtmlLexContext::AttributeValue => self.consume_token_attribute_value(current),
                    HtmlLexContext::SvelteAttributeValue => {
                        self.consume_token_svelte_attribute_value(current)
                    }
                    HtmlLexContext::SvelteTemplateChunk { quote } => {
                        self.consume_token_svelte_template_chunk(current, quote)
                    }
                    HtmlLexContext::Doctype => self.consume_token_doctype(current),
                    HtmlLexContext::EmbeddedLanguage(lang) => {
                        self.consume_token_embedded_language(current, lang)
                    }
                    HtmlLexContext::TextExpression(kind) => match kind {
                        TextExpressionKind::Double => self.consume_double_text_expression(current),
                        TextExpressionKind::Single => self.consume_single_text_expression(),
                    },
                    HtmlLexContext::RestrictedSingleExpression(kind) => {
                        self.consume_restricted_single_text_expression(kind)
                    }
                    HtmlLexContext::CdataSection => self.consume_inside_cdata(current),
                    HtmlLexContext::AstroFencedCodeBlock => self.consume_astro_frontmatter(current),
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

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    #[inline]
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
                HtmlReLexContext::HtmlText { framework } => {
                    self.consume_html_text(current, framework != HtmlFramework::Svelte)
                }
                // Re-lexing is only used mid-tag (e.g. to split `:`/`.`), never at the
                // tag-name position, so the classification mode is irrelevant here.
                HtmlReLexContext::InsideTag => {
                    self.consume_token_inside_tag(current, TagNameMode::Html, true)
                }
                HtmlReLexContext::InsideTagAstro => {
                    self.consume_token_inside_tag_astro(current, TagNameMode::Html)
                }
                HtmlReLexContext::InsideTagSvelte => {
                    self.consume_token_inside_tag_svelte(current, TagNameMode::Html)
                }
                HtmlReLexContext::SingleCurly => self.consume_byte(T!['{']),
                HtmlReLexContext::SvelteAttributeString => self.consume_string_literal(current),
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

    // The extra characters allowed here `-`, `_`, and `:` are not usually allowed in the HTML tag name.
    // However, Prettier considers them to be valid characters in tag names, so we allow them to remain compatible.

    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':')
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

fn slash_starts_regex(previous_non_whitespace: Option<u8>) -> bool {
    match previous_non_whitespace {
        None => true,
        Some(byte) => !matches!(
            lookup_byte(byte),
            IDT | DOL | DIG | ZER | PNC | BTC | PLS | MIN
        ),
    }
}

/// Check if a char is a linebreak (for JS-style comments in Svelte)
fn is_linebreak(chr: char) -> bool {
    matches!(chr, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// For v-for value identifiers: alphanumeric and underscore only
/// Stops at comma, parens, space, quotes
fn is_vue_v_for_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
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

/// Scans embedded JavaScript for the delimiter that ends the construct holding
/// it: the `}` of an Astro or Svelte expression, the `---` fence of Astro
/// frontmatter, or the `]` of a Vue dynamic directive argument.
///
/// The closing delimiter must be plain code: a `}`, `---`, or `]` that sits
/// inside a string, a comment, a regex literal, a template literal, or JSX is
/// ordinary content and does not end the construct. The scanner recognises
/// those nested constructs with a stack of [`JsContext`] frames: opening one
/// pushes a frame, its terminator pops the frame again, and the scan stops at
/// the first stop delimiter found while no frame is open.
struct JsScanner<'src> {
    source: &'src [u8],
    /// The offset of the byte being classified.
    position: usize,
    /// The constructs the current position is nested in, innermost last; an
    /// empty stack means plain code, the only place the stop delimiter ends
    /// the scan. Eight inline frames cover typical nesting depths without a
    /// heap allocation.
    stack: SmallVec<[JsContext; 8]>,
    /// Whether `<` in expression position may open a JSX element.
    jsx: bool,
    /// Whether the scan ever entered JSX from code position.
    entered_jsx: bool,
}

/// One level of JavaScript nesting tracked by [`JsScanner`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsContext {
    /// Code nested inside a delimiter, closed by the given byte.
    Code(u8),
    /// The body of a template literal, between backticks.
    Template,
    /// A JSX tag, between `<` and its `>`.
    JsxTag {
        /// Whether the tag opened with `</`, which never has children.
        closing: bool,
        /// The offset of the tag name, just past the `<` or `</`.
        name_start: usize,
        /// Whether the scan is inside an unquoted attribute value.
        /// A `/` is part of the value.
        in_unquoted_value: bool,
    },
    /// A type argument list on a JSX tag, between `<` and its `>`.
    TypeArguments,
    /// The children of a JSX element, where quotes and braces are text.
    JsxChildren,
}

impl JsContext {
    /// A tag whose name starts at `name_start`, before any attribute is seen.
    const fn jsx_tag(closing: bool, name_start: usize) -> Self {
        Self::JsxTag {
            closing,
            name_start,
            in_unquoted_value: false,
        }
    }
}

/// The delimiter a [`JsScanner`] run stops at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsScanStop {
    /// The `}` closing an Astro or Svelte text expression.
    ExpressionEnd,
    /// The `---` closing Astro frontmatter.
    FrontmatterFence,
    /// The `]` closing a Vue dynamic directive argument.
    ArgumentEnd,
}

impl JsScanStop {
    /// Returns whether `byte` on its own closes the construct for this stop.
    /// [`Self::FrontmatterFence`] never matches here: its delimiter is the
    /// three-byte `---`, which [`JsScanner::run`] matches separately.
    const fn ends_at(self, byte: u8) -> bool {
        matches!(
            (self, byte),
            (Self::ExpressionEnd, b'}') | (Self::ArgumentEnd, b']')
        )
    }
}

impl<'src> JsScanner<'src> {
    /// Returns the offset of the `}` that closes a text expression, or
    /// `source.len()` when the expression is unterminated.
    fn expression_length(source: &'src [u8], jsx: bool) -> usize {
        Self::scan(source, JsScanStop::ExpressionEnd, jsx)
    }

    /// Returns the offset of the `---` that closes Astro frontmatter, or
    /// `source.len()` when the fence is missing. Astro's component script is
    /// TypeScript, so `<` there opens a type assertion rather than JSX.
    fn frontmatter_length(source: &'src [u8]) -> usize {
        Self::scan(source, JsScanStop::FrontmatterFence, false)
    }

    /// Returns the offset of the `]` that closes a Vue dynamic directive
    /// argument, or `source.len()` when the bracket is unbalanced.
    fn argument_length(source: &'src [u8]) -> usize {
        Self::scan(source, JsScanStop::ArgumentEnd, false)
    }

    fn scan(source: &'src [u8], stop: JsScanStop, jsx: bool) -> usize {
        let mut scanner = Self {
            source,
            position: 0,
            stack: SmallVec::new(),
            jsx,
            entered_jsx: false,
        };
        let length = scanner.run(stop);
        if length < source.len() || !scanner.entered_jsx {
            return length;
        }

        // Running off the end inside JSX means a `<` was really a comparison
        // or a cast, misread as an element whose closing tag never comes;
        // rescan reading `<` as an operator.
        scanner.position = 0;
        scanner.stack.clear();
        scanner.jsx = false;
        scanner.run(stop)
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.get(self.position).copied()
    }

    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.get(self.position + offset).copied()
    }

    /// Clamped to the source end, so an escape at the very end cannot step past it.
    fn advance(&mut self, count: usize) {
        self.position = (self.position + count).min(self.source.len());
    }

    fn scanned(&self) -> &[u8] {
        &self.source[..self.position]
    }

    fn previous_non_whitespace(&self) -> Option<u8> {
        self.scanned()
            .iter()
            .rev()
            .copied()
            .find(|byte| !byte.is_ascii_whitespace())
    }

    fn set_in_unquoted_value(&mut self, value: bool) {
        if let Some(JsContext::JsxTag {
            in_unquoted_value, ..
        }) = self.stack.last_mut()
        {
            *in_unquoted_value = value;
        }
    }

    fn run(&mut self, stop: JsScanStop) -> usize {
        while let Some(byte) = self.current_byte() {
            match self.stack.last().copied() {
                None | Some(JsContext::Code(_)) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    SLH => self.skip_slash(),
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    BTO if stop == JsScanStop::ArgumentEnd => {
                        self.stack.push(JsContext::Code(b']'));
                        self.advance(1);
                    }
                    BEC | BTC => {
                        if self.stack.last() == Some(&JsContext::Code(byte)) {
                            self.stack.pop();
                        } else if self.stack.is_empty() && stop.ends_at(byte) {
                            return self.position;
                        }
                        self.advance(1);
                    }
                    MIN if stop == JsScanStop::FrontmatterFence
                        && self.stack.is_empty()
                        && self.source[self.position..].starts_with(b"---") =>
                    {
                        return self.position;
                    }
                    LSS if self.jsx && self.jsx_element_starts() => {
                        self.entered_jsx = true;
                        self.stack
                            .push(JsContext::jsx_tag(false, self.position + 1));
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::Template) => match lookup_byte(byte) {
                    BSL => self.advance(2),
                    TPL => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    DOL if self.byte_at(1) == Some(b'{') => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(2);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::JsxTag {
                    closing,
                    name_start,
                    in_unquoted_value,
                }) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    EQL => {
                        self.set_in_unquoted_value(starts_unquoted_value(self.byte_at(1)));
                        self.advance(1);
                    }
                    MOR => {
                        self.stack.pop();
                        let self_closing =
                            !in_unquoted_value && self.previous_non_whitespace() == Some(b'/');
                        if !closing && !self_closing && !is_void_element(self.tag_name(name_start))
                        {
                            self.stack.push(JsContext::JsxChildren);
                        }
                        self.advance(1);
                    }
                    // A `,` cannot appear in a JSX tag, so this was a type parameter list.
                    COM => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    LSS => {
                        self.stack.push(JsContext::TypeArguments);
                        self.advance(1);
                    }
                    _ if in_unquoted_value && byte.is_ascii_whitespace() => {
                        self.set_in_unquoted_value(false);
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::TypeArguments) => match lookup_byte(byte) {
                    QOT => self.skip_string(byte),
                    TPL => {
                        self.stack.push(JsContext::Template);
                        self.advance(1);
                    }
                    LSS => {
                        self.stack.push(JsContext::TypeArguments);
                        self.advance(1);
                    }
                    // The `>` of a function type's arrow does not close the list.
                    EQL if self.byte_at(1) == Some(b'>') => self.advance(2),
                    MOR => {
                        self.stack.pop();
                        self.advance(1);
                    }
                    _ => self.advance(1),
                },

                Some(JsContext::JsxChildren) => match lookup_byte(byte) {
                    BEO => {
                        self.stack.push(JsContext::Code(b'}'));
                        self.advance(1);
                    }
                    LSS => match self.byte_at(1) {
                        Some(b'/') => {
                            self.stack.pop();
                            self.stack.push(JsContext::jsx_tag(true, self.position + 2));
                            self.advance(2);
                        }
                        Some(next) if starts_tag_name(next) => {
                            self.stack
                                .push(JsContext::jsx_tag(false, self.position + 1));
                            self.advance(1);
                        }
                        _ => self.advance(1),
                    },
                    _ => self.advance(1),
                },
            }
        }

        self.position
    }

    /// Skips the string literal whose opening `quote` is the current byte,
    /// stopping just past the closing quote or at the end of the source.
    fn skip_string(&mut self, quote: u8) {
        self.advance(1);
        while let Some(byte) = self.current_byte() {
            self.advance(1);
            if byte == b'\\' {
                self.advance(1);
            } else if byte == quote {
                break;
            }
        }
    }

    /// Skips from a `/` in code position past the comment or regex literal it
    /// opens, or past the slash alone.
    fn skip_slash(&mut self) {
        match self.byte_at(1) {
            Some(b'/') => {
                self.advance(2);
                self.skip_line_comment();
            }
            Some(b'*') => {
                self.advance(2);
                self.skip_block_comment();
            }
            // `/>` closes a tag that was not recognised as JSX; it never opens a regex.
            Some(b'>') => self.advance(1),
            _ if self.scanned().last() != Some(&b'<')
                && slash_starts_regex(self.previous_non_whitespace()) =>
            {
                self.advance(1);
                self.skip_regex();
            }
            _ => self.advance(1),
        }
    }

    /// Skips the regex literal body, stopping just past the closing `/` or at
    /// the end of the source. A `/` inside a `[…]` character class does not
    /// close the literal.
    fn skip_regex(&mut self) {
        let mut in_character_class = false;

        while let Some(byte) = self.current_byte() {
            self.advance(1);
            match byte {
                b'\\' => self.advance(1),
                b'[' => in_character_class = true,
                b']' => in_character_class = false,
                b'/' if !in_character_class => break,
                _ => {}
            }
        }
    }

    /// Skips just past the terminating newline, or to the end of the source.
    fn skip_line_comment(&mut self) {
        while let Some(byte) = self.current_byte() {
            self.advance(1);
            if byte == b'\n' {
                break;
            }
        }
    }

    /// Skips just past the closing `*/`, or to the end of the source.
    fn skip_block_comment(&mut self) {
        while let Some(byte) = self.current_byte() {
            if byte == b'*' && self.byte_at(1) == Some(b'/') {
                self.advance(2);
                return;
            }
            self.advance(1);
        }
    }

    /// Returns whether the `<` at the current position opens a JSX element
    /// rather than a comparison, a shift, or a TypeScript type argument list.
    fn jsx_element_starts(&self) -> bool {
        self.byte_at(1).is_some_and(starts_tag_name) && at_expression_position(self.scanned())
    }

    /// Returns the tag name starting at `start`, which is empty for a fragment.
    fn tag_name(&self, start: usize) -> &[u8] {
        let name = self.source.get(start..).unwrap_or_default();
        let length = name
            .iter()
            .position(|byte| !is_tag_name_byte(*byte))
            .unwrap_or(name.len());

        &name[..length]
    }
}

/// Returns whether a byte can follow `<` in a JSX element, counting the `>` of
/// a fragment.
fn starts_tag_name(byte: u8) -> bool {
    byte == b'>' || byte == b'_' || byte.is_ascii_alphabetic()
}

/// Returns whether `name` is a [void element](https://html.spec.whatwg.org/#void-elements),
/// which never has children. The keyword lookup is exact because Astro matches
/// the spec names case-sensitively: `<BR>` is a component, not a line break.
fn is_void_element(name: &[u8]) -> bool {
    std::str::from_utf8(name)
        .ok()
        .and_then(HtmlSyntaxKind::from_keyword)
        .is_some_and(|kind| VOID_ELEMENTS.contains(kind))
}

/// Returns whether `byte`, the one right after an `=` in a tag, opens an
/// unquoted attribute value.
fn starts_unquoted_value(byte: Option<u8>) -> bool {
    byte.is_some_and(|byte| {
        !matches!(byte, b'"' | b'\'' | b'`' | b'{') && !byte.is_ascii_whitespace()
    })
}

/// Returns whether the code scanned so far ends where an expression may start.
/// After an operand — a non-keyword identifier, a literal, or a closing bracket
/// — the next `<` is an operator instead.
fn at_expression_position(scanned: &[u8]) -> bool {
    let Some(index) = scanned.iter().rposition(|byte| !byte.is_ascii_whitespace()) else {
        return true;
    };

    if is_js_word_byte(scanned[index]) {
        let start = scanned[..index]
            .iter()
            .rposition(|byte| !is_js_word_byte(*byte))
            .map_or(0, |index| index + 1);
        return matches!(
            &scanned[start..=index],
            b"await"
                | b"case"
                | b"delete"
                | b"do"
                | b"else"
                | b"in"
                | b"instanceof"
                | b"new"
                | b"of"
                | b"return"
                | b"throw"
                | b"typeof"
                | b"void"
                | b"yield"
        );
    }

    !matches!(scanned[index], b')' | b']' | b'"' | b'\'' | b'`')
}

fn is_js_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

#[cfg(test)]
mod js_scanner {
    use crate::lexer::JsScanner;

    /// The offset of the closing `---` fence, or `None` when the scan reaches
    /// the end of the source without finding one.
    fn fence(source: &str) -> Option<usize> {
        let length = JsScanner::frontmatter_length(source.as_bytes());
        (length < source.len()).then_some(length)
    }

    /// The offset of the `}` closing a text expression, or `None` when the
    /// expression is unterminated.
    fn expression(source: &str) -> Option<usize> {
        let length = JsScanner::expression_length(source.as_bytes(), true);
        (length < source.len()).then_some(length)
    }

    #[test]
    fn unterminated_quotes_hide_the_fence() {
        assert_eq!(fence("'\"`\n---\n"), None);
        assert_eq!(fence("`'\"\"'\n---\n"), None);
    }

    #[test]
    fn balanced_quotes_leave_the_fence_visible() {
        assert_eq!(fence(" '\"``\"' \n---\n"), Some(9));
    }

    #[test]
    fn issue_9108_double_quoted_with_apostrophe() {
        assert!(fence("\"test'\"\n---\n").is_some());
    }

    #[test]
    fn issue_9108_single_quoted_with_double_quote() {
        assert!(fence("'it\"s'\n---\n").is_some());
    }

    #[test]
    fn issue_9108_template_with_mixed_quotes() {
        assert!(fence("`it's a \"test\"`\n---\n").is_some());
    }

    #[test]
    fn issue_9108_multiple_strings_with_mixed_quotes() {
        assert!(fence("const a = \"it's\"; const b = 'say \"hi\"';\n---\n").is_some());
    }

    #[test]
    fn issue_8882_multiline_block_comment_with_apostrophe() {
        assert!(fence("/*\n * Doesn't that stink?\n */\n---\n").is_some());
    }

    #[test]
    fn issue_8882_jsdoc_comment_then_code() {
        let source = "/**\n * In this comment, if you add any string opening or closing, such as an apostrophe, the file will show \n * a bunch of errors. Doesn't (remove the apostrophe in the previous word to fix) that stink? \n */\nimport type { HTMLAttributes } from \"astro/types\";\nconst { class: className } = Astro.props;\n---\n";
        assert!(fence(source).is_some());
    }

    #[test]
    fn issue_8882_multiline_block_comment_quotes_every_line() {
        let source = "/* line one 'unclosed\n   line two `unclosed\n   line three \"unclosed */\nconst x = 1;\n---\n";
        assert!(fence(source).is_some());
    }

    #[test]
    fn fence_inside_single_line_comment_is_not_a_fence() {
        assert_eq!(fence("// ---"), None);
    }

    #[test]
    fn fence_inside_block_comment_is_not_a_fence() {
        assert_eq!(fence("/*\n---\n"), None);
    }

    #[test]
    fn a_line_comment_ends_at_its_newline() {
        assert!(fence("// ---\nconst f = \"something\"\n---\n").is_some());
    }

    #[test]
    fn escaped_double_quote_inside_double_string() {
        assert!(fence("\"\\\"\"\n---\n").is_some());
    }

    #[test]
    fn escaped_single_quote_inside_single_string() {
        assert!(fence("'\\''\n---\n").is_some());
    }

    #[test]
    fn escaped_backtick_inside_template() {
        assert!(fence("`\\``\n---\n").is_some());
    }

    #[test]
    fn double_backslash_then_quote() {
        assert!(fence("\"\\\\\"\n---\n").is_some());
    }

    #[test]
    fn issue_9187_regex_with_single_quote() {
        assert!(fence("const test = /'/\n---\n").is_some());
    }

    #[test]
    fn issue_9187_regex_with_double_quote() {
        assert!(fence("const test = /\"/\n---\n").is_some());
    }

    #[test]
    fn issue_9187_regex_with_dashes() {
        assert!(fence("const test = /---/\n---\n").is_some());
    }

    #[test]
    fn issue_9187_regex_with_escape_and_quantifier() {
        assert!(fence("const test = /\\d{4}/\n---\n").is_some());
    }

    #[test]
    fn regex_character_class_may_contain_a_slash() {
        assert!(fence("const a = /[/]'/;\n---\n").is_some());
    }

    #[test]
    fn nested_template_interpolation_closes_the_fence() {
        assert!(fence("const a = `x${`/y`}`;\n---\n").is_some());
    }

    #[test]
    fn nested_template_interpolation_ends_the_expression() {
        let source = "`/blog${n === 0 ? '' : `/${n + 1}`}`}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn apostrophe_in_jsx_text_ends_the_expression() {
        let source = "items.map((i) => <li>it's {i}</li>)}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_self_closing_element_ends_the_expression() {
        let source = "<Icon />}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_void_element_has_no_children() {
        let source = "cond && <span>It's<br>ok</span>}";
        assert_eq!(expression(source), Some(source.len() - 1));
        let source = "open && <label><input type=\"text\">don't</label>}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    /// Astro matches the void element names case-sensitively.
    #[test]
    fn an_uppercase_void_name_is_a_component() {
        let source = "x && <BR>it's</BR>}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_slash_in_an_unquoted_value_does_not_close_the_tag() {
        let source = "x && <div a=4/>it's</div>}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_slash_after_an_unquoted_value_closes_the_tag() {
        let source = "x && <div a=4 />}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_template_attribute_value_may_contain_an_angle_bracket() {
        let source = "x && <C a=`b > c` />}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn type_arguments_do_not_end_a_tag() {
        let source = "x && <List<string>>it's</List>}";
        assert_eq!(expression(source), Some(source.len() - 1));
        let source = "x && <Table<Map<string, number>>>it's</Table>}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_function_type_argument_does_not_end_the_list() {
        let source = "x && <C<(x: A) => B>>don't</C>}";
        assert_eq!(expression(source), Some(source.len() - 1));
        let source = "x && <C<() => B> />}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_fragment_ends_the_expression() {
        let source = "<><b>a</b></>}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn returned_jsx_ends_the_expression() {
        let source = "() => { return <div>a</div>; }}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_comparison_does_not_start_jsx() {
        let source = "a < c}";
        assert_eq!(expression(source), Some(source.len() - 1));
        let source = "a<c}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_generic_call_does_not_start_jsx() {
        let source = "new Map<string, number>()}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_type_parameter_list_does_not_start_jsx() {
        let source = "<T,>(x: T) => x}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    /// A `<` that opens neither JSX nor a recognisably non-JSX construct
    /// swallows the rest of the source; the scan retries without JSX rather
    /// than let the expression run away.
    #[test]
    fn a_runaway_element_falls_back_to_scanning_without_jsx() {
        let source = "await <T extends U>(v)}";
        assert_eq!(expression(source), Some(source.len() - 1));
    }

    #[test]
    fn a_trailing_escape_does_not_scan_past_the_input() {
        let source = "`a\\";
        assert_eq!(expression(source), None);
        assert_eq!(
            JsScanner::expression_length(source.as_bytes(), true),
            source.len()
        );
    }

    /// Astro's frontmatter is TypeScript, where `<` opens a type assertion.
    #[test]
    fn a_type_assertion_in_frontmatter_closes_the_fence() {
        assert_eq!(fence("const a = <string>x;\n---\n"), Some(21));
    }

    #[test]
    fn a_closing_tag_in_frontmatter_does_not_open_a_regex() {
        assert!(fence("const items = xs.map((x) => <li>{x}</li>);\n---\n").is_some());
    }
}
