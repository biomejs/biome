use crate::{MarkdownFormatterContext, prelude::*};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_markdown_syntax::{MarkdownSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatMarkdownSyntaxNode;

impl FormatRule<MarkdownSyntaxNode> for FormatMarkdownSyntaxNode {
    type Context = MarkdownFormatterContext;

    fn fmt(&self, node: &MarkdownSyntaxNode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<MarkdownFormatterContext> for MarkdownSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatMarkdownSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMarkdownSyntaxNode)
    }
}

impl IntoFormat<MarkdownFormatterContext> for MarkdownSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatMarkdownSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMarkdownSyntaxNode)
    }
}
