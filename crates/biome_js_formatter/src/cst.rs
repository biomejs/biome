use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::{AsFormat, IntoFormat, JsFormatContext};
use biome_js_syntax::{JsSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsSyntaxNode;

impl biome_formatter::FormatRule<JsSyntaxNode> for FormatJsSyntaxNode {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsSyntaxNode, f: &mut JsFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<JsFormatContext> for JsSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatJsSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsSyntaxNode)
    }
}

impl IntoFormat<JsFormatContext> for JsSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatJsSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsSyntaxNode)
    }
}
