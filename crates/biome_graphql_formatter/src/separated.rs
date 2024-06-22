use crate::prelude::*;
use crate::FormatGraphqlSyntaxToken;
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::{FormatRefWithRule, FormatRuleWithOptions};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct GraphqlFormatSeparatedElementRule<N> {
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for GraphqlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = GraphqlLanguage> + AsFormat<GraphqlFormatContext> + 'static,
{
    type Context = GraphqlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, GraphqlSyntaxToken, FormatGraphqlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a GraphqlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type GraphqlFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<GraphqlLanguage, Node>,
    Node,
    GraphqlFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = GraphqlLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or created by
    /// calling the `separator_factory` function. The last trailing separator
    /// will not be printed by default. Use `with_trailing_separator` to add it
    /// in where necessary.
    fn format_separated(&self, separator: &'static str) -> GraphqlFormatSeparatedIter<Self::Node> {
        GraphqlFormatSeparatedIter::new(
            self.elements(),
            separator,
            GraphqlFormatSeparatedElementRule { node: PhantomData },
        )
        .with_trailing_separator(TrailingSeparator::Disallowed)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = GraphqlLanguage> {}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct GraphqlFormatSeparatedElementRuleWithOptions<N, O> {
    node: PhantomData<N>,
    options: O,
}

impl<N, O> GraphqlFormatSeparatedElementRuleWithOptions<N, O> {
    pub(crate) fn new(options: O) -> Self {
        Self {
            node: PhantomData,
            options,
        }
    }
}

impl<N, O, R> FormatSeparatedElementRule<N> for GraphqlFormatSeparatedElementRuleWithOptions<N, O>
where
    O: Clone + Copy,
    R: FormatNodeRule<N> + FormatRuleWithOptions<N, Context = GraphqlFormatContext, Options = O>,
    N: AstNode<Language = GraphqlLanguage>
        + for<'a> AsFormat<GraphqlFormatContext, Format<'a> = FormatRefWithRule<'a, N, R>>
        + 'static,
{
    type Context = GraphqlFormatContext;
    type FormatNode<'a> = FormatRefWithRule<'a, N, R>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, GraphqlSyntaxToken, FormatGraphqlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format().with_options(self.options)
    }

    fn format_separator<'a>(&self, separator: &'a GraphqlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}
