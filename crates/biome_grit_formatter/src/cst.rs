use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatToken};
use biome_grit_syntax::{map_syntax_node, GritSyntaxNode, GritSyntaxToken};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatGritSyntaxNode;

impl FormatRule<GritSyntaxNode> for FormatGritSyntaxNode {
    type Context = GritFormatContext;

    fn fmt(&self, node: &GritSyntaxNode, f: &mut GritFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<GritFormatContext> for GritSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, GritSyntaxNode, FormatGritSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatGritSyntaxNode)
    }
}

impl IntoFormat<GritFormatContext> for GritSyntaxNode {
    type Format = FormatOwnedWithRule<GritSyntaxNode, FormatGritSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatGritSyntaxNode)
    }
}

/// Format implementation specific to GritQL tokens.
pub(crate) type FormatGritSyntaxToken = FormatToken<GritFormatContext>;

impl AsFormat<GritFormatContext> for GritSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, GritSyntaxToken, FormatGritSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatGritSyntaxToken::default())
    }
}

impl IntoFormat<GritFormatContext> for GritSyntaxToken {
    type Format = FormatOwnedWithRule<GritSyntaxToken, FormatGritSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatGritSyntaxToken::default())
    }
}
