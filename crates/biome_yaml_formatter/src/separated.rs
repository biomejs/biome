use crate::prelude::*;
use crate::{AsFormat, FormatYamlSyntaxToken, on_removed, on_skipped};
use biome_formatter::FormatRefWithRule;
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use biome_yaml_syntax::{YamlLanguage, YamlSyntaxToken};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct YamlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = YamlLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for YamlFormatSeparatedElementRule<N>
where
    N: AstNode<Language = YamlLanguage> + AsFormat<YamlFormatContext> + 'static,
{
    type Context = YamlFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, YamlSyntaxToken, FormatYamlSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a YamlSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type YamlFormatSeparatedIter<Node, C> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<YamlLanguage, Node>,
    Node,
    YamlFormatSeparatedElementRule<Node>,
    C,
>;

pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = YamlLanguage>
{
    fn format_separated(
        &self,
        separator: &'static str,
    ) -> YamlFormatSeparatedIter<Self::Node, YamlFormatContext> {
        YamlFormatSeparatedIter::new(
            self.elements(),
            separator,
            YamlFormatSeparatedElementRule { node: PhantomData },
            on_skipped,
            on_removed,
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = YamlLanguage> {}
