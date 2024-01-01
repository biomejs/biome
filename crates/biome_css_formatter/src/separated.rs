use crate::prelude::*;
use crate::FormatCssSyntaxToken;
use biome_css_syntax::{CssLanguage, CssSyntaxToken};
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::FormatRefWithRule;
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct CssFormatSeparatedElementRule<N> {
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for CssFormatSeparatedElementRule<N>
where
    N: AstNode<Language = CssLanguage> + AsFormat<CssFormatContext> + 'static,
{
    type Context = CssFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, CssSyntaxToken, FormatCssSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a CssSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

// Unused currently because CSS formatting is very barebones for now
#[allow(unused)]
type CssFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<CssLanguage, Node>,
    Node,
    CssFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = CssLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or created by
    /// calling the `separator_factory` function. The last trailing separator
    /// will not be printed by default. Use `with_trailing_separator` to add it
    /// in where necessary.
    fn format_separated(&self, separator: &'static str) -> CssFormatSeparatedIter<Self::Node> {
        CssFormatSeparatedIter::new(
            self.elements(),
            separator,
            CssFormatSeparatedElementRule { node: PhantomData },
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = CssLanguage> {}
