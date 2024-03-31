use crate::lexer::HtmlLexer;
use biome_html_syntax::HtmlSyntaxKind::EOF;
use biome_html_syntax::{HtmlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::{BumpWithContext, NthToken};
use biome_parser::token_source::{TokenSource, Trivia};
use biome_rowan::TriviaPieceKind;
use std::collections::VecDeque;

pub(crate) struct HtmlTokenSource<'source> {
    lexer: BufferedLexer<'source, HtmlLexer<'source>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
    /// Cache for the non-trivia token lookahead. For example for the source `.class {};` if the
    /// [TokenSource]'s currently positioned at the start of the file (`.`). The `nth(2)` non-trivia token,
    /// as returned by the [TokenSource], is the `{` token but retrieving it requires skipping over the
    /// one whitespace trivia tokens (between `class` and `{`).
    /// The [TokenSource] state then is:
    ///
    /// * `non_trivia_lookahead`: [IDENT: 'class', L_CURLY]
    /// * `lookahead_offset`: 3 (the `{` is the 3th token after the `.` keyword)
    non_trivia_lookahead: VecDeque<Lookahead>,

    /// Offset of the last cached lookahead token from the current [BufferedLexer] token.
    lookahead_offset: usize,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum HtmlLexContext {
    /// The default state
    #[default]
    Regular,
    #[allow(unused)]
    /// When the lexer is inside a element list, newlines, spaces and quotes are part of the text
    ElementList,
}

#[derive(Debug, Copy, Clone)]
struct Lookahead {
    kind: HtmlSyntaxKind,
    after_newline: bool,
}

impl LexContext for HtmlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }
}

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
    pub(crate) fn new(lexer: BufferedLexer<'source, HtmlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: vec![],
            lookahead_offset: 0,
            non_trivia_lookahead: VecDeque::new(),
        }
    }

    fn next_non_trivia_token(&mut self, context: HtmlLexContext, first_token: bool) {
        let mut processed_tokens = 0;
        let mut trailing = !first_token;

        // Drop the last cached lookahead, we're now moving past it
        self.non_trivia_lookahead.pop_front();

        loop {
            let kind = self.lexer.next_token(context);
            processed_tokens += 1;

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

        if self.lookahead_offset != 0 {
            debug_assert!(
                self.lookahead_offset >= processed_tokens,
                "lookadead offeset: {}, processed tokens: {}",
                self.lookahead_offset,
                processed_tokens
            );
            self.lookahead_offset -= processed_tokens;
        }
    }

    #[inline(always)]
    fn lookahead(&mut self, n: usize) -> Option<Lookahead> {
        assert_ne!(n, 0);

        // Return the cached token if any
        if let Some(lookahead) = self.non_trivia_lookahead.get(n - 1) {
            return Some(*lookahead);
        }

        // Jump right to where we've left of last time rather than going through all tokens again.
        let iter = self.lexer.lookahead().skip(self.lookahead_offset);
        let mut remaining = n - self.non_trivia_lookahead.len();

        for item in iter {
            self.lookahead_offset += 1;

            if !item.kind().is_trivia() {
                remaining -= 1;

                let lookahead = Lookahead {
                    after_newline: item.has_preceding_line_break(),
                    kind: item.kind(),
                };

                self.non_trivia_lookahead.push_back(lookahead);

                if remaining == 0 {
                    return Some(lookahead);
                }
            }
        }

        None
    }
}

impl<'src> TokenSource for HtmlTokenSource<'src> {
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

impl<'source> BumpWithContext for HtmlTokenSource<'source> {
    type Context = HtmlLexContext;
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            if !context.is_regular() {
                self.lookahead_offset = 0;
                self.non_trivia_lookahead.clear();
            }

            self.next_non_trivia_token(context, false);
        }
    }

    fn skip_as_trivia_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            if !context.is_regular() {
                self.lookahead_offset = 0;
                self.non_trivia_lookahead.clear();
            }

            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> NthToken for HtmlTokenSource<'source> {
    #[inline(always)]

    fn nth(&mut self, n: usize) -> Self::Kind {
        if n == 0 {
            self.current()
        } else {
            self.lookahead(n).map_or(EOF, |lookahead| lookahead.kind)
        }
    }
    #[inline(always)]
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.has_preceding_line_break()
        } else {
            self.lookahead(n)
                .map_or(false, |lookahead| lookahead.after_newline)
        }
    }
}
