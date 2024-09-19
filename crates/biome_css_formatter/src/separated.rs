use crate::prelude::*;
use crate::FormatCssSyntaxToken;
use biome_css_syntax::{CssIdentifier, CssLanguage, CssSyntaxToken};
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::{FormatRefWithRule, FormatRuleWithOptions};
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

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct CssFormatSeparatedElementRuleWithOptions<N, O> {
    node: PhantomData<N>,
    options: O,
}

impl<N, O> CssFormatSeparatedElementRuleWithOptions<N, O> {
    pub(crate) fn new(options: O) -> Self {
        Self {
            node: PhantomData,
            options,
        }
    }
}

impl<N, O, R> FormatSeparatedElementRule<N> for CssFormatSeparatedElementRuleWithOptions<N, O>
where
    O: Clone + Copy,
    R: FormatNodeRule<N> + FormatRuleWithOptions<N, Context = CssFormatContext, Options = O>,
    N: AstNode<Language = CssLanguage>
        + for<'a> AsFormat<CssFormatContext, Format<'a> = FormatRefWithRule<'a, N, R>>
        + 'static,
{
    type Context = CssFormatContext;
    type FormatNode<'a> = FormatRefWithRule<'a, N, R>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, CssSyntaxToken, FormatCssSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format().with_options(self.options)
    }

    fn format_separator<'a>(&self, separator: &'a CssSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type CssFormatSeparatedIterWithOptions<Node, Options> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<CssLanguage, Node>,
    Node,
    CssFormatSeparatedElementRuleWithOptions<Node, Options>,
>;

/// AST Separated list formatting extension methods with options
#[expect(dead_code)]
pub(crate) trait FormatAstSeparatedListWithOptionsExtension<O>:
    AstSeparatedList<Language = CssLanguage>
{
    /// Prints a separated list of nodes with options
    ///
    /// Trailing separators will be reused from the original list or created by
    /// calling the `separator_factory` function. The last trailing separator
    /// will not be printed by default. Use `with_trailing_separator` to add it
    /// in where necessary.
    fn format_separated_with_options(
        &self,
        separator: &'static str,
        options: O,
    ) -> CssFormatSeparatedIterWithOptions<Self::Node, O> {
        FormatSeparatedIter::new(
            self.elements(),
            separator,
            CssFormatSeparatedElementRuleWithOptions::new(options),
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T, O> FormatAstSeparatedListWithOptionsExtension<O> for T where
    T: AstSeparatedList<Language = CssLanguage, Node = CssIdentifier>
{
}
