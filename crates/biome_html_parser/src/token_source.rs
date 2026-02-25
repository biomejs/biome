use crate::lexer::HtmlLexer;
use biome_html_syntax::HtmlSyntaxKind::{AS_KW, CATCH_KW, EOF, THEN_KW};
use biome_html_syntax::{HtmlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{
    TokenSource, TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia,
};
use biome_rowan::TriviaPieceKind;

pub(crate) struct HtmlTokenSource<'source> {
    lexer: BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HtmlLexContext {
    /// The default state. This state is used for lexing outside of tags.
    ///
    /// When the lexer is outside of a tag, special characters are lexed as text.
    ///
    /// The exceptions being `<` which indicates the start of a tag, and `>` which is invalid syntax if not preceded with a `<`.
    #[default]
    Regular,
    /// When the lexer is inside a tag, special characters are lexed as tag tokens.
    InsideTag,
    /// Like [InsideTag], but with Vue-style directive tokens enabled (`.`, `:`, `@`, `#`, etc.).
    /// When `svelte` is `true`, also recognizes `//` and `/* */` as JS-style comments (for Svelte
    /// component names which need both member-expression `.` support and comment support).
    InsideTagWithDirectives { svelte: bool },
    /// Like [InsideTag], but with Svelte-specific tokens enabled.
    /// This enables parsing of JS-style `//` and `/* */` comments as trivia.
    InsideTagSvelte,
    /// Like [InsideTag], but with Astro-specific tokens enabled.
    /// This enables parsing of Astro directives (client:, set:, class:, is:, server:)
    InsideTagAstro,
    /// Lexes Vue directive arguments inside `[]`.
    VueDirectiveArgument,
    /// When the parser encounters a `=` token (the beginning of the attribute initializer clause), it switches to this context.
    ///
    /// This is because attribute values can start and end with a `"` or `'` character, or be unquoted, and the lexer needs to know to start lexing a string literal.
    AttributeValue,

    /// Context to be used when parsing the contents of Svelte blocks. Svelte blocks usually start with `{@`, `{:`, `{/` or `{#`.
    /// When lexing using this context, specific tokens are emitted such as `if`, `else`, `debug`, etc.
    ///
    /// Outside of this context, the lexer doesn't yield any particular keywords.
    Svelte,

    /// The binding properties in Svelte are special and require a special lexing. They accept everything until `=` is found.
    SvelteBindingLiteral,

    /// Lex tokens inside text expressions. In the following examples, `foo` is the text expression:
    /// - `{{ foo }}`
    /// - `attr={ foo }`
    TextExpression(TextExpressionKind),
    /// Single expression that stops at certain keywords (e.g., 'as' in Svelte each blocks)
    RestrictedSingleExpression(RestrictedExpressionStopAt),
    /// Enables the `html` keyword token.
    ///
    /// When the parser has encounters the sequence `<!DOCTYPE`, it switches to this context. It will remain in this context until the next `>` token is encountered.
    Doctype,
    /// Treat everything as text until the closing tag is encountered.
    EmbeddedLanguage(HtmlEmbeddedLanguage),
    /// CDATA Sections are treated as text until the closing CDATA token is encountered.
    CdataSection,
    /// Lexing the Astro frontmatter. When in this context, the lexer will treat `---`
    /// as a boundary for `HTML_LITERAL`
    AstroFencedCodeBlock,
}

impl HtmlLexContext {
    pub fn single_expression() -> Self {
        Self::TextExpression(TextExpressionKind::Single)
    }

    pub fn double_expression() -> Self {
        Self::TextExpression(TextExpressionKind::Double)
    }

    pub fn restricted_expression(kind: RestrictedExpressionStopAt) -> Self {
        Self::RestrictedSingleExpression(kind)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum TextExpressionKind {
    // {{ expr }}
    #[default]
    Double,
    // { expr }
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RestrictedExpressionStopAt {
    /// Stops at 'as' keyword or ',' (for Svelte #each blocks)
    AsOrComma,
    /// Stops at `)`
    ClosingParen,
    /// Stops at `then` or `catch` keywords
    ThenOrCatch,
}

impl RestrictedExpressionStopAt {
    pub(crate) fn matches_punct(&self, byte: u8) -> bool {
        match self {
            Self::AsOrComma => byte == b',',
            Self::ClosingParen => byte == b')',
            Self::ThenOrCatch => false,
        }
    }

    pub(crate) fn matches_keyword(&self, keyword: HtmlSyntaxKind) -> bool {
        match self {
            Self::AsOrComma => keyword == AS_KW,
            Self::ClosingParen => false,
            Self::ThenOrCatch => keyword == THEN_KW || keyword == CATCH_KW,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HtmlEmbeddedLanguage {
    Script,
    Style,
    Preformatted,
}

impl HtmlEmbeddedLanguage {
    pub fn end_tag(&self) -> &'static str {
        match self {
            Self::Script => "</script>",
            Self::Style => "</style>",
            Self::Preformatted => "</pre>",
        }
    }
}

impl LexContext for HtmlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum HtmlReLexContext {
    /// Specialised relex that manages certain characters such as commas, etc.
    Svelte,
    /// Relex tokens using `HtmlLexer::consume_html_text`
    HtmlText,
    /// Relex tokens as if the parser was inside a tag.
    InsideTag,
    /// Relex tokens as if the parser was inside a tag in an Astro file.
    InsideTagAstro,
}

pub(crate) type HtmlTokenSourceCheckpoint = TokenSourceCheckpoint<HtmlSyntaxKind>;

impl<'source> HtmlTokenSource<'source> {
    /// Creates a new token source for the given string
    pub fn from_str(source: &'source str) -> Self {
        let lexer = HtmlLexer::from_str(source);

        let buffered = BufferedLexer::new(lexer);
        let mut source = Self::new(buffered);

        source.next_non_trivia_token(HtmlLexContext::Regular, true);
        source
    }

    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: vec![],
        }
    }

    fn next_non_trivia_token(&mut self, context: HtmlLexContext, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(context);

            let trivia_kind = TriviaPieceKind::try_from(kind);

            match trivia_kind {
                Err(_) => {
                    // Not trivia
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                    }

                    self.trivia_list
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
        }
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> HtmlTokenSourceCheckpoint {
        HtmlTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: HtmlTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }

    pub fn re_lex(&mut self, mode: HtmlReLexContext) -> HtmlSyntaxKind {
        self.lexer.re_lex(mode)
    }

    /// Signals to the lexer that the frontmatter decision has been made.
    /// After this call, `---` in the `Regular` context is treated as plain
    /// HTML text rather than a `FENCE` token.
    pub fn set_after_frontmatter(&mut self, value: bool) {
        self.lexer.lexer_mut().set_after_frontmatter(value);
    }
}

impl TokenSource for HtmlTokenSource<'_> {
    type Kind = HtmlSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.lexer.current()
    }

    fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    fn bump(&mut self) {
        self.bump_with_context(HtmlLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(HtmlLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for HtmlTokenSource<'_> {
    type Context = HtmlLexContext;
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> TokenSourceWithBufferedLexer<HtmlLexer<'source>> for HtmlTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<HtmlSyntaxKind, HtmlLexer<'source>> {
        &mut self.lexer
    }
}
