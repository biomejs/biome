use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_grit_syntax::{map_syntax_node, GritSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatGritSyntaxNode;

impl FormatRule<GritSyntaxNode> for FormatGritSyntaxNode {
    type Context = GritFormatContext;

    fn fmt(&self, node: &GritSyntaxNode, f: &mut GritFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<GritFormatContext> for FormatGritSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatHtmlSyntaxNode)
    }
}

impl IntoFormat<HtmlFormatContext> for FormatHtmlSyntaxNode {
    type Format = FormatOwnedWithRule<HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatHtmlSyntaxNode)
    }
}
