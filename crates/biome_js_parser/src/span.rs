use biome_rowan::{Language, SyntaxElement, SyntaxNode, SyntaxToken, TextRange, TextSize};
use std::{fmt::Debug, ops::Range};

/// A value which can be used as the range inside of a diagnostic.
///
/// This is essentially a hack to allow us to use SyntaxElement, SyntaxNode, etc directly
pub trait Span {
    fn as_range(&self) -> TextRange;
}

impl<T: Span> Span for &T {
    fn as_range(&self) -> TextRange {
        (*self).as_range()
    }
}

impl<T: Span> Span for &mut T {
    fn as_range(&self) -> TextRange {
        (**self).as_range()
    }
}

impl<T: Copy> Span for Range<T>
where
    TextSize: TryFrom<T>,
    <TextSize as TryFrom<T>>::Error: Debug,
{
    fn as_range(&self) -> TextRange {
        TextRange::new(
            TextSize::try_from(self.start).expect("integer overflow"),
            TextSize::try_from(self.end).expect("integer overflow"),
        )
    }
}

impl<T: Language> Span for SyntaxNode<T> {
    fn as_range(&self) -> TextRange {
        self.text_range_with_trivia()
    }
}

impl<T: Language> Span for SyntaxToken<T> {
    fn as_range(&self) -> TextRange {
        self.text_range()
    }
}

impl<T: Language> Span for SyntaxElement<T> {
    fn as_range(&self) -> TextRange {
        match self {
            SyntaxElement::Node(n) => n.text_range_with_trivia(),
            SyntaxElement::Token(t) => t.text_range(),
        }
    }
}

impl Span for TextRange {
    fn as_range(&self) -> TextRange {
        *self
    }
}
