mod borrowed_token;
mod element;
mod node;
mod rewriter;
mod token;
mod trivia;

use crate::{AstNode, RawSyntaxKind};
pub use borrowed_token::{
    BorrowedToken, BorrowedTokensIterator, BorrowedTriviaPiece, BorrowedTriviaPiecesIterator,
};
pub use element::{SyntaxElement, SyntaxElementKey};
pub use node::{
    AnySyntaxNode, EmbeddedSendNode, Preorder, PreorderOfKinds, PreorderWithTokens, SendNode,
    SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxNodeOptionExt,
    SyntaxNodeWithOffset, SyntaxSlot, SyntaxSlots,
};
pub use rewriter::{SyntaxRewriter, VisitNodeSignal};
use std::fmt;
use std::fmt::Debug;
pub use token::{SyntaxToken, SyntaxTokenWithOffset};
pub use trivia::{
    ChainTriviaPiecesIterator, SyntaxTrivia, SyntaxTriviaPiece, SyntaxTriviaPieceComments,
    SyntaxTriviaPieceNewline, SyntaxTriviaPieceSkipped, SyntaxTriviaPieceWhitespace,
    SyntaxTriviaPiecesIterator, TriviaPiece, TriviaPieceKind, chain_trivia_pieces,
    trim_leading_trivia_pieces, trim_trailing_trivia_pieces,
};

/// Type tag for each node or token of a language
pub trait SyntaxKind: fmt::Debug + PartialEq + Copy {
    const TOMBSTONE: Self;
    const EOF: Self;

    /// Returns `true` if this is a kind of a bogus node.
    fn is_bogus(&self) -> bool;

    /// Converts this into to the best matching bogus node kind.
    fn to_bogus(&self) -> Self;

    /// Converts this kind to a raw syntax kind.
    fn to_raw(&self) -> RawSyntaxKind;

    /// Creates a syntax kind from a raw kind.
    fn from_raw(raw: RawSyntaxKind) -> Self;

    /// Returns `true` if this kind is for a root node.
    fn is_root(&self) -> bool;

    /// Returns `true` if this kind is a list node.
    fn is_list(&self) -> bool;

    /// Returns `true` if this kind is a trivia.
    fn is_trivia(self) -> bool;

    /// Returns a string for keywords, punctuation tokens, and the `EOL` token,
    /// or `None` otherwise.
    fn to_string(&self) -> Option<&'static str>;

    /// Returns `true` if this kind is allowed to precede file suppression comments,
    fn is_allowed_before_suppressions(&self) -> bool {
        false
    }
}

pub trait Language: Sized + Clone + Copy + fmt::Debug + Eq + Ord + std::hash::Hash {
    type Kind: SyntaxKind;
    type Root: AstNode<Language = Self> + Clone + Eq + fmt::Debug;
}

/// A list of `SyntaxNode`s and/or `SyntaxToken`s
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxList<L: Language> {
    list: SyntaxNode<L>,
}

impl<L: Language> SyntaxList<L> {
    /// Creates a new list wrapping a List `SyntaxNode`
    fn new(node: SyntaxNode<L>) -> Self {
        Self { list: node }
    }

    /// Iterates over the elements in the list.
    pub fn iter(&self) -> SyntaxSlots<L> {
        self.list.slots()
    }

    /// Returns the number of items in this list
    pub fn len(&self) -> usize {
        self.list.slots().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<SyntaxSlot<L>> {
        self.list.slots().next()
    }

    pub fn last(&self) -> Option<SyntaxSlot<L>> {
        self.list.slots().last()
    }

    pub fn node(&self) -> &SyntaxNode<L> {
        &self.list
    }

    pub fn into_node(self) -> SyntaxNode<L> {
        self.list
    }
}

impl<L: Language> IntoIterator for &SyntaxList<L> {
    type Item = SyntaxSlot<L>;
    type IntoIter = SyntaxSlots<L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<L: Language> IntoIterator for SyntaxList<L> {
    type Item = SyntaxSlot<L>;
    type IntoIter = SyntaxSlots<L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use biome_text_size::TextRange;

    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::syntax::{
        Preorder, PreorderWithTokens, SyntaxElement, SyntaxNode, SyntaxToken, SyntaxTriviaPiece,
        TriviaPiece, TriviaPieceKind,
    };
    use crate::{Direction, SyntaxKindSet, WalkEvent};

    #[test]
    fn empty_list() {
        let mut builder: RawSyntaxTreeBuilder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder.finish_node();
        let list = builder.finish().into_list();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        assert_eq!(list.first(), None);
        assert_eq!(list.last(), None);

        assert_eq!(list.iter().collect::<Vec<_>>(), Vec::default());
    }

    #[test]
    fn node_list() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::EXPRESSION_LIST);

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.finish_node();

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
        builder.finish_node();

