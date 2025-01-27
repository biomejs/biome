use crate::prelude::*;
use crate::token_source::Trivia;
use biome_rowan::{
    Language, NodeCache, SyntaxFactory, SyntaxKind, SyntaxNode, TextRange, TextSize, TreeBuilder,
    TriviaPiece,
};

/// An abstraction for syntax tree implementations
pub trait TreeSink {
    type Kind: SyntaxKind;

    /// Adds new token to the current branch.
    fn token(&mut self, kind: Self::Kind, end: TextSize);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: Self::Kind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    /// Emit errors
    fn errors(&mut self, errors: Vec<ParseDiagnostic>);
}

/// Structure for converting events to a syntax tree representation, while preserving whitespace.
///
/// `LosslessTreeSink` also handles attachment of trivia (whitespace) to nodes.
#[derive(Debug)]
pub struct LosslessTreeSink<'a, L, Factory>
where
    L: Language,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    text: &'a str,
    trivia_list: &'a [Trivia],
    text_pos: TextSize,
    trivia_pos: usize,
    parents_count: usize,
    errors: Vec<ParseDiagnostic>,
    inner: TreeBuilder<'a, L, Factory>,
    /// Signal that the sink must generate an EOF token when its finishing. See [LosslessTreeSink::finish] for more details.
    needs_eof: bool,
    trivia_pieces: Vec<TriviaPiece>,
}

impl<L, Factory> TreeSink for LosslessTreeSink<'_, L, Factory>
where
    L: Language,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    type Kind = L::Kind;

    fn token(&mut self, kind: L::Kind, end: TextSize) {
        self.do_token(kind, end);
    }

    fn start_node(&mut self, kind: L::Kind) {
        self.inner.start_node(kind);
        self.parents_count += 1;
    }

    fn finish_node(&mut self) {
        self.parents_count -= 1;

        if self.parents_count == 0 && self.needs_eof {
            self.do_token(L::Kind::EOF, TextSize::from(self.text.len() as u32));
        }

        self.inner.finish_node();
    }

    fn errors(&mut self, errors: Vec<ParseDiagnostic>) {
        self.errors = errors;
    }
}

impl<'a, L, Factory> LosslessTreeSink<'a, L, Factory>
where
    L: Language,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    pub fn new(text: &'a str, trivia: &'a [Trivia]) -> Self {
        Self {
            text,
            trivia_list: trivia,
            text_pos: 0.into(),
            trivia_pos: 0,
            parents_count: 0,
            inner: TreeBuilder::default(),
            errors: vec![],
            needs_eof: true,
            trivia_pieces: Vec::with_capacity(128),
        }
    }

    /// Reusing `NodeCache` between different [LosslessTreeSink]s saves memory.
    /// It allows to structurally share underlying trees.
    pub fn with_cache(text: &'a str, trivia: &'a [Trivia], cache: &'a mut NodeCache) -> Self {
        Self {
            text,
            trivia_list: trivia,
            text_pos: 0.into(),
            trivia_pos: 0,
            parents_count: 0,
            inner: TreeBuilder::with_cache(cache),
            errors: vec![],
            needs_eof: true,
            trivia_pieces: Vec::with_capacity(128),
        }
    }

    /// Finishes the tree and return the root node with possible parser errors.
    ///
    /// If tree is finished without a [biome_rowan::SyntaxKind::EOF], one will be generated and all pending trivia
    /// will be appended to its leading trivia.
    pub fn finish(self) -> (SyntaxNode<L>, Vec<ParseDiagnostic>) {
        (self.inner.finish(), self.errors)
    }

    #[inline]
    fn do_token(&mut self, kind: L::Kind, token_end: TextSize) {
        if kind == L::Kind::EOF {
            self.needs_eof = false;
        }

        let token_start = self.text_pos;

        // Every trivia up to the token (including line breaks) will be the leading trivia
        self.eat_trivia(false);
        let trailing_start = self.trivia_pieces.len();

        self.text_pos = token_end;

        // Everything until the next linebreak (but not including it)
        // will be the trailing trivia...
        self.eat_trivia(true);

        let token_range = TextRange::new(token_start, self.text_pos);

        let text = &self.text[token_range];
        let leading = &self.trivia_pieces[0..trailing_start];
        let trailing = &self.trivia_pieces[trailing_start..];

        self.inner.token_with_trivia(kind, text, leading, trailing);
        self.trivia_pieces.clear();
    }

    fn eat_trivia(&mut self, trailing: bool) {
        for trivia in &self.trivia_list[self.trivia_pos..] {
            if trailing != trivia.trailing() || self.text_pos != trivia.offset() {
                break;
            }

            let trivia_piece = TriviaPiece::new(trivia.kind(), trivia.len());
            self.trivia_pieces.push(trivia_piece);

            self.text_pos += trivia.len();
            self.trivia_pos += 1;
        }
    }
}
