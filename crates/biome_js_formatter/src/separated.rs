use crate::prelude::*;
use crate::{AsFormat, FormatJsSyntaxToken};
use biome_formatter::FormatRefWithRule;
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_js_syntax::{JsLanguage, JsSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct JsFormatSeparatedElementRule<N>
where
    N: AstNode<Language = JsLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for JsFormatSeparatedElementRule<N>
where
    N: AstNode<Language = JsLanguage> + AsFormat<JsFormatContext> + 'static,
{
    type Context = JsFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, JsSyntaxToken, FormatJsSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a JsSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type JsFormatSeparatedIter<Node, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<JsLanguage, Node>,
    Node,
    JsFormatSeparatedElementRule<Node>,
    C,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = JsLanguage>
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
    ) -> JsFormatSeparatedIter<Self::Node, JsFormatContext> {
        JsFormatSeparatedIter::new(
            self.elements(),
            separator,
            JsFormatSeparatedElementRule { node: PhantomData },
            on_skipped,
            on_removed,
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = JsLanguage> {}
