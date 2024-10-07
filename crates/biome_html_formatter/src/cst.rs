use biome_formatter::{Format, FormatOwnedWithRule, FormatRefWithRule, FormatResult};

use crate::{AsFormat, HtmlFormatContext, HtmlFormatter, IntoFormat};
use biome_html_syntax::{map_syntax_node, HtmlSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatHtmlSyntaxNode;

impl biome_formatter::FormatRule<HtmlSyntaxNode> for FormatHtmlSyntaxNode {
    type Context = HtmlFormatContext;

    fn fmt(&self, node: &HtmlSyntaxNode, f: &mut HtmlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<HtmlFormatContext> for HtmlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatHtmlSyntaxNode)
    }
}

impl IntoFormat<HtmlFormatContext> for HtmlSyntaxNode {
    type Format = FormatOwnedWithRule<HtmlSyntaxNode, FormatHtmlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatHtmlSyntaxNode)
    }
}
