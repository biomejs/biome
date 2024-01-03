use crate::lexer::{CssLexContext, CssLexer, CssReLexContext};
use crate::CssParserOptions;
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_css_syntax::{CssSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{BufferedLexer, LexContext};
use biome_parser::prelude::{BumpWithContext, NthToken, TokenSource};
use biome_parser::token_source::{TokenSourceCheckpoint, Trivia};
use biome_rowan::TriviaPieceKind;
use std::collections::VecDeque;

pub(crate) struct CssTokenSource<'src> {
    lexer: BufferedLexer<'src, CssLexer<'src>>,

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

#[derive(Debug, Copy, Clone)]
struct Lookahead {
    kind: CssSyntaxKind,
    after_newline: bool,
}

pub(crate) type CssTokenSourceCheckpoint = TokenSourceCheckpoint<CssSyntaxKind>;

impl<'src> CssTokenSource<'src> {
    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<'src, CssLexer<'src>>) -> CssTokenSource<'src> {
        CssTokenSource {
            lexer,
            trivia_list: vec![],
            lookahead_offset: 0,
            non_trivia_lookahead: VecDeque::new(),
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(source: &'src str, config: CssParserOptions) -> Self {
        let lexer = CssLexer::from_str(source).with_config(config);

        let buffered = BufferedLexer::new(lexer);
        let mut source = CssTokenSource::new(buffered);

        source.next_non_trivia_token(CssLexContext::default(), true);
        source
    }

    fn next_non_trivia_token(&mut self, context: CssLexContext, first_token: bool) {
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
            debug_assert!(self.lookahead_offset >= processed_tokens);
            self.lookahead_offset -= processed_tokens;
        }
    }

    pub fn re_lex(&mut self, mode: CssReLexContext) -> CssSyntaxKind {
        let current_kind = self.current();

        let new_kind = self.lexer.re_lex(mode);

        // Only need to clear the lookahead cache when the token did change
        if current_kind != new_kind {
            self.non_trivia_lookahead.clear();
            self.lookahead_offset = 0;
        }

        new_kind
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

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> CssTokenSourceCheckpoint {
        CssTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: CssTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
        self.non_trivia_lookahead.clear();
        self.lookahead_offset = 0;
    }
}

impl<'source> TokenSource for CssTokenSource<'source> {
    type Kind = CssSyntaxKind;

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
        self.bump_with_context(CssLexContext::Regular)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(CssLexContext::Regular)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl<'source> BumpWithContext for CssTokenSource<'source> {
    type Context = CssLexContext;

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

impl<'source> NthToken for CssTokenSource<'source> {
    /// Gets the kind of the nth non-trivia token
    #[inline(always)]
    fn nth(&mut self, n: usize) -> CssSyntaxKind {
        if n == 0 {
            self.current()
        } else {
            self.lookahead(n).map_or(EOF, |lookahead| lookahead.kind)
        }
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
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
