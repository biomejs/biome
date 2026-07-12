use crate::FormatCssSyntaxToken;
use crate::prelude::*;
use biome_css_syntax::{CssIdentifier, CssLanguage, CssSyntaxToken};
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::{
    FormatRefWithRule, FormatScopedOptions, FormatScopedOptionsExt as _, FormatWithRule,
    FormatWithScopedOptions,
};
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

type CssFormatSeparatedIter<Node, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<CssLanguage, Node>,
    Node,
    CssFormatSeparatedElementRule<Node>,
    C,
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
    fn format_separated(
        &self,
        separator: &'static str,
    ) -> CssFormatSeparatedIter<Self::Node, CssFormatContext> {
        CssFormatSeparatedIter::new(
            self.elements(),
            separator,
            CssFormatSeparatedElementRule { node: PhantomData },
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = CssLanguage> {}

#[derive(Debug, Clone)]
pub(crate) struct CssFormatSeparatedElementRuleWithScopedOptions<N, O> {
    node: PhantomData<N>,
    options: O,
}

impl<N, O> CssFormatSeparatedElementRuleWithScopedOptions<N, O> {
    fn new(options: O) -> Self {
        Self {
            node: PhantomData,
            options,
        }
    }
}

impl<N, O> FormatSeparatedElementRule<N> for CssFormatSeparatedElementRuleWithScopedOptions<N, O>
where
    O: Clone + FormatScopedOptions<CssFormatContext, N>,
    N: AstNode<Language = CssLanguage> + AsFormat<CssFormatContext> + 'static,
    for<'a> N::Format<'a>: FormatWithRule<CssFormatContext, Item = N>,
{
    type Context = CssFormatContext;
    type FormatNode<'a> = FormatWithScopedOptions<N::Format<'a>, O>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, CssSyntaxToken, FormatCssSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format().with_scoped_options(self.options.clone())
    }

    fn format_separator<'a>(&self, separator: &'a CssSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type CssFormatSeparatedIterWithScopedOptions<Node, Options, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<CssLanguage, Node>,
    Node,
    CssFormatSeparatedElementRuleWithScopedOptions<Node, Options>,
    C,
>;

/// AST separated-list formatting with scoped options.
pub(crate) trait FormatAstSeparatedListWithScopedOptionsExtension<O>:
    AstSeparatedList<Language = CssLanguage>
{
    /// Prints a separated list of nodes with scoped options.
    ///
    /// Trailing separators will be reused from the original list or created by
    /// calling the `separator_factory` function. The last trailing separator
    /// will not be printed by default. Use `with_trailing_separator` to add it
    /// in where necessary.
    fn format_separated_with_scoped_options(
        &self,
        separator: &'static str,
        options: O,
    ) -> CssFormatSeparatedIterWithScopedOptions<Self::Node, O, CssFormatContext> {
        FormatSeparatedIter::new(
            self.elements(),
            separator,
            CssFormatSeparatedElementRuleWithScopedOptions::new(options),
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T, O> FormatAstSeparatedListWithScopedOptionsExtension<O> for T where
    T: AstSeparatedList<Language = CssLanguage, Node = CssIdentifier>
{
}
