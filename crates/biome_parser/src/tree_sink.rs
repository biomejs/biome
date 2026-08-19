use crate::prelude::*;
use crate::token_source::Trivia;
use biome_rowan::{
    Language, NodeCache, SyntaxFactory, SyntaxKind, SyntaxNode, TextRange, TextSize, TreeBuilder,
    TriviaPiece,
};

/// Maximum depth of the green tree produced by the parser. This bounds the
/// recursive `Drop` depth of syntax nodes on pathological input.
const MAX_TREE_DEPTH: usize = 2048;

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
    depth_limit_exceeded: bool,
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
        if self.parents_count < MAX_TREE_DEPTH {
            self.inner.start_node(kind);
        } else if !self.depth_limit_exceeded {
            self.report_depth_limit();
        }
        self.parents_count += 1;
    }

    fn finish_node(&mut self) {
        self.parents_count -= 1;

        if self.parents_count == 0 && self.needs_eof {
            self.do_token(L::Kind::EOF, TextSize::from(self.text.len() as u32));
        }

        if self.parents_count < MAX_TREE_DEPTH {
            self.inner.finish_node();
        }
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
            depth_limit_exceeded: false,
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
            depth_limit_exceeded: false,
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

    #[cold]
    fn report_depth_limit(&mut self) {
        self.depth_limit_exceeded = true;
        let diagnostic = ParseDiagnostic::new(
            format!("Syntax nesting exceeds maximum depth of {MAX_TREE_DEPTH}."),
            TextRange::empty(self.text_pos),
        );
        let index = self.errors.partition_point(|error| match error.span() {
            Some(range) => range.start() <= self.text_pos,
            None => true,
        });
        self.errors.insert(index, diagnostic);
    }

    #[inline]
    fn do_token(&mut self, kind: L::Kind, token_end: TextSize) {
        if kind == L::Kind::EOF {
            self.needs_eof = false;
        }

        let token_start = self.text_pos;

        // Every trivia up to the token (including line breaks) will be the leading trivia
        self.eat_trivia(false, token_end);
        let trailing_start = self.trivia_pieces.len();

        self.text_pos = token_end;

        // Everything until the next linebreak (but not including it)
        // will be the trailing trivia...
        self.eat_trivia(true, token_end);

        let token_range = TextRange::new(token_start, self.text_pos);

        let text = &self.text[token_range];
        let leading = &self.trivia_pieces[0..trailing_start];
        let trailing = &self.trivia_pieces[trailing_start..];

        self.inner.token_with_trivia(kind, text, leading, trailing);
        self.trivia_pieces.clear();
    }

    fn eat_trivia(&mut self, trailing: bool, token_end: TextSize) {
        for trivia in &self.trivia_list[self.trivia_pos..] {
            if trailing != trivia.trailing()
                || self.text_pos != trivia.offset()
                // Some non-trivia tokens have zero length. In that case to check whether this
                // trivia is that token's leading trivia we also need to take into account the
                // trivia end offset.
                || (!trailing && trivia.end_offset() > token_end)
            {
                break;
            }

            let trivia_piece = TriviaPiece::new(trivia.kind(), trivia.len());
            self.trivia_pieces.push(trivia_piece);

            self.text_pos += trivia.len();
            self.trivia_pos += 1;
        }
    }
}

/// An offset-aware wrapper around `LosslessTreeSink` for parsing embedded content.
///
/// This wrapper applies a base offset to all text positions during parsing,
/// allowing embedded content (like JavaScript in HTML script tags) to maintain
/// correct source positions relative to the parent document.
#[derive(Debug)]
pub struct OffsetLosslessTreeSink<'a, L, Factory>
where
    L: Language,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    inner: LosslessTreeSink<'a, L, Factory>,
    base_offset: TextSize,
}

