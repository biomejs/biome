use crate::grit_tree::GritTargetTree;
use crate::util::TextRangeGritExt;
use biome_css_syntax::{CssSyntaxKind, CssSyntaxNode, CssSyntaxToken};
use biome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::{NodeOrToken, SyntaxKind, SyntaxSlot, TextRange};
use grit_util::{error::GritResult, AstCursor, AstNode as GritAstNode, ByteRange, CodeRange};
use std::{borrow::Cow, fmt::Debug, ops::Deref};

use NodeOrToken::*;

/// Generates the `GritTargetNode` and `GritTargetSyntaxKind` enums.
///
/// Note that `GritTargetNode` can represent both nodes and tokens. While Biome
/// uses different terminology for the two, Grit treats them one and the
/// same. It does sometimes refer to "named nodes" when specifically talking
/// about nodes and not tokens, since tokens are not named in the grammar.
///
/// These enums can represent nodes, tokens and kinds for all the languages we
/// support running Grit queries on.
///
/// We intentionally use enums for these types, rather than using generics for
/// specifying specific language-specific-types:
/// - If we used generics instead, those would infest all code using these
///   types, and we would end up with an explosion of generics all over the Grit
///   runtime.
/// - Using generics wouldn't only make the code itself a lot more complex, it
///   would inflate compile times and binary size as well. It's hard to say how
///   much this would matter, but there will be quite some code in the Grit
///   runtime, and each supported language would effectively require its own
///   binary instance of the entire runtime.
/// - Theoretically, this may enable us to run queries on mixed-language trees
///   in the future. Even though GritQL does not currently have syntax support
///   for this, it may allow us to one day query CSS rules inside a JS template
///   literal, for instance.
macro_rules! generate_target_node {
    ($([$lang:ident, $lang_node:ident, $lang_token:ident, $lang_kind:ident]),+) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum GritTargetLanguageNode {
            $($lang(NodeOrToken<$lang_node, $lang_token>)),+
        }

        $(impl From<$lang_node> for GritTargetLanguageNode {
            fn from(value: $lang_node) -> Self {
                Self::$lang(Node(value))
            }
        })+

        $(impl From<$lang_token> for GritTargetLanguageNode {
            fn from(value: $lang_token) -> Self {
                Self::$lang(Token(value))
            }
        })+

        $(impl From<NodeOrToken<$lang_node, $lang_token>> for GritTargetLanguageNode {
            fn from(value: NodeOrToken<$lang_node, $lang_token>) -> Self {
                Self::$lang(value)
            }
        })+

        impl GritTargetLanguageNode {
            pub fn first_child(&self) -> Option<Self> {
                match self {
                    $(Self::$lang(Node(node)) => node.first_child().map(Into::into)),+,
                    _ => None
                }
            }

            pub fn has_children(&self) -> bool {
                match self {
                    $(Self::$lang(Node(node)) => node.first_child().is_some()),+,
                    _ => false
                }
            }

            pub fn index(&self) -> u32 {
                match self {
                    $(Self::$lang(Node(node)) => node.index() as u32),+,
                    $(Self::$lang(Token(token)) => token.index() as u32),+
                }
            }

            #[inline]
            pub fn is_token(&self) -> bool {
                match self {
                    $(Self::$lang(Node(_)) => false),+,
                    $(Self::$lang(Token(_)) => true),+
                }
            }

            #[inline]
            pub fn kind(&self) -> GritTargetSyntaxKind {
                match self {
                    $(Self::$lang(Node(node)) => node.kind().into()),+,
                    $(Self::$lang(Token(token)) => token.kind().into()),+
                }
            }

            pub fn next_sibling(&self) -> Option<Self> {
                match self {
                    $(Self::$lang(Node(node)) => node.next_sibling_or_token().map(Into::into)),+,
                    $(Self::$lang(Token(token)) => token.next_sibling_or_token().map(Into::into)),+
                }
            }

            pub fn owned_text(&self) -> Cow<str> {
                match self {
                    $(Self::$lang(Node(node)) => Cow::Owned(node.text_with_trivia().to_string())),+,
                    $(Self::$lang(Token(token)) => Cow::Borrowed(token.text())),+
                }
            }

            pub fn parent(&self) -> Option<Self> {
                match self {
                    $(Self::$lang(Node(node)) => node.parent().map(Into::into)),+,
                    $(Self::$lang(Token(token)) => token.parent().map(Into::into)),+
                }
            }

            pub fn previous_sibling(&self) -> Option<Self> {
                match self {
                    $(Self::$lang(Node(node)) => node.prev_sibling_or_token().map(Into::into)),+,
                    $(Self::$lang(Token(token)) => token.prev_sibling_or_token().map(Into::into)),+
                }
            }

            pub fn slots<'a>(&self, tree: &'a GritTargetTree) -> Option<Vec<GritSyntaxSlot<'a>>> {
                match self {
                    $(Self::$lang(Node(node)) => Some(
                        node.slots()
                            .map(|slot| match slot {
                                SyntaxSlot::Node(node) => GritSyntaxSlot::Node(GritTargetNode::new(node.into(), tree)),
                                SyntaxSlot::Token(token) => GritSyntaxSlot::Node(GritTargetNode::new(token.into(), tree)),
                                SyntaxSlot::Empty { index } => GritSyntaxSlot::Empty { index }
                            })
                            .collect()
                    )),+,
                    $(Self::$lang(Token(_token)) => None),+
                }
            }

            pub fn text_range_with_trivia(&self) -> TextRange {
                match self {
                    $(Self::$lang(Node(node)) => node.text_range_with_trivia()),+,
                    $(Self::$lang(Token(token)) => token.text_range()),+
                }
            }

            #[inline]
            pub fn text_trimmed_range(&self) -> TextRange {
                match self {
                    $(Self::$lang(Node(node)) => node.text_trimmed_range()),+,
                    $(Self::$lang(Token(token)) => token.text_trimmed_range()),+
                }
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum GritTargetSyntaxKind {
            $($lang_kind($lang_kind)),+
        }

        $(impl From<$lang_kind> for GritTargetSyntaxKind {
            fn from(value: $lang_kind) -> Self {
                Self::$lang_kind(value)
            }
        })+

        impl GritTargetSyntaxKind {
            pub fn is_bogus(&self) -> bool {
                match self {
                    $(Self::$lang_kind(kind) => kind.is_bogus()),+
                }
            }

            pub fn is_list(&self) -> bool {
                match self {
                    $(Self::$lang_kind(kind) => kind.is_list()),+
                }
            }
        }
    };
}

