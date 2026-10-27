use crate::FormatTomlSyntaxToken;
use crate::prelude::*;
use biome_formatter::FormatRefWithRule;
use biome_formatter::separated::{
    FormatSeparatedElementRule, FormatSeparatedIter, TrailingSeparator,
};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use biome_toml_syntax::{TomlLanguage, TomlSyntaxToken};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct TomlFormatSeparatedElementRule<N> {
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for TomlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = TomlLanguage> + AsFormat<TomlFormatContext> + 'static,
{
    type Context = TomlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, TomlSyntaxToken, FormatTomlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a TomlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type TomlFormatSeparatedIter<Node, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<TomlLanguage, Node>,
    Node,
    TomlFormatSeparatedElementRule<Node>,
    C,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = TomlLanguage>
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
    ) -> TomlFormatSeparatedIter<Self::Node, TomlFormatContext> {
        TomlFormatSeparatedIter::new(
            self.elements(),
            separator,
            TomlFormatSeparatedElementRule { node: PhantomData },
            on_skipped,
            on_removed,
        )
        .with_trailing_separator(trailing_separator)
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = TomlLanguage> {}
