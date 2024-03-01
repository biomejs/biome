use crate::prelude::*;
use crate::FormatJsonSyntaxToken;
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_formatter::FormatRefWithRule;
use biome_json_syntax::{JsonLanguage, JsonSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct JsonFormatSeparatedElementRule<N> {
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for JsonFormatSeparatedElementRule<N>
where
    N: AstNode<Language = JsonLanguage> + AsFormat<JsonFormatContext> + 'static,
{
    type Context = JsonFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, JsonSyntaxToken, FormatJsonSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a JsonSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type JsonFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<JsonLanguage, Node>,
    Node,
    JsonFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = JsonLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(
        &self,
        separator: &'static str,
        trailing_separator: TrailingSeparator,
    ) -> JsonFormatSeparatedIter<Self::Node> {
        JsonFormatSeparatedIter::new(
            self.elements(),
            separator,
            JsonFormatSeparatedElementRule { node: PhantomData },
        )
        .with_trailing_separator(trailing_separator)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = JsonLanguage> {}
