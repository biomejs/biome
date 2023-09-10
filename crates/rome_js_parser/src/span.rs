use biome_rowan::{Language, SyntaxElement, SyntaxNode, SyntaxToken, TextRange, TextSize};
use std::{fmt::Debug, ops::Range};

/// A value which can be used as the range inside of a diagnostic.
///
/// This is essentially a hack to allow us to use SyntaxElement, SyntaxNode, etc directly
pub trait Span {
    fn as_range(&self) -> TextRange;

    /// Make a new span which extends to another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^^^
    /// ```
    fn join<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().start(), other.as_range().end())
    }

    /// Make a new span which is between another span
    ///
    /// ```text
    /// from      to
    ///     ^^^^^^
    /// ```
    fn between<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().end(), other.as_range().start())
    }

    /// Make a new span which extends until another span
    ///
    /// ```text
    /// from      to
    /// ^^^^^^^^^^
    /// ```
    fn until<T: Span>(&self, other: T) -> TextRange {
        TextRange::new(self.as_range().start(), other.as_range().start())
    }

    fn sub_start(&self, amount: TextSize) -> TextRange {
        self.as_range().sub_start(amount)
    }

    fn add_start(&self, amount: TextSize) -> TextRange {
        self.as_range().add_start(amount)
    }

    fn sub_end(&self, amount: TextSize) -> TextRange {
        self.as_range().sub_end(amount)
    }

    fn add_end(&self, amount: TextSize) -> TextRange {
        self.as_range().add_end(amount)
    }
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
        self.text_range()
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
            SyntaxElement::Node(n) => n.text_range(),
            SyntaxElement::Token(t) => t.text_range(),
        }
    }
}

impl Span for TextRange {
    fn as_range(&self) -> TextRange {
        *self
    }
}