generate_target_node! {
    [CssLanguage, CssSyntaxNode, CssSyntaxToken, CssSyntaxKind],
    [JsLanguage, JsSyntaxNode, JsSyntaxToken, JsSyntaxKind]
}

#[derive(Clone, PartialEq)]
pub struct GritTargetNode<'a> {
    node: GritTargetLanguageNode,
    tree: &'a GritTargetTree,
}

impl<'a> GritTargetNode<'a> {
    pub fn new(node: GritTargetLanguageNode, tree: &'a GritTargetTree) -> Self {
        Self { node, tree }
    }

    pub fn child_by_slot_index(&self, index: u32) -> Option<Self> {
        self.slots()
            .and_then(|mut slots| slots.nth(index as usize))
            .and_then(|slot| match slot {
                GritSyntaxSlot::Node(node) => Some(node),
                GritSyntaxSlot::Empty { .. } => None,
            })
    }

    pub fn descendants(&'a self) -> impl Iterator<Item = Self> {
        DescendantsIterator::new(self)
    }

    pub fn named_children(&self) -> impl Iterator<Item = Self> + Clone {
        NamedChildrenIterator::new(self)
    }

    #[inline]
    pub fn end_byte(&self) -> u32 {
        self.text_trimmed_range().end().into()
    }

    pub fn first_child(&self) -> Option<Self> {
        self.node.first_child().map(|node| Self {
            node,
            tree: self.tree,
        })
    }

    #[inline]
    pub fn is_bogus(&self) -> bool {
        self.kind().is_bogus()
    }

    #[inline]
    pub fn is_list(&self) -> bool {
        self.kind().is_list()
    }

