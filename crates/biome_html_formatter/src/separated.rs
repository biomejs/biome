use crate::FormatHtmlSyntaxToken;
use crate::prelude::*;
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::{FormatRefWithRule, FormatRuleWithOptions};
use biome_html_syntax::{HtmlLanguage, HtmlSyntaxToken, SvelteName};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct HtmlFormatSeparatedElementRule<N> {
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for HtmlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = HtmlLanguage> + AsFormat<HtmlFormatContext> + 'static,
{
    type Context = HtmlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, HtmlSyntaxToken, FormatHtmlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a HtmlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type HtmlFormatSeparatedIter<Node, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<HtmlLanguage, Node>,
    Node,
    HtmlFormatSeparatedElementRule<Node>,
    C,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = HtmlLanguage>
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
    ) -> HtmlFormatSeparatedIter<Self::Node, HtmlFormatContext> {
        HtmlFormatSeparatedIter::new(
            self.elements(),
            separator,
            HtmlFormatSeparatedElementRule { node: PhantomData },
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = HtmlLanguage> {}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct HtmlFormatSeparatedElementRuleWithOptions<N, O> {
    node: PhantomData<N>,
    options: O,
}

impl<N, O> HtmlFormatSeparatedElementRuleWithOptions<N, O> {
    pub(crate) fn new(options: O) -> Self {
        Self {
            node: PhantomData,
            options,
        }
    }
}

impl<N, O, R> FormatSeparatedElementRule<N> for HtmlFormatSeparatedElementRuleWithOptions<N, O>
where
    O: Clone + Copy,
    R: FormatNodeRule<N> + FormatRuleWithOptions<N, Context = HtmlFormatContext, Options = O>,
    N: AstNode<Language = HtmlLanguage>
        + for<'a> AsFormat<HtmlFormatContext, Format<'a> = FormatRefWithRule<'a, N, R>>
        + 'static,
{
    type Context = HtmlFormatContext;
    type FormatNode<'a> = FormatRefWithRule<'a, N, R>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, HtmlSyntaxToken, FormatHtmlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format().with_options(self.options)
    }

    fn format_separator<'a>(&self, separator: &'a HtmlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type HtmlFormatSeparatedIterWithOptions<Node, Options, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<HtmlLanguage, Node>,
    Node,
    HtmlFormatSeparatedElementRuleWithOptions<Node, Options>,
    C,
>;

/// AST Separated list formatting extension methods with options
#[expect(dead_code)]
pub(crate) trait FormatAstSeparatedListWithOptionsExtension<O>:
    AstSeparatedList<Language = HtmlLanguage>
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
    ) -> HtmlFormatSeparatedIterWithOptions<Self::Node, O, HtmlFormatContext> {
        FormatSeparatedIter::new(
            self.elements(),
            separator,
            HtmlFormatSeparatedElementRuleWithOptions::new(options),
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T, O> FormatAstSeparatedListWithOptionsExtension<O> for T where
    // TODO: probably it requires an update because it's tight to svelte grammar
    T: AstSeparatedList<Language = HtmlLanguage, Node = SvelteName>
{
}
