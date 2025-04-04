use crate::prelude::*;
use biome_css_syntax::{CssSyntaxNode, map_syntax_node};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatCssSyntaxNode;

impl FormatRule<CssSyntaxNode> for FormatCssSyntaxNode {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssSyntaxNode, f: &mut CssFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<CssFormatContext> for CssSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatCssSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatCssSyntaxNode)
    }
}

impl IntoFormat<CssFormatContext> for CssSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatCssSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatCssSyntaxNode)
    }
}
