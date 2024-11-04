use biome_formatter::{
    separated::{FormatSeparatedElementRule, FormatSeparatedIter},
    FormatRefWithRule,
};

use crate::prelude::*;
use biome_grit_syntax::{GritLanguage, GritSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

use crate::{cst::FormatGritSyntaxToken, AsFormat, GritFormatContext};

#[derive(Clone)]
pub(crate) struct GritFormatSeparatedElementRule<N>
where
    N: AstNode<Language = GritLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for GritFormatSeparatedElementRule<N>
where
    N: AstNode<Language = GritLanguage> + AsFormat<GritFormatContext> + 'static,
{
    type Context = GritFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, GritSyntaxToken, FormatGritSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a GritSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type GritFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<GritLanguage, Node>,
    Node,
    GritFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = GritLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(&self, separator: &'static str) -> GritFormatSeparatedIter<Self::Node> {
        GritFormatSeparatedIter::new(
            self.elements(),
            separator,
            GritFormatSeparatedElementRule { node: PhantomData },
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = GritLanguage> {}