    #[inline]
    pub fn slots(&self) -> Option<impl Iterator<Item = GritSyntaxSlot<'a>>> {
        SlotIterator::new(self)
    }

    #[inline]
    pub fn source(&self) -> &'a str {
        self.tree.text()
    }

    #[inline]
    pub fn start_byte(&self) -> u32 {
        self.text_trimmed_range().start().into()
    }

    pub fn text(&self) -> &'a str {
        let trimmed_range = self.text_trimmed_range();
        &self.source()[trimmed_range.start().into()..trimmed_range.end().into()]
    }

    /// Matches the `kind` of this node, and those of all its children, with
    /// those of another node.
    ///
    /// This is a relatively cheap way to discover whether two parsed snippets
    /// are identical when they were parsed from the same source string, but
    /// with different contexts. In that use case, we already know the snippet
    /// is essentially the same, but we only detect a meaningful difference in
    /// context by looking for a variance in node kinds.
    pub fn matches_kinds_recursively_with(&self, other: &Self) -> bool {
        let mut cursor_a = self.walk();
        let mut cursor_b = other.walk();

        // Are we navigating back up? If so, we shouldn't try to visit any
        // children until we've visited another sibling, or we'd run in circles.
        let mut up = false;

        loop {
            if cursor_a.node().kind() != cursor_b.node().kind() {
                break false;
            }

            if !up && cursor_a.goto_first_child() {
                if !cursor_b.goto_first_child() {
                    break false;
                }
            } else if cursor_a.goto_next_sibling() {
                if cursor_b.goto_first_child() || !cursor_b.goto_next_sibling() {
                    break false;
                }

                up = false;
            } else if cursor_a.goto_parent() {
                if (!up && cursor_b.goto_first_child())
                    || cursor_b.goto_next_sibling()
                    || !cursor_b.goto_parent()
                {
                    break false;
                }

                up = true;
            } else {
                break true;
            }
        }
    }
}

impl Debug for GritTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GritTargetNode")
            .field("node", &self.node)
            .finish()
    }
}

impl Deref for GritTargetNode<'_> {
    type Target = GritTargetLanguageNode;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl GritAstNode for GritTargetNode<'_> {
    fn ancestors(&self) -> impl Iterator<Item = Self> {
        AncestorIterator::new(self.clone())
    }

    fn byte_range(&self) -> ByteRange {
        self.text_trimmed_range().to_byte_range()
    }

    fn code_range(&self) -> CodeRange {
        self.text_trimmed_range().to_code_range(self.text())
    }

    #[expect(refining_impl_trait)]
    fn children(&self) -> impl Iterator<Item = Self> + Clone {
        ChildrenIterator::new(self)
    }

    fn parent(&self) -> Option<Self> {
        self.node.parent().map(|node| Self {
            node,
            tree: self.tree,
        })
    }

    fn next_sibling(&self) -> Option<Self> {
        self.node.next_sibling().map(|node| Self {
            node,
            tree: self.tree,
        })
    }

    fn previous_sibling(&self) -> Option<Self> {
        self.node.previous_sibling().map(|node| Self {
            node,
            tree: self.tree,
        })
    }

    fn next_named_node(&self) -> Option<Self> {
        let mut current_node = Cow::Borrowed(self);
        loop {
            if let Some(sibling) = current_node.next_sibling() {
                return Some(sibling);
            }
            current_node = Cow::Owned(current_node.parent()?);
        }
    }

    fn previous_named_node(&self) -> Option<Self> {
        let mut current_node = Cow::Borrowed(self);
        loop {
            if let Some(sibling) = current_node.previous_sibling() {
                return Some(sibling);
            }
            current_node = Cow::Owned(current_node.parent()?);
        }
    }

    fn text(&self) -> GritResult<Cow<str>> {
        Ok(Cow::Borrowed(self.text()))
    }

    fn walk(&self) -> impl AstCursor<Node = Self> {
        GritTargetNodeCursor::new(self)
    }
}

impl GritTargetSyntaxKind {
    pub fn as_css_kind(&self) -> Option<CssSyntaxKind> {
        match self {
            Self::CssSyntaxKind(kind) => Some(*kind),
            _ => None,
        }
    }

    pub fn as_js_kind(&self) -> Option<JsSyntaxKind> {
        match self {
            Self::JsSyntaxKind(kind) => Some(*kind),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GritSyntaxSlot<'a> {
    /// Slot that stores a node child
    Node(GritTargetNode<'a>),
    /// Slot that marks that the child in this position isn't present in the source code.
    Empty { index: u32 },
}

impl GritSyntaxSlot<'_> {
    pub fn contains_list(&self) -> bool {
        match self {
            GritSyntaxSlot::Node(node) => node.kind().is_list(),
            GritSyntaxSlot::Empty { .. } => false,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            GritSyntaxSlot::Node(node) => node.index(),
            GritSyntaxSlot::Empty { index } => *index,
        }
    }
}

#[derive(Clone)]
pub struct AncestorIterator<'a> {
    node: Option<GritTargetNode<'a>>,
}

impl<'a> AncestorIterator<'a> {
    fn new(node: GritTargetNode<'a>) -> Self {
        Self { node: Some(node) }
    }
}

impl<'a> Iterator for AncestorIterator<'a> {
    type Item = GritTargetNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.clone()?;
        self.node = node.parent();
        Some(node)
    }
}

