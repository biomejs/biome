use std::collections::VecDeque;

use crate::lexer::GraphqlLexer;
use biome_graphql_syntax::GraphqlSyntaxKind::EOF;
use biome_graphql_syntax::{GraphqlSyntaxKind, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::TokenSource;
use biome_parser::token_source::{NthToken, Trivia};
use biome_rowan::TriviaPieceKind;

pub(crate) struct GraphqlTokenSource<'source> {
    lexer: BufferedLexer<'source, GraphqlLexer<'source>>,
    trivia_list: Vec<Trivia>,

    /// Cache for the non-trivia token lookahead. For example for the source `let a = 10;` if the
    /// [TokenSource]'s currently positioned at the start of the file (`let`). The `nth(2)` non-trivia token,
    /// as returned by the [TokenSource], is the `=` token but retrieving it requires skipping over the
    /// two whitespace trivia tokens (first between `let` and `a`, second between `a` and `=`).
    /// The [TokenSource] state then is:
    ///
    /// * `non_trivia_lookahead`: [IDENT: 'a', EQ]
    /// * `lookahead_offset`: 4 (the `=` is the 4th token after the `let` keyword)
    non_trivia_lookahead: VecDeque<Lookahead>,

    /// Offset of the last cached lookahead token from the current [BufferedLexer] token.
    lookahead_offset: usize,
}

#[derive(Debug, Copy, Clone)]
struct Lookahead {
    kind: GraphqlSyntaxKind,
    after_newline: bool,
}

impl<'source> GraphqlTokenSource<'source> {
    pub(crate) fn new(lexer: BufferedLexer<'source, GraphqlLexer<'source>>) -> Self {
        Self {
            lexer,
            trivia_list: Vec::new(),
            non_trivia_lookahead: VecDeque::new(),
            lookahead_offset: 0,
        }
    }
    pub fn from_str(source: &'source str) -> Self {
        let lexer = GraphqlLexer::from_str(source);
        let lexer = BufferedLexer::new(lexer);

        let mut source = GraphqlTokenSource::new(lexer);

        source.next_non_trivia_token(true);
        source
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

    fn next_non_trivia_token(&mut self, first_token: bool) {
        let mut processed_tokens = 0;
        let mut trailing = !first_token;

        // Drop the last cached lookahead, we're now moving past it
        self.non_trivia_lookahead.pop_front();

        loop {
            let kind = self.lexer.next_token(());
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
}

impl<'source> TokenSource for GraphqlTokenSource<'source> {
    type Kind = GraphqlSyntaxKind;

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
        if self.current() != EOF {
            self.next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl<'source> NthToken for GraphqlTokenSource<'source> {
    /// Gets the kind of the nth non-trivia token
    fn nth(&mut self, n: usize) -> GraphqlSyntaxKind {
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