impl<'a, L, Factory> OffsetLosslessTreeSink<'a, L, Factory>
where
    L: Language + 'static,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    /// Create a new offset-aware tree sink with the given base offset
    pub fn new(text: &'a str, trivia: &'a [Trivia], base_offset: TextSize) -> Self {
        Self {
            inner: LosslessTreeSink::new(text, trivia),
            base_offset,
        }
    }

    /// Create a new offset-aware tree sink with cache and base offset
    pub fn with_cache(
        text: &'a str,
        trivia: &'a [Trivia],
        cache: &'a mut NodeCache,
        base_offset: TextSize,
    ) -> Self {
        Self {
            inner: LosslessTreeSink::with_cache(text, trivia, cache),
            base_offset,
        }
    }

    /// Finishes the tree and returns the root node with possible parser errors.
    ///
    /// The returned syntax node will have all its text ranges adjusted by the base offset.
    pub fn finish(self) -> (biome_rowan::SyntaxNodeWithOffset<L>, Vec<ParseDiagnostic>) {
        let (node, diagnostics) = self.inner.finish();
        let offset_node = biome_rowan::SyntaxNodeWithOffset::new(node, self.base_offset);
        (offset_node, diagnostics)
    }
}

impl<L, Factory> TreeSink for OffsetLosslessTreeSink<'_, L, Factory>
where
    L: Language,
    Factory: SyntaxFactory<Kind = L::Kind>,
{
    type Kind = L::Kind;

    fn token(&mut self, kind: L::Kind, end: TextSize) {
        // Forward to inner sink - the offset will be applied when finishing
        self.inner.token(kind, end);
    }

    fn start_node(&mut self, kind: L::Kind) {
        self.inner.start_node(kind);
    }

    fn finish_node(&mut self) {
        self.inner.finish_node();
    }

    fn errors(&mut self, errors: Vec<ParseDiagnostic>) {
        self.inner.errors(errors);
    }
}

#[cfg(test)]
mod tests {
    use super::{LosslessTreeSink, MAX_TREE_DEPTH, TreeSink};
    use biome_rowan::{
        SyntaxNode, TextRange, TextSize,
        raw_language::{RawLanguage, RawLanguageKind, RawLanguageSyntaxFactory},
    };

    type RawTreeSink<'a> = LosslessTreeSink<'a, RawLanguage, RawLanguageSyntaxFactory>;

    fn nested_tree(depth: usize) -> (SyntaxNode<RawLanguage>, Vec<crate::ParseDiagnostic>) {
        let mut sink = RawTreeSink::new("a", &[]);

        for _ in 0..depth {
            sink.start_node(RawLanguageKind::ROOT);
        }
        sink.token(RawLanguageKind::STRING_TOKEN, TextSize::from(1));
        for _ in 0..depth {
            sink.finish_node();
        }

        sink.finish()
    }

    #[test]
    fn accepts_tree_at_depth_limit() {
        let (root, diagnostics) = nested_tree(MAX_TREE_DEPTH);

        assert_eq!(root.to_string(), "a");
        assert_eq!(root.descendants().count(), MAX_TREE_DEPTH);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn depth_limit_diagnostic_keeps_existing_errors_sorted() {
        let mut sink = RawTreeSink::new("a", &[]);
        sink.errors(vec![crate::ParseDiagnostic::new(
            "later error",
            TextRange::empty(TextSize::from(1)),
        )]);

        for _ in 0..=MAX_TREE_DEPTH {
            sink.start_node(RawLanguageKind::ROOT);
        }
        sink.token(RawLanguageKind::STRING_TOKEN, TextSize::from(1));
        for _ in 0..=MAX_TREE_DEPTH {
            sink.finish_node();
        }

        let (_, diagnostics) = sink.finish();
        let spans: Vec<_> = diagnostics
            .iter()
            .map(|error| error.span().unwrap())
            .collect();
        assert_eq!(
            spans,
            [
                TextRange::empty(TextSize::from(0)),
                TextRange::empty(TextSize::from(1)),
            ]
        );
    }

    #[test]
    fn reports_tree_past_depth_limit_once() {
        let (root, diagnostics) = nested_tree(MAX_TREE_DEPTH + 100);

        assert_eq!(root.to_string(), "a");
        assert_eq!(root.descendants().count(), MAX_TREE_DEPTH);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].span(),
            Some(TextRange::empty(TextSize::from(0)))
        );
    }
}
