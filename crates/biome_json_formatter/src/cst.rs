use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_json_syntax::{JsonSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsonSyntaxNode;

impl FormatRule<JsonSyntaxNode> for FormatJsonSyntaxNode {
    type Context = JsonFormatContext;

    fn fmt(&self, node: &JsonSyntaxNode, f: &mut JsonFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<JsonFormatContext> for JsonSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatJsonSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsonSyntaxNode)
    }
}

impl IntoFormat<JsonFormatContext> for JsonSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatJsonSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsonSyntaxNode)
    }
}