#[derive(Clone, Debug)]
pub struct ChildrenIterator<'a> {
    cursor: Option<GritTargetNodeCursor<'a>>,
}

impl<'a> ChildrenIterator<'a> {
    fn new(node: &GritTargetNode<'a>) -> Self {
        let mut cursor = GritTargetNodeCursor::new(node);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
        }
    }
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = GritTargetNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if !c.goto_next_sibling() {
            self.cursor = None;
        }
        Some(node)
    }
}

#[derive(Clone, Debug)]
pub struct DescendantsIterator<'a> {
    cursor: Option<GritTargetNodeCursor<'a>>,
}

impl<'a> DescendantsIterator<'a> {
    fn new(node: &GritTargetNode<'a>) -> Self {
        let mut cursor = GritTargetNodeCursor::new(node);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
        }
    }
}

impl<'a> Iterator for DescendantsIterator<'a> {
    type Item = GritTargetNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if c.goto_first_child() || c.goto_next_sibling() {
            // Good.
        } else {
            loop {
                if !c.goto_parent() {
                    self.cursor = None;
                    break;
                }
                if c.goto_next_sibling() {
                    break;
                }
            }
        }
        Some(node)
    }
}

#[derive(Clone, Debug)]
pub struct NamedChildrenIterator<'a> {
    cursor: Option<GritTargetNodeCursor<'a>>,
}

impl<'a> NamedChildrenIterator<'a> {
    fn new(node: &GritTargetNode<'a>) -> Self {
        let mut cursor = GritTargetNodeCursor::new(node);
        let mut cursor = cursor.goto_first_child().then_some(cursor);
        if let Some(c) = cursor.as_mut() {
            while c.is_at_token() {
                if !c.goto_next_sibling() {
                    cursor = None;
                    break;
                }
            }
        }
        Self { cursor }
    }
}

impl<'a> Iterator for NamedChildrenIterator<'a> {
    type Item = GritTargetNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if c.goto_next_sibling() {
            while c.is_at_token() {
                if !c.goto_next_sibling() {
                    self.cursor = None;
                    break;
                }
            }
        } else {
            self.cursor = None;
        }
        Some(node)
    }
}

#[derive(Clone, Debug)]
pub struct SlotIterator<'a> {
    // We collect slots in a vector to avoid type issues that would result from
    // language-specific slot iterators. The vector is reversed so we can
    // cheaply pop elements.
    slots: Vec<GritSyntaxSlot<'a>>,
}

impl<'a> SlotIterator<'a> {
    fn new(node: &GritTargetNode<'a>) -> Option<Self> {
        node.node.slots(node.tree).map(|mut slots| {
            slots.reverse();
            Self { slots }
        })
    }
}

impl<'a> Iterator for SlotIterator<'a> {
    type Item = GritSyntaxSlot<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slots.pop()
    }
}

#[derive(Clone, Debug)]
struct GritTargetNodeCursor<'a> {
    node: GritTargetNode<'a>,
    root: GritTargetNode<'a>,
}

impl<'a> GritTargetNodeCursor<'a> {
    fn new(node: &GritTargetNode<'a>) -> Self {
        Self {
            node: node.clone(),
            root: node.clone(),
        }
    }

    fn is_at_token(&self) -> bool {
        self.node.is_token()
    }
}

impl<'a> AstCursor for GritTargetNodeCursor<'a> {
    type Node = GritTargetNode<'a>;

    fn goto_first_child(&mut self) -> bool {
        match self.node.first_child() {
            Some(child) => {
                self.node = child;
                true
            }
            None => false,
        }
    }

    fn goto_parent(&mut self) -> bool {
        if self.node == self.root {
            return false;
        }
        match self.node.parent() {
            Some(parent) => {
                self.node = parent;
                true
            }
            None => false,
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
        if self.node == self.root {
            return false;
        }
        match self.node.next_sibling() {
            Some(sibling) => {
                self.node = sibling;
                true
            }
            None => false,
        }
    }

    fn node(&self) -> Self::Node {
        self.node.clone()
    }
}
