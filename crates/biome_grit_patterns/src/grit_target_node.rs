use crate::util::TextRangeGritExt;
use biome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::{SyntaxKind, SyntaxNodeText, TextRange};
use grit_util::{AstCursor, AstNode as GritAstNode, ByteRange, CodeRange};
use std::{borrow::Cow, str::Utf8Error};

/// Generates the `GritTargetNode`, `GritTargetToken`, and
/// `GritTargetSyntaxKind` enums.
///
/// These enums can represent nodes, tokens and kinds for all the languages we
/// support running Grit queries on.
///
/// We intentionally use enums for these types, rather than using generics for
/// specifying specific types:
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
    ($([$lang_node:ident, $lang_token:ident, $lang_kind:ident]),+) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum GritTargetNode {
            $($lang_node($lang_node)),+
        }

        $(impl From<$lang_node> for GritTargetNode {
            fn from(value: $lang_node) -> Self {
                Self::$lang_node(value)
            }
        })+

        impl GritTargetNode {
            fn first_child(&self) -> Option<Self> {
                match self {
                    $(Self::$lang_node(node) => node.first_child().map(Into::into)),+
                }
            }

            fn first_token(&self) -> Option<GritTargetToken> {
                match self {
                    $(Self::$lang_node(node) => node.first_token().map(Into::into)),+
                }
            }

            pub fn kind(&self) -> GritTargetSyntaxKind {
                match self {
                    $(Self::$lang_node(node) => node.kind().into()),+
                }
            }

            pub fn text(&self) -> SyntaxNodeText {
                match self {
                    $(Self::$lang_node(node) => node.text()),+
                }
            }

            pub fn text_trimmed(&self) -> SyntaxNodeText {
                match self {
                    $(Self::$lang_node(node) => node.text_trimmed()),+
                }
            }

            pub fn text_trimmed_range(&self) -> TextRange {
                match self {
                    $(Self::$lang_node(node) => node.text_trimmed_range()),+
                }
            }

            pub fn start_byte(&self) -> u32 {
                self.text_trimmed_range().start().into()
            }

            pub fn end_byte(&self) -> u32 {
                self.text_trimmed_range().end().into()
            }
        }

        impl GritAstNode for GritTargetNode {
            fn ancestors(&self) -> impl Iterator<Item = Self> {
                AncestorIterator::new(self)
            }

            fn children(&self) -> impl Iterator<Item = Self> {
                ChildrenIterator::new(self)
            }

            fn parent(&self) -> Option<Self> {
                match self {
                    $(Self::$lang_node(node) => node.parent().map(Into::into)),+
                }
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

            fn next_sibling(&self) -> Option<Self> {
                match self {
                    $(Self::$lang_node(node) => node.next_sibling().map(Into::into)),+
                }
            }

            fn previous_sibling(&self) -> Option<Self> {
                match self {
                    $(Self::$lang_node(node) => node.prev_sibling().map(Into::into)),+
                }
            }

            fn text(&self) -> Result<Cow<str>, Utf8Error> {
                Ok(Cow::Owned(self.text_trimmed().to_string()))
            }

            fn byte_range(&self) -> ByteRange {
                self.text_trimmed_range().to_byte_range()
            }

            fn code_range(&self) -> CodeRange {
                let range = self.text_trimmed_range();
                CodeRange {
                    start: range.start().into(),
                    end: range.end().into(),
                    // Code ranges contain an address so they can quickly check whether
                    // a particular binding belongs to a given range or not.
                    address: self
                        .first_token()
                        .map(|token| token.text().as_ptr() as usize)
                        .unwrap_or_default(),
                }
            }

            fn walk(&self) -> impl AstCursor<Node = Self> {
                GritTargetNodeCursor::new(self)
            }
        }

        #[derive(Clone, Debug)]
        pub enum GritTargetToken {
            $($lang_token($lang_token)),+
        }

        $(impl From<$lang_token> for GritTargetToken {
            fn from(value: $lang_token) -> Self {
                Self::$lang_token(value)
            }
        })+

        impl GritTargetToken {
            fn text(&self) -> &str {
                match self {
                    $(Self::$lang_token(token) => token.text()),+
                }
            }
        }

        #[derive(Clone, Debug, PartialEq)]
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
        }
    };
}

generate_target_node! {
    [JsSyntaxNode, JsSyntaxToken, JsSyntaxKind]
}

impl GritTargetSyntaxKind {
    pub fn as_js_kind(&self) -> Option<JsSyntaxKind> {
        match self {
            Self::JsSyntaxKind(kind) => Some(*kind),
        }
    }
}

#[derive(Clone)]
pub struct AncestorIterator {
    node: Option<GritTargetNode>,
}

impl AncestorIterator {
    fn new(node: &GritTargetNode) -> Self {
        Self {
            node: Some(node.clone()),
        }
    }
}

impl Iterator for AncestorIterator {
    type Item = GritTargetNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.as_ref().cloned()?;
        self.node = node.parent();
        Some(node)
    }
}

pub struct ChildrenIterator {
    cursor: Option<GritTargetNodeCursor>,
}

impl ChildrenIterator {
    fn new(node: &GritTargetNode) -> Self {
        let mut cursor = GritTargetNodeCursor::new(node);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
        }
    }
}

impl Iterator for ChildrenIterator {
    type Item = GritTargetNode;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if !c.goto_next_sibling() {
            self.cursor = None;
        }
        Some(node)
    }
}

#[derive(Clone)]
struct GritTargetNodeCursor {
    node: GritTargetNode,
}

impl GritTargetNodeCursor {
    fn new(node: &GritTargetNode) -> Self {
        Self { node: node.clone() }
    }
}

impl AstCursor for GritTargetNodeCursor {
    type Node = GritTargetNode;

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
        match self.node.parent() {
            Some(parent) => {
                self.node = parent;
                true
            }
            None => false,
        }
    }

    fn goto_next_sibling(&mut self) -> bool {
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
