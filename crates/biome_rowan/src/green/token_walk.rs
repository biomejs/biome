//! Borrowed source-order walks over green tokens.
//!
//! The walks share depth-first slot traversal and absolute-offset tracking
//! without constructing red syntax elements.

use std::{iter::FusedIterator, slice};

use crate::{TextRange, TextSize};

use super::{GreenNodeRef, GreenTokenRef, Slot};

/// A borrowed green token and the absolute start of its full text.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GreenTokenAtOffset<'a> {
    pub(crate) green: GreenTokenRef<'a>,
    pub(crate) offset: TextSize,
}

pub(crate) struct GreenTokens<'a> {
    walk: GreenTokenWalk<'a, AllTokens>,
}

impl<'a> GreenTokens<'a> {
    #[inline]
    pub(crate) fn new(node: GreenNodeRef<'a>, offset: TextSize) -> Self {
        Self {
            walk: GreenTokenWalk::new(node, offset, AllTokens),
        }
    }
}

impl<'a> Iterator for GreenTokens<'a> {
    type Item = GreenTokenAtOffset<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.walk.next()
    }
}

impl FusedIterator for GreenTokens<'_> {}

pub(crate) struct GreenTokensWithComments<'a> {
    walk: GreenTokenWalk<'a, TokensWithComments>,
}

impl<'a> GreenTokensWithComments<'a> {
    #[inline]
    pub(crate) fn new(node: GreenNodeRef<'a>, offset: TextSize) -> Self {
        Self {
            walk: GreenTokenWalk::new(node, offset, TokensWithComments),
        }
    }
}

impl<'a> Iterator for GreenTokensWithComments<'a> {
    type Item = GreenTokenAtOffset<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.walk.next()
    }
}

impl FusedIterator for GreenTokensWithComments<'_> {}

pub(crate) struct GreenTokensForRange<'a> {
    walk: GreenTokenWalk<'a, TokensForRange>,
}

impl<'a> GreenTokensForRange<'a> {
    /// `range` must lie inside the absolute text range of `node`.
    #[inline]
    pub(crate) fn new(node: GreenNodeRef<'a>, offset: TextSize, range: TextRange) -> Self {
        Self {
            walk: GreenTokenWalk::new(node, offset, TokensForRange { range }),
        }
    }
}

impl<'a> Iterator for GreenTokensForRange<'a> {
    type Item = GreenTokenAtOffset<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.walk.next()
    }
}

impl FusedIterator for GreenTokensForRange<'_> {}

struct GreenTokenWalk<'a, P> {
    pending_children: Vec<(slice::Iter<'a, Slot>, TextSize)>,
    policy: P,
}

impl<'a, P> GreenTokenWalk<'a, P>
where
    P: GreenTokenWalkPolicy<'a>,
{
    #[inline]
    fn new(node: GreenNodeRef<'a>, offset: TextSize, policy: P) -> Self {
        let pending_children = policy
            .slots(node, offset)
            .map_or_else(Vec::new, |slots| vec![(slots.iter(), offset)]);

        Self {
            pending_children,
            policy,
        }
    }
}

impl<'a, P> Iterator for GreenTokenWalk<'a, P>
where
    P: GreenTokenWalkPolicy<'a>,
{
    type Item = GreenTokenAtOffset<'a>;

    // Inlining removes policy checks that are constant for each semantic walk.
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = {
                let (slots, node_offset) = self.pending_children.last_mut()?;
                slots.next().map(|slot| (slot, *node_offset))
            };

            let Some((slot, node_offset)) = next else {
                self.pending_children.pop();
                continue;
            };

            let offset = node_offset + slot.rel_offset();
            if self.policy.stops_before(offset) {
                self.pending_children.clear();
                return None;
            }

            match slot {
                Slot::Node { node, .. } => {
                    if let Some(slots) = self.policy.slots(node.as_ref(), offset) {
                        self.pending_children.push((slots.iter(), offset));
                    }
                }
                Slot::Token { token, .. } => {
                    let green = token.as_ref();
                    if self.policy.includes_token(green) {
                        return Some(GreenTokenAtOffset { green, offset });
                    }
                }
                Slot::Empty { .. } => {}
            }
        }
    }
}

impl<'a, P> FusedIterator for GreenTokenWalk<'a, P> where P: GreenTokenWalkPolicy<'a> {}

trait GreenTokenWalkPolicy<'a> {
    fn slots(&self, node: GreenNodeRef<'a>, offset: TextSize) -> Option<&'a [Slot]>;

    #[inline]
    fn stops_before(&self, _offset: TextSize) -> bool {
        false
    }

    #[inline]
    fn includes_token(&self, _token: GreenTokenRef<'a>) -> bool {
        true
    }
}

struct AllTokens;