        builder.finish_node();

        let node = builder.finish();
        let list = node.into_list();

        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        let first = list.first().and_then(|e| e.into_node()).unwrap();
        assert_eq!(first.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(first.text_with_trivia(), "1");

        let last = list.last().and_then(|e| e.into_node()).unwrap();
        assert_eq!(last.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(last.text_with_trivia(), "2");

        let node_texts: Vec<_> = list
            .iter()
            .map(|e| e.into_node().map(|n| n.text_with_trivia().to_string()))
            .collect();

        assert_eq!(
            node_texts,
            vec![Some(String::from("1")), Some(String::from("2"))]
        )
    }

    #[test]
    fn node_or_token_list() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.finish_node();

        builder.token(RawLanguageKind::NUMBER_TOKEN, ",");

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
        builder.finish_node();

        builder.finish_node();

        let node = builder.finish();
        let list = node.into_list();

        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);

        let first = list.first().and_then(|e| e.into_node()).unwrap();
        assert_eq!(first.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(first.text_with_trivia(), "1");

        let last = list.last().and_then(|e| e.into_node()).unwrap();
        assert_eq!(last.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(last.text_with_trivia(), "2");

        let kinds: Vec<_> = list.iter().map(|e| e.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                Some(RawLanguageKind::LITERAL_EXPRESSION),
                Some(RawLanguageKind::NUMBER_TOKEN),
                Some(RawLanguageKind::LITERAL_EXPRESSION)
            ]
        )
    }

    #[test]
    fn siblings() {
        let mut builder = RawSyntaxTreeBuilder::new();

        // list
        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        // element 1
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "a");
        builder.finish_node();

        // element 2
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "b");
        builder.finish_node();

        // Missing ,

