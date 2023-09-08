use crate::diagnostic::ParseDiagnostic;
use rome_rowan::{SyntaxKind, TextRange, TextSize, TriviaPieceKind};

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

/// Token source that supports inspecting the 'nth' token (lookahead)
pub trait NthToken: TokenSource {
    /// Gets the kind of the nth non-trivia token
    fn nth(&mut self, n: usize) -> Self::Kind;

    /// Returns true if the nth non-trivia token is preceded by a line break
    fn has_nth_preceding_line_break(&mut self, n: usize) -> bool;
}
