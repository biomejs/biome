use crate::{MdFormatContext, prelude::*};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_markdown_syntax::{MdSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatMarkdownSyntaxNode;

impl FormatRule<MdSyntaxNode> for FormatMarkdownSyntaxNode {
    type Context = MdFormatContext;

    fn fmt(&self, node: &MdSyntaxNode, f: &mut MarkdownFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<MdFormatContext> for MdSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatMarkdownSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMarkdownSyntaxNode)
    }
}

impl IntoFormat<MdFormatContext> for MdSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatMarkdownSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMarkdownSyntaxNode)
    }
}