        // element 3
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "c");
        builder.finish_node();

        builder.finish_node();

        let root = builder.finish();

        let first = root.children().next().unwrap();
        assert_eq!(first.text_with_trivia().to_string(), "a");
        assert_eq!(
            first
                .next_sibling()
                .map(|e| e.text_with_trivia().to_string()),
            Some(String::from("b"))
        );

        let second = root.children().nth(1).unwrap();
        assert_eq!(second.text_with_trivia().to_string(), "b");

        // Skips the missing element
        assert_eq!(
            second
                .next_sibling()
                .map(|e| e.text_with_trivia().to_string()),
            Some(String::from("c"))
        );

        assert_eq!(
            second
                .prev_sibling()
                .map(|e| e.text_with_trivia().to_string()),
            Some(String::from("a"))
        );

        let last = root.children().last().unwrap();
        assert_eq!(last.text_with_trivia(), "c");
        assert_eq!(last.next_sibling(), None);
        assert_eq!(
            last.prev_sibling()
                .map(|e| e.text_with_trivia().to_string()),
            Some(String::from("b"))
        );

        assert_eq!(
            first
                .siblings(Direction::Next)
                .map(|s| s.text_with_trivia().to_string())
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );

        assert_eq!(
            last.siblings(Direction::Prev)
                .map(|s| s.text_with_trivia().to_string())
                .collect::<Vec<_>>(),
            vec!["c", "b", "a"]
        );
    }

    #[test]
    fn siblings_with_tokens() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::ROOT);

        builder.token(RawLanguageKind::FOR_KW, "for");
        builder.token(RawLanguageKind::L_PAREN_TOKEN, "(");
        builder.token(RawLanguageKind::SEMICOLON_TOKEN, ";");

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "x");
        builder.finish_node();

        builder.token(RawLanguageKind::SEMICOLON_TOKEN, ";");
        builder.token(RawLanguageKind::R_PAREN_TOKEN, ")");

        builder.finish_node();

        let root = builder.finish();

        let first_semicolon = root
            .children_with_tokens()
            .nth(2)
            .and_then(|e| e.into_token())
            .unwrap();

        assert_eq!(first_semicolon.text(), ";");

        assert_eq!(
            first_semicolon
                .siblings_with_tokens(Direction::Next)
                .map(|e| e.to_string())
                .collect::<Vec<_>>(),
            vec!["x", ";", ")"]
        );

        assert_eq!(
            first_semicolon.next_sibling_or_token(),
            first_semicolon.siblings_with_tokens(Direction::Next).next()
        );
        assert_eq!(
            first_semicolon.prev_sibling_or_token(),
            first_semicolon.siblings_with_tokens(Direction::Prev).next()
        );
    }

    fn trivia_texts(pieces: impl Iterator<Item = SyntaxTriviaPiece<RawLanguage>>) -> Vec<String> {
        pieces.map(|piece| piece.text().to_owned()).collect()
    }

    fn node_with_token(kind: RawLanguageKind, text: &str) -> SyntaxNode<RawLanguage> {
        let token = SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, text, [], []);
        SyntaxNode::new_detached(kind, [Some(token.into())])
    }

    fn token_navigation_tree() -> SyntaxNode<RawLanguage> {
        let before = SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, "before", [], []);
        let inside = SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, "inside", [], []);
        let after = SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, "after", [], []);
        let leading_empty = SyntaxNode::new_detached(RawLanguageKind::BOGUS, []);
        let trailing_empty = SyntaxNode::new_detached(RawLanguageKind::BOGUS, []);
        let subtree = SyntaxNode::new_detached(
            RawLanguageKind::BOGUS,
            [
                Some(leading_empty.into()),
                Some(inside.into()),
                Some(trailing_empty.into()),
            ],
        );
        let empty = SyntaxNode::new_detached(RawLanguageKind::BOGUS, [None]);

        SyntaxNode::new_detached(
            RawLanguageKind::ROOT,
            [
                Some(before.into()),
                Some(subtree.into()),
                None,
                Some(empty.into()),
                Some(after.into()),
            ],
        )
    }

    #[test]
    fn kind_filtered_child_and_sibling_navigation() {
        let first_literal = node_with_token(RawLanguageKind::LITERAL_EXPRESSION, "1");
        let condition = node_with_token(RawLanguageKind::CONDITION, "2");
        let last_literal = node_with_token(RawLanguageKind::LITERAL_EXPRESSION, "3");
        let leading =
            SyntaxToken::<RawLanguage>::new_detached(RawLanguageKind::STRING_TOKEN, "a", [], []);
        let plus =
            SyntaxToken::<RawLanguage>::new_detached(RawLanguageKind::PLUS_TOKEN, "+", [], []);
        let root = SyntaxNode::new_detached(
            RawLanguageKind::ROOT,
            [
                Some(leading.into()),
                Some(first_literal.into()),
                None,
                Some(condition.into()),
                Some(plus.into()),
                Some(last_literal.into()),
            ],
        );
        let literals = SyntaxKindSet::<RawLanguage>::of(RawLanguageKind::LITERAL_EXPRESSION);
        let condition = SyntaxKindSet::<RawLanguage>::of(RawLanguageKind::CONDITION);
        let condition_or_plus = condition.union(SyntaxKindSet::of(RawLanguageKind::PLUS_TOKEN));

        let literal_texts: Vec<_> = root
            .children_of_kinds(literals)
            .map(|node| node.text_with_trivia().to_string())
            .collect();
        assert_eq!(literal_texts, ["1", "3"]);
        assert_eq!(
            root.first_child_of_kinds(literals)
                .unwrap()
                .text_with_trivia(),
            "1"
        );
        assert!(root.first_child_of_kinds(SyntaxKindSet::EMPTY).is_none());

        let elements: Vec<_> = root
            .children_with_tokens_of_kinds(condition_or_plus)
            .map(|element| (element.kind(), element.to_string()))
            .collect();
        assert_eq!(
            elements,
            [
                (RawLanguageKind::CONDITION, "2".to_string()),
                (RawLanguageKind::PLUS_TOKEN, "+".to_string()),
            ]
        );
        assert_eq!(
            root.first_child_or_token_of_kinds(SyntaxKindSet::of(RawLanguageKind::STRING_TOKEN))
                .unwrap()
                .to_string(),
            "a"
        );

        let mut children = root.children();
        let first_literal = children.next().unwrap();
        let condition_node = children.next().unwrap();
        let last_literal = children.next().unwrap();

        assert_eq!(
            first_literal.next_sibling_of_kinds(condition),
            Some(condition_node.clone())
        );
        assert_eq!(
            first_literal.next_sibling_of_kinds(literals),
            Some(last_literal.clone())
        );
        assert_eq!(
            condition_node
                .next_sibling_or_token_of_kinds(condition_or_plus)
                .unwrap()
                .kind(),
            RawLanguageKind::PLUS_TOKEN
        );
        assert!(last_literal.next_sibling_of_kinds(literals).is_none());
    }

    #[test]
    fn nodes_and_elements_navigate_to_surrounding_tokens() {
        let root = token_navigation_tree();
        let before = root.first_token().unwrap();
        let after = root.last_token().unwrap();
        let mut root_children = root.children();
        let subtree = root_children.next().unwrap();
        let empty = root_children.next().unwrap();
        let inside = subtree.first_token().unwrap();
        let mut subtree_children = subtree.children();
        let leading_empty = subtree_children.next().unwrap();
        let trailing_empty = subtree_children.next().unwrap();

        assert_eq!(root.prev_token(), None);
        assert_eq!(root.next_token(), None);
        assert_eq!(subtree.prev_token(), Some(before.clone()));
        assert_eq!(subtree.next_token(), Some(after.clone()));
        assert_eq!(leading_empty.prev_token(), Some(before.clone()));
        assert_eq!(leading_empty.next_token(), Some(inside.clone()));
        assert_eq!(trailing_empty.prev_token(), Some(inside.clone()));
        assert_eq!(trailing_empty.next_token(), Some(after.clone()));
        assert_eq!(empty.prev_token(), Some(inside.clone()));
        assert_eq!(empty.next_token(), Some(after.clone()));

        let subtree_element: SyntaxElement<_> = subtree.into();
        assert_eq!(subtree_element.first_token(), Some(inside.clone()));
        assert_eq!(subtree_element.last_token(), Some(inside.clone()));
        assert_eq!(subtree_element.prev_token(), Some(before.clone()));
        assert_eq!(subtree_element.next_token(), Some(after.clone()));

        let empty_element: SyntaxElement<_> = empty.into();
        assert_eq!(empty_element.first_token(), None);
        assert_eq!(empty_element.last_token(), None);
        assert_eq!(empty_element.prev_token(), Some(inside.clone()));
        assert_eq!(empty_element.next_token(), Some(after.clone()));

        let before_element: SyntaxElement<_> = before.clone().into();
        assert_eq!(before_element.first_token(), Some(before.clone()));
        assert_eq!(before_element.last_token(), Some(before.clone()));
        assert_eq!(before_element.prev_token(), None);
        assert_eq!(before_element.next_token(), Some(inside.clone()));

        let inside_element: SyntaxElement<_> = inside.into();
        assert_eq!(inside_element.prev_token(), Some(before));
        assert_eq!(inside_element.next_token(), Some(after));
    }

    #[test]
    fn token_navigation_keeps_zero_width_tokens() {
        let before = SyntaxToken::<RawLanguage>::new_detached(
            RawLanguageKind::STRING_TOKEN,
            "before",
            [],
            [],
        );
        let zero_width = SyntaxToken::<RawLanguage>::new_detached(RawLanguageKind::EOF, "", [], []);
        let after = SyntaxToken::<RawLanguage>::new_detached(
            RawLanguageKind::STRING_TOKEN,
            "after",
            [],
            [],
        );
        let subtree = SyntaxNode::new_detached(RawLanguageKind::BOGUS, [Some(before.into())]);
        let root = SyntaxNode::new_detached(
            RawLanguageKind::ROOT,
            [
                Some(subtree.into()),
                Some(zero_width.into()),
                Some(after.into()),
            ],
        );
        let subtree = root.children().next().unwrap();
        let zero_width = subtree.next_token().unwrap();

        assert_eq!(zero_width.kind(), RawLanguageKind::EOF);
        assert_eq!(zero_width.text(), "");
        assert_eq!(zero_width.prev_token(), subtree.last_token());
        assert_eq!(zero_width.next_token().unwrap().text(), "after");
    }

    #[test]
    fn tree_top_handles_nodes_and_detached_tokens() {
        let root = token_navigation_tree();
        let subtree = root.children().next().unwrap();
        let token = subtree.first_token().unwrap();
        let node_element: SyntaxElement<_> = subtree.clone().into();
        let token_element: SyntaxElement<_> = token.clone().into();

        assert_eq!(root.tree_top(), root);
        assert_eq!(subtree.tree_top(), root);
        assert_eq!(token.tree_top(), Some(root.clone()));
        assert_eq!(
            token.ancestors().collect::<Vec<_>>(),
            token.parent_ancestors().collect::<Vec<_>>()
        );
        assert_eq!(node_element.tree_top(), Some(root.clone()));
        assert_eq!(token_element.tree_top(), Some(root));

        let detached_node = SyntaxNode::<RawLanguage>::new_detached(RawLanguageKind::BOGUS, []);
        let detached_token = SyntaxToken::<RawLanguage>::new_detached(
            RawLanguageKind::STRING_TOKEN,
            "token",
            [],
            [],
        );
        let detached_node_element: SyntaxElement<_> = detached_node.clone().into();
        let detached_token_element: SyntaxElement<_> = detached_token.clone().into();

        assert_eq!(detached_node.tree_top(), detached_node);
        assert_eq!(detached_token.tree_top(), None);
        assert_eq!(detached_node_element.tree_top(), Some(detached_node));
        assert_eq!(detached_token_element.tree_top(), None);
    }

    #[test]
    fn trivia_around_tokens_nodes_and_elements_is_in_source_order() {
        let root = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "a /*a*/",
                &[],
                &[
                    TriviaPiece::whitespace(1),
                    TriviaPiece::single_line_comment(5),
                ],
            );
            builder.start_node(RawLanguageKind::BOGUS);
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                "\n /*b*/b \t",
                &[
                    TriviaPiece::newline(1),
                    TriviaPiece::whitespace(1),
                    TriviaPiece::single_line_comment(5),
                ],
                &[TriviaPiece::whitespace(2)],
            );
            builder.finish_node();
            builder.token_with_trivia(
                RawLanguageKind::STRING_TOKEN,
                " //c\nc",
                &[
                    TriviaPiece::whitespace(1),
                    TriviaPiece::single_line_comment(3),
                    TriviaPiece::newline(1),
                ],
                &[],
            );
            builder.start_node(RawLanguageKind::BOGUS);
            builder.finish_node();
        });

        let first = root.first_token().unwrap();
        let middle_node = root.children().next().unwrap();
        let middle = middle_node.first_token().unwrap();
        let last = root.last_token().unwrap();
        let tokenless = root.children().nth(1).unwrap();
        let middle_element: SyntaxElement<_> = middle_node.clone().into();

        let before_middle = [" ", "/*a*/", "\n", " ", "/*b*/"];
        assert_eq!(trivia_texts(first.trivia_after()), before_middle);
        assert_eq!(trivia_texts(middle.trivia_before()), before_middle);
        assert_eq!(trivia_texts(middle_node.trivia_before()), before_middle);
        assert_eq!(trivia_texts(middle_element.trivia_before()), before_middle);
        assert_eq!(
            trivia_texts(middle.trivia_before().rev()),
            ["/*b*/", " ", "\n", "/*a*/", " "]
        );

        let after_middle = [" \t", " ", "//c", "\n"];
        assert_eq!(trivia_texts(middle.trivia_after()), after_middle);
        assert_eq!(trivia_texts(last.trivia_before()), after_middle);
        assert_eq!(trivia_texts(middle_node.trivia_after()), after_middle);
        assert_eq!(trivia_texts(middle_element.trivia_after()), after_middle);

        assert!(first.trivia_before().next().is_none());
        assert!(last.trivia_after().next().is_none());
        assert!(tokenless.trivia_before().next().is_none());
        assert!(tokenless.trivia_after().next().is_none());
    }

    #[test]
    fn preorder_iterators_are_cloneable_and_debuggable() {
        let root = token_navigation_tree();
        let mut nodes = root.preorder();
        nodes.next();
        let cloned_nodes = nodes.clone();
        let _ = format!("{cloned_nodes:?}");
        let node_events = |events: Preorder<RawLanguage>| {
            events
                .map(|event| match event {
                    WalkEvent::Enter(node) => (true, node.kind()),
                    WalkEvent::Leave(node) => (false, node.kind()),
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(node_events(nodes), node_events(cloned_nodes));

        let mut elements = root.preorder_with_tokens(Direction::Next);
        elements.next();
        let cloned_elements = elements.clone();
        let _ = format!("{cloned_elements:?}");
        let element_events = |events: PreorderWithTokens<RawLanguage>| {
            events
                .map(|event| match event {
                    WalkEvent::Enter(element) => (true, element.kind()),
                    WalkEvent::Leave(element) => (false, element.kind()),
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(element_events(elements), element_events(cloned_elements));
    }

    #[test]
    fn edge_tokens_stay_within_the_requested_tokenless_subtree() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token(RawLanguageKind::STRING_TOKEN, "before");
        builder.start_node(RawLanguageKind::BOGUS);
        builder.start_node(RawLanguageKind::BOGUS);
        builder.finish_node();
        builder.finish_node();
        builder.token(RawLanguageKind::STRING_TOKEN, "after");
        builder.finish_node();

        let root = builder.finish();
        let tokenless = root
            .children_with_tokens()
            .nth(1)
            .and_then(|element| element.into_node())
            .unwrap();

        assert_eq!(tokenless.first_token(), None);
        assert_eq!(tokenless.last_token(), None);
    }

    #[test]
    fn edge_tokens_fall_back_across_empty_children_and_keep_zero_length_tokens() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::BOGUS);
        builder.start_node(RawLanguageKind::BOGUS);
        builder.finish_node();
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "first");
        builder.finish_node();
        builder.token(RawLanguageKind::EOF, "");
        builder.start_node(RawLanguageKind::BOGUS);
        builder.finish_node();
        builder.finish_node();
        builder.finish_node();

        let root = builder.finish();
        let subtree = root.children().next().unwrap();

        assert_eq!(subtree.first_token().unwrap().text(), "first");
        assert_eq!(subtree.last_token().unwrap().text(), "");
    }

    #[test]
    fn edge_tokens_keep_zero_length_first_token() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::BOGUS);
        builder.token(RawLanguageKind::EOF, "");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "after");
        builder.finish_node();
        builder.finish_node();
        builder.finish_node();

        let root = builder.finish();
        let subtree = root.children().next().unwrap();
        let zero_length = subtree
            .children_with_tokens()
            .next()
            .and_then(|element| element.into_token())
            .unwrap();
        let first = subtree.first_token().unwrap();

        assert_eq!(first.kind(), RawLanguageKind::EOF);
        assert_eq!(first.text(), "");
        assert_eq!(first, zero_length);
    }

    #[test]
    fn edge_tokens_fall_back_across_multiple_empty_ancestors() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::BOGUS);
        for _ in 0..3 {
            builder.start_node(RawLanguageKind::BOGUS);
        }
        for _ in 0..3 {
            builder.finish_node();
        }
        builder.token(RawLanguageKind::STRING_TOKEN, "first");
        builder.token(RawLanguageKind::STRING_TOKEN, "last");
        for _ in 0..3 {
            builder.start_node(RawLanguageKind::BOGUS);
        }
        for _ in 0..3 {
            builder.finish_node();
        }
        builder.finish_node();
        builder.finish_node();

        let root = builder.finish();
        let subtree = root.children().next().unwrap();

        assert_eq!(subtree.first_token().unwrap().text(), "first");
        assert_eq!(subtree.last_token().unwrap().text(), "last");
    }

    #[test]
    fn edge_tokens_skip_missing_slots() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::CONDITION);
        builder.token(RawLanguageKind::L_PAREN_TOKEN, "(");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.finish_node();
        builder.finish_node();
        builder.token(RawLanguageKind::STRING_TOKEN, "outside");
        builder.finish_node();

        let root = builder.finish();
        let condition = root.children().next().unwrap();

        assert_eq!(condition.first_token().unwrap().text(), "(");
        assert_eq!(condition.last_token().unwrap().text(), "(");
    }

    #[test]
    fn edge_tokens_handle_deep_chains() {
        const DEPTH: usize = 4_096;

        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        for _ in 0..DEPTH {
            builder.start_node(RawLanguageKind::BOGUS);
        }
        builder.token(RawLanguageKind::STRING_TOKEN, "edge");
        for _ in 0..DEPTH {
            builder.finish_node();
        }
        builder.finish_node();

        let mut root = builder.finish();

        assert_eq!(
            root.text_trimmed_range(),
            TextRange::new(0.into(), 4.into())
        );

        // Keep cleanup iterative so this test isolates lookup stack usage.
        let texts = [root.first_token(), root.last_token()].map(|token| {
            let token = token?;
            let text = token.text().to_owned();
            let mut parent = token.parent();
            drop(token);

            while let Some(node) = parent {
                parent = node.parent();
            }

            Some(text)
        });

        while let Some(child) = root.first_child() {
            root = child.detach();
        }

        assert_eq!(texts[0].as_deref(), Some("edge"));
        assert_eq!(texts[1].as_deref(), Some("edge"));
    }

    #[test]
    fn text_trimmed_range_skips_empty_tokens_and_tokenless_nodes() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::BOGUS);
        builder.finish_node();
        builder.token(RawLanguageKind::EOF, "");
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            " a",
            &[TriviaPiece::whitespace(1)],
            &[],
        );
        builder.finish_node();
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "b ",
            &[],
            &[TriviaPiece::whitespace(1)],
        );
        builder.finish_node();
        builder.token_with_trivia(
            RawLanguageKind::EOF,
            "\n",
            &[TriviaPiece::new(TriviaPieceKind::Newline, 1)],
            &[],
        );
        builder.finish_node();

        let root = builder.finish();
        let tokenless = root.children().next().unwrap();

        assert_eq!(
            root.text_trimmed_range(),
            TextRange::new(1.into(), 3.into())
        );
        assert_eq!(tokenless.text_trimmed_range(), TextRange::empty(0.into()));
    }

    #[test]
    fn text_trimmed_range_of_node_with_only_empty_tokens_leaves_the_node() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "before ",
            &[],
            &[TriviaPiece::whitespace(1)],
        );
        builder.start_node(RawLanguageKind::BOGUS);
        builder.token(RawLanguageKind::EOF, "");
        builder.token_with_trivia(
            RawLanguageKind::EOF,
            "  ",
            &[TriviaPiece::whitespace(2)],
            &[],
        );
        builder.finish_node();
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            " after",
            &[TriviaPiece::whitespace(1)],
            &[],
        );
        builder.finish_node();

        let root = builder.finish();
        let empty = root.children().next().unwrap();

        assert_eq!(
            empty.text_range_with_trivia(),
            TextRange::new(7.into(), 9.into())
        );
        assert_eq!(empty.text_trimmed_range(), TextRange::empty(10.into()));
    }

    #[test]
    fn text_trimmed_range_looks_past_child_with_only_trivia() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            "a ",
            &[],
            &[TriviaPiece::whitespace(1)],
        );
        builder.start_node(RawLanguageKind::BOGUS);
        builder.token_with_trivia(
            RawLanguageKind::EOF,
            "  ",
            &[TriviaPiece::whitespace(2)],
            &[],
        );
        builder.finish_node();
        builder.finish_node();

        let root = builder.finish();

        assert_eq!(
            root.text_trimmed_range(),
            TextRange::new(0.into(), 1.into())
        );
    }

    #[test]
    pub fn syntax_text_and_len() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "\n\t let \t\t",
            &[TriviaPiece::whitespace(3)],
            &[TriviaPiece::whitespace(3)],
        );
        builder.finish_node();

        // // Node texts

        let node = builder.finish();
        assert_eq!("\n\t let \t\t", node.text_with_trivia());
        assert_eq!("let", node.text_trimmed());
        assert_eq!("\n\t ", node.first_leading_trivia().unwrap().text());
        assert_eq!(" \t\t", node.last_trailing_trivia().unwrap().text());

        // Token texts

        let token = node.first_token().unwrap();
        assert_eq!("\n\t let \t\t", token.text());
        assert_eq!("let", token.text_trimmed());
        assert_eq!("\n\t ", token.leading_trivia().text());
        assert_eq!(" \t\t", token.trailing_trivia().text());
    }

    #[test]
    pub fn syntax_range() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "\n\t let \t\t",
            &[TriviaPiece::whitespace(3)],
            &[TriviaPiece::whitespace(3)],
        );
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "a ",
            &[TriviaPiece::whitespace(0)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.token_with_trivia(
            RawLanguageKind::EQUAL_TOKEN,
            "\n=\n",
            &[TriviaPiece::whitespace(1)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.token_with_trivia(
            RawLanguageKind::SEMICOLON_TOKEN,
            ";\t\t",
            &[],
            &[TriviaPiece::whitespace(2)],
        );
        builder.finish_node();

        let node = builder.finish();

        // Node Ranges

        assert_eq!(
            TextRange::new(0.into(), 18.into()),
            node.text_range_with_trivia()
        );
        assert_eq!(
            TextRange::new(3.into(), 16.into()),
            node.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(0.into(), 3.into()),
            node.first_leading_trivia().unwrap().text_range()
        );
        assert_eq!(
            TextRange::new(16.into(), 18.into()),
            node.last_trailing_trivia().unwrap().text_range()
        );

        // as NodeOrToken

        let eq_token = node
            .descendants_with_tokens(Direction::Next)
            .find(|x| x.kind() == RawLanguageKind::EQUAL_TOKEN)
            .unwrap();

        assert_eq!(TextRange::new(11.into(), 14.into()), eq_token.text_range());
        assert_eq!(
            TextRange::new(12.into(), 13.into()),
            eq_token.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(11.into(), 12.into()),
            eq_token.leading_trivia().unwrap().text_range()
        );
        assert_eq!(
            TextRange::new(13.into(), 14.into()),
            eq_token.trailing_trivia().unwrap().text_range()
        );

        // as Token

        let eq_token = eq_token.as_token().unwrap();
        assert_eq!(TextRange::new(11.into(), 14.into()), eq_token.text_range());
        assert_eq!(
            TextRange::new(12.into(), 13.into()),
            eq_token.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(11.into(), 12.into()),
            eq_token.leading_trivia().text_range()
        );
        assert_eq!(
            TextRange::new(13.into(), 14.into()),
            eq_token.trailing_trivia().text_range()
        );
    }

    #[test]
    pub fn syntax_trivia_pieces() {
        use crate::*;
        let node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
            builder.token_with_trivia(
                RawLanguageKind::LET_TOKEN,
                "\n\t /**/let \t\t",
                &[
                    TriviaPiece::whitespace(3),
                    TriviaPiece::single_line_comment(4),
                ],
                &[TriviaPiece::whitespace(3)],
            );
        });

        let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
        assert_eq!(2, pieces.len());

        assert_eq!("\n\t ", pieces[0].text());
        assert_eq!(TextSize::from(3), pieces[0].text_len());
        assert_eq!(TextRange::new(0.into(), 3.into()), pieces[0].text_range());
        assert!(pieces[0].is_whitespace());

        assert_eq!("/**/", pieces[1].text());
        assert_eq!(TextSize::from(4), pieces[1].text_len());
        assert_eq!(TextRange::new(3.into(), 7.into()), pieces[1].text_range());
        assert!(pieces[1].is_comments());

        let pieces_rev: Vec<_> = node
            .first_leading_trivia()
            .unwrap()
            .pieces()
            .rev()
            .collect();

        assert_eq!(2, pieces_rev.len());
        assert_eq!("/**/", pieces_rev[0].text());
        assert_eq!("\n\t ", pieces_rev[1].text());
    }

    #[test]
    fn replacing_trivia_clears_descendant_flags() {
        let token = SyntaxToken::<RawLanguage>::new_detached(
            RawLanguageKind::STRING_TOKEN,
            "?value//",
            [TriviaPiece::new(TriviaPieceKind::Skipped, 1)],
            [TriviaPiece::single_line_comment(2)],
        );
        let root =
            SyntaxNode::<RawLanguage>::new_detached(RawLanguageKind::ROOT, [Some(token.into())]);

        assert!(root.has_comments_descendants());
        assert!(root.has_skipped_descendants());

        let token = root.first_token().unwrap();
        let replacement = token.with_leading_trivia([]);
        let root = root
            .replace_child(token.into(), replacement.into())
            .unwrap();
        assert!(root.has_comments_descendants());
        assert!(!root.has_skipped_descendants());

        let token = root.first_token().unwrap();
        let replacement = token.with_trailing_trivia([]);
        let root = root
            .replace_child(token.into(), replacement.into())
            .unwrap();
        assert!(!root.has_comments_descendants());
        assert!(!root.has_skipped_descendants());
    }

    #[test]
    fn single_space_and_newline_trivia_helpers_replace_trivia() {
        let token = SyntaxToken::<RawLanguage>::new_detached(
            RawLanguageKind::LET_TOKEN,
            "/*a*/let//b",
            [TriviaPiece::multi_line_comment(5)],
            [TriviaPiece::single_line_comment(3)],
        );

        assert_eq!(token.with_leading_space().text(), " let//b");
        assert_eq!(token.with_trailing_space().text(), "/*a*/let ");
        assert_eq!(token.with_surrounding_spaces().text(), " let ");
        assert_eq!(token.with_leading_newline().text(), "\nlet//b");

        let leading: Vec<_> = token
            .with_leading_newline()
            .leading_trivia()
            .pieces()
            .collect();
        assert_eq!(leading.len(), 1);
        assert!(leading[0].is_newline());

        let trailing: Vec<_> = token
            .with_trailing_space()
            .trailing_trivia()
            .pieces()
            .collect();
        assert_eq!(trailing.len(), 1);
        assert!(trailing[0].is_whitespace());
    }
}
