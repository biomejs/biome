use crate::lexer::{BufferedLexer, LexerWithCheckpoint};
use crate::{diagnostic::ParseDiagnostic, lexer::LexerCheckpoint};
use biome_rowan::{SyntaxKind, TextRange, TextSize, TriviaPieceKind};

/// A comment or a whitespace trivia in the source code.
#[derive(Debug, Copy, Clone)]
pub struct Trivia {
    /// The kind of the trivia token.
    kind: TriviaPieceKind,

    /// The range of the trivia in the source text
    range: TextRange,

    /// Whatever this is the trailing or leading trivia of a non-trivia token.
    trailing: bool,
}

impl Trivia {
    pub fn new(kind: TriviaPieceKind, range: TextRange, trailing: bool) -> Self {
        Self {
            kind,
            range,
            trailing,
        }
    }
    /// Returns the kind of the token
    pub fn kind(&self) -> TriviaPieceKind {
        self.kind
    }

    /// Returns the token's length in bytes
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns the byte offset of the trivia in the source text
    pub fn offset(&self) -> TextSize {
        self.range.start()
    }

    /// Returns `true` if this is the trailing trivia of a non-trivia token or false otherwise.
    pub fn trailing(&self) -> bool {
        self.trailing
    }

    /// Returns the text range of this trivia
    pub fn text_range(&self) -> TextRange {
        self.range
    }
}

pub trait TokenSource {
    type Kind: SyntaxKind;

    /// Returns the kind of the current non-trivia token
    fn current(&self) -> Self::Kind;

    /// Returns the range of the current non-trivia token
    fn current_range(&self) -> TextRange;

    /// Returns the source text
    fn text(&self) -> &str;

    /// Returns the byte offset of the current token from the start of the source document
    fn position(&self) -> TextSize {
        self.current_range().start()
    }

    /// Returns true if the current token is preceded by a line break
    fn has_preceding_line_break(&self) -> bool;

    fn bump(&mut self);

    fn skip_as_trivia(&mut self);

    /// Ends this token source and returns the source text's trivia
    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>);
}

pub trait BumpWithContext: TokenSource {
    type Context;

    fn bump_with_context(&mut self, context: Self::Context);

    /// Skips the current token as skipped token trivia
    fn skip_as_trivia_with_context(&mut self, context: Self::Context);
}

pub trait TokenSourceWithBufferedLexer<Lex>: TokenSource {
    fn lexer(&mut self) -> &mut BufferedLexer<Self::Kind, Lex>;
}

/// Token source that supports inspecting the 'nth' token (lookahead)
pub trait NthToken<Lex>: TokenSource {
    /// Gets the kind of the nth non-trivia token
    fn nth(&mut self, n: usize) -> Self::Kind;

    /// Returns true if the nth non-trivia token is preceded by a line break
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool;
}

impl<'l, Lex, T> NthToken<Lex> for T
where
    T: TokenSourceWithBufferedLexer<Lex>,
    Lex: LexerWithCheckpoint<'l, Kind = T::Kind>,
{
    /// Gets the kind of the nth non-trivia token
    fn nth(&mut self, n: usize) -> T::Kind {
        if n == 0 {
            self.current()
        } else {
            self.lexer()
                .nth_non_trivia(n)
                .map_or(T::Kind::EOF, |lookahead| lookahead.kind())
        }
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.has_preceding_line_break()
        } else {
            self.lexer()
                .nth_non_trivia(n)
                .is_some_and(|lookahead| lookahead.has_preceding_line_break())
        }
    }
}

#[derive(Debug)]
pub struct TokenSourceCheckpoint<K>
where
    K: SyntaxKind,
{
    pub lexer_checkpoint: LexerCheckpoint<K>,
    /// A `u32` should be enough because `TextSize` is also limited to `u32`.
    /// The worst case is a document where every character is its own token. This would
    /// result in `u32::MAX` tokens
    pub trivia_len: u32,
}

impl<K> TokenSourceCheckpoint<K>
where
    K: SyntaxKind,
{
    /// byte offset in the source text
    pub fn current_start(&self) -> TextSize {
        self.lexer_checkpoint.current_start()
    }

    pub fn trivia_position(&self) -> usize {
        self.trivia_len as usize
    }
}
