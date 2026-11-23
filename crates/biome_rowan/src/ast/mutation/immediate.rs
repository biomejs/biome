use std::ops;

use super::trivia::{self, TokenlessSource};
use crate::{AstNode, AstNodeList, AstSeparatedList, SyntaxElement, SyntaxToken};

pub trait AstNodeExt: AstNode {
    /// Return a new version of this node with the node `prev_node` replaced with `next_node`
    ///
    /// `prev_node` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_node` is not a descendant of this node
    fn replace_node_discard_trivia<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized;

    /// Return a new version of this node with the node `prev_node` replaced with `next_node`,
    /// transferring the leading and trailing trivia of `prev_node` to `next_node`
    ///
    /// `prev_node` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_node` is not a descendant of this node
    fn replace_node<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized;

    /// Return a new version of this node with the token `prev_token` replaced with `next_token`
    ///
    /// `prev_token` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_token` is not a descendant of this node
    fn replace_token_discard_trivia(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized;

    /// Return a new version of this node with the token `prev_token` replaced with `next_token`,
    /// transferring the leading and trailing trivia of `prev_token` to `next_token`
    ///
    /// `prev_token` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_token` is not a descendant of this node
    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized;

    fn detach(self) -> Self;
}

impl<T> AstNodeExt for T
where
    T: AstNode,
{
    fn replace_node_discard_trivia<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized,
    {
        Some(Self::unwrap_cast(self.into_syntax().replace_child(
            prev_node.into_syntax().into(),
            next_node.into_syntax().into(),
        )?))
    }

    fn replace_node<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized,
    {
        let prev_element = SyntaxElement::Node(prev_node.into_syntax());
        let next_element = trivia::copy(
            &prev_element,
            SyntaxElement::Node(next_node.into_syntax()),
            TokenlessSource::PreserveDestination,
        )?;

        Some(Self::unwrap_cast(
            self.into_syntax()
                .replace_child(prev_element, next_element)?,
        ))
    }

    fn replace_token_discard_trivia(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::unwrap_cast(
            self.into_syntax()
                .replace_child(prev_token.into(), next_token.into())?,
        ))
    }

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let prev_element = SyntaxElement::Token(prev_token);
        let next_element = trivia::copy(
            &prev_element,
            SyntaxElement::Token(next_token),
            TokenlessSource::PreserveDestination,
        )?;

        Some(Self::unwrap_cast(
            self.into_syntax()
                .replace_child(prev_element, next_element)?,
        ))
    }

    /// Return a new version of this node detached from its parent node
    fn detach(self) -> Self {
        Self::unwrap_cast(self.into_syntax().detach())
    }
}

pub trait AstNodeListExt: AstNodeList {
    /// Replace a range of the children of this list with the content of an iterator
    fn splice<R, I>(self, range: R, replace_with: I) -> Self
    where
        Self: AstNode<Language = <Self as AstNodeList>::Language> + Sized,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = Self::Node>;
}

impl<T> AstNodeListExt for T
where
    T: AstNodeList,
{
    fn splice<R, I>(self, range: R, replace_with: I) -> Self
    where
        Self: AstNode<Language = <Self as AstNodeList>::Language> + Sized,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = Self::Node>,
    {
        Self::unwrap_cast(
            self.into_syntax_list().into_node().splice_slots(
                range,
                replace_with
                    .into_iter()
                    .map(|node| Some(node.into_syntax().into())),
            ),
        )
    }
}

pub trait AstSeparatedListExt: AstSeparatedList {
    /// Replace a range of the children of this list with the content of an iterator
    ///
    /// Both the range and iterator work on pairs of node and separator token
    fn splice<R, I>(self, range: R, replace_with: I) -> Self
    where
        Self: AstNode<Language = <Self as AstSeparatedList>::Language> + Sized,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<
            Item = (
                Self::Node,
                Option<SyntaxToken<<Self as AstSeparatedList>::Language>>,
            ),
        >;
}

impl<T> AstSeparatedListExt for T
where
    T: AstSeparatedList,
{
    fn splice<R, I>(self, range: R, replace_with: I) -> Self
    where
        Self: AstNode<Language = <Self as AstSeparatedList>::Language> + Sized,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<
            Item = (
                Self::Node,
                Option<SyntaxToken<<Self as AstSeparatedList>::Language>>,
            ),
        >,
    {
        let start_bound = match range.start_bound() {
            ops::Bound::Included(index) => ops::Bound::Included(*index * 2),
            ops::Bound::Excluded(index) => ops::Bound::Excluded(*index * 2),
            ops::Bound::Unbounded => ops::Bound::Unbounded,
        };
        let end_bound = match range.end_bound() {
            ops::Bound::Included(index) => ops::Bound::Included(*index * 2),
            ops::Bound::Excluded(index) => ops::Bound::Excluded(*index * 2),
            ops::Bound::Unbounded => ops::Bound::Unbounded,
        };

        Self::unwrap_cast(self.into_syntax_list().into_node().splice_slots(
            (start_bound, end_bound),
            replace_with.into_iter().flat_map(|(node, separator)| {
                [
                    Some(node.into_syntax().into()),
                    separator.map(|token| token.into()),
                ]
            }),
        ))
    }
}