impl<'a> GreenTokenWalkPolicy<'a> for AllTokens {
    #[inline]
    fn slots(&self, node: GreenNodeRef<'a>, _offset: TextSize) -> Option<&'a [Slot]> {
        Some(node.slice())
    }
}

struct TokensWithComments;

impl<'a> GreenTokenWalkPolicy<'a> for TokensWithComments {
    #[inline]
    fn slots(&self, node: GreenNodeRef<'a>, _offset: TextSize) -> Option<&'a [Slot]> {
        node.has_comments().then(|| node.slice())
    }

    #[inline]
    fn includes_token(&self, token: GreenTokenRef<'a>) -> bool {
        token.has_comments()
    }
}

struct TokensForRange {
    range: TextRange,
}

impl<'a> GreenTokenWalkPolicy<'a> for TokensForRange {
    #[inline]
    fn slots(&self, node: GreenNodeRef<'a>, offset: TextSize) -> Option<&'a [Slot]> {
        let slots = node.slice();
        let start = self.range.start();
        if offset >= start {
            return Some(slots);
        }

        let first = slots.partition_point(|slot| offset + slot.rel_range().end() <= start);
        Some(&slots[first..])
    }

    #[inline]
    fn stops_before(&self, offset: TextSize) -> bool {
        offset >= self.range.end()
    }
}

#[cfg(test)]
mod tests {
    use super::{GreenTokens, GreenTokensForRange, GreenTokensWithComments};
    use crate::green::{GreenElement, GreenNode, GreenToken, GreenTrivia, RawSyntaxKind};
    use crate::{TextRange, TriviaPiece};

    fn tree() -> GreenNode {
        let comment = GreenToken::with_trivia(
            RawSyntaxKind(0),
            "/*c*/b",
            GreenTrivia::new([TriviaPiece::multi_line_comment(5)]),
            GreenTrivia::empty(),
        );
        let nested = GreenNode::new(
            RawSyntaxKind(0),
            [
                Some(GreenElement::Token(comment)),
                None,
                Some(GreenElement::Token(GreenToken::new_raw(
                    RawSyntaxKind(0),
                    "",
                ))),
                Some(GreenElement::Token(GreenToken::new_raw(
                    RawSyntaxKind(0),
                    "ç",
                ))),
            ],
        );

        GreenNode::new(
            RawSyntaxKind(0),
            [
                Some(GreenElement::Token(GreenToken::new_raw(
                    RawSyntaxKind(0),
                    "a",
                ))),
                Some(GreenElement::Node(nested)),
                Some(GreenElement::Token(GreenToken::new_raw(
                    RawSyntaxKind(0),
                    "z",
                ))),
            ],
        )
    }

    #[test]
    fn tokens_are_in_source_order_with_absolute_offsets() {
        let tree = tree();
        let mut tokens = GreenTokens::new(tree.as_ref(), 10.into());
        let actual: Vec<_> = tokens
            .by_ref()
            .map(|token| (token.green.text(), u32::from(token.offset)))
            .collect();

        assert_eq!(
            actual,
            [("a", 10), ("/*c*/b", 11), ("", 17), ("ç", 17), ("z", 19)]
        );
        assert!(tokens.next().is_none());
        assert!(tokens.next().is_none());
    }

    #[test]
    fn comment_tokens_skip_unflagged_subtrees() {
        let tree = tree();
        let actual: Vec<_> = GreenTokensWithComments::new(tree.as_ref(), 10.into())
            .map(|token| (token.green.text(), u32::from(token.offset)))
            .collect();

        assert_eq!(actual, [("/*c*/b", 11)]);

        let plain = GreenNode::new(
            RawSyntaxKind(0),
            [Some(GreenElement::Token(GreenToken::new_raw(
                RawSyntaxKind(0),
                "plain",
            )))],
        );
        assert!(
            GreenTokensWithComments::new(plain.as_ref(), 0.into())
                .next()
                .is_none()
        );
    }

    #[test]
    fn range_tokens_preserve_zero_width_and_empty_range_behavior() {
        let tree = tree();
        let actual: Vec<_> = GreenTokensForRange::new(
            tree.as_ref(),
            10.into(),
            TextRange::new(12.into(), 18.into()),
        )
        .map(|token| (token.green.text(), u32::from(token.offset)))
        .collect();
        assert_eq!(actual, [("/*c*/b", 11), ("", 17), ("ç", 17)]);

        let inside_character: Vec<_> = GreenTokensForRange::new(
            tree.as_ref(),
            10.into(),
            TextRange::new(18.into(), 18.into()),
        )
        .map(|token| token.green.text())
        .collect();
        assert_eq!(inside_character, ["ç"]);

        assert!(
            GreenTokensForRange::new(
                tree.as_ref(),
                10.into(),
                TextRange::new(17.into(), 17.into()),
            )
            .next()
            .is_none()
        );
    }
}
