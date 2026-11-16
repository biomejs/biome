use crate::cursor::{NodeData, SyntaxElement, SyntaxNode, SyntaxTrivia};
use crate::green::GreenElementRef;
use crate::{Direction, GreenToken, GreenTokenData, RawSyntaxKind, TokenText, green};
use biome_text_size::{TextRange, TextSize};
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;
use std::rc::Rc;
use std::{fmt, iter};

use super::{GreenElement, NodeKind, WeakGreenElement};

#[derive(Clone, Debug)]
pub(crate) struct SyntaxToken {
    ptr: Rc<NodeData>,
}

impl SyntaxToken {
    pub(super) fn new(
        green: &GreenTokenData,
        parent: SyntaxNode,
        index: u32,
        offset: TextSize,
    ) -> Self {
        Self {
            ptr: NodeData::new(
                NodeKind::Child {
                    green: WeakGreenElement::new(GreenElementRef::Token(green)),
                    parent: parent.ptr,
                },
                index,
                offset,
            ),
        }
    }

    pub(crate) fn new_detached(green: GreenToken) -> Self {
        Self {
            ptr: NodeData::new(
                NodeKind::Root {
                    green: GreenElement::Token(green),
                },
                0,
                TextSize::from(0),
            ),
        }
    }

    #[inline]
    pub(crate) fn green(&self) -> &GreenTokenData {
        match self.data().green().as_token() {
            Some(token) => token,
            None => {
                panic!(
                    "corrupted tree: a node thinks it is a token: {:?}",
                    self.data().green().as_node().unwrap().to_string()
                );
            }
        }
    }

    pub(crate) fn key(&self) -> (NonNull<()>, TextSize) {
        self.data().key()
    }

    #[inline]
    pub(super) fn data(&self) -> &NodeData {
        self.ptr.as_ref()
    }

    #[inline]
    pub(super) fn into_green(self) -> green::GreenElement {
        self.ptr.into_green()
    }

    #[inline]
    pub fn kind(&self) -> RawSyntaxKind {
        self.data().kind()
    }

    #[inline]
    pub fn text_range(&self) -> TextRange {
        self.data().text_range()
    }

    #[inline]
    pub fn text_trimmed_range(&self) -> TextRange {
        let green_token = self.green();
        let leading_len = green_token.leading_trivia().text_len();
        let trailing_len = green_token.trailing_trivia().text_len();

        let range = self.text_range();
        TextRange::new(range.start() + leading_len, range.end() - trailing_len)
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.data().slot() as usize
    }

    #[inline]
    pub fn text(&self) -> &str {
        self.green().text()
    }

    #[inline]
    pub fn token_text(&self) -> TokenText {
        TokenText::new(self.green().to_owned())
    }

    #[inline]
    pub fn token_text_trimmed(&self) -> TokenText {
        let green = self.green().to_owned();
        let mut range = self.text_trimmed_range();
        range -= self.data().offset;
        TokenText::with_range(green, range)
    }

    #[inline]
    pub fn text_trimmed(&self) -> &str {
        self.green().text_trimmed()
    }

    #[inline]
    pub fn parent(&self) -> Option<SyntaxNode> {
        self.data().parent_node()
    }

    #[inline]
    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> + use<> {
        std::iter::successors(self.parent(), SyntaxNode::parent)
    }

    #[inline]
    pub fn tree_top(&self) -> Option<SyntaxNode> {
        self.ancestors().last()
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data().next_sibling_or_token()
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.data().prev_sibling_or_token()
    }

    #[inline]
    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement> + use<> {
        let next = move |el: &SyntaxElement| match direction {
            Direction::Next => el.next_sibling_or_token(),
            Direction::Prev => el.prev_sibling_or_token(),
        };

        let me: SyntaxElement = self.clone().into();

        iter::successors(next(&me), next)
    }

    pub fn next_token(&self) -> Option<Self> {
        SyntaxElement::Token(self.clone()).next_token()
    }

    pub fn prev_token(&self) -> Option<Self> {
        SyntaxElement::Token(self.clone()).prev_token()
    }

    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn detach(self) -> Self {
        Self {
            ptr: self.ptr.detach(),
        }
    }

    #[inline]
    pub fn leading_trivia(&self) -> SyntaxTrivia {
        SyntaxTrivia::leading(self.clone())
    }

    #[inline]
    pub fn trailing_trivia(&self) -> SyntaxTrivia {
        SyntaxTrivia::trailing(self.clone())
    }
}

// Identity semantics for hash & eq
impl PartialEq for SyntaxToken {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data().key() == other.data().key()
    }
}

impl Eq for SyntaxToken {}

impl Hash for SyntaxToken {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data().key().hash(state);
    }
}

impl fmt::Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}
