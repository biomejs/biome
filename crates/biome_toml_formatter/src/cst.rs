use crate::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_toml_syntax::{TomlSyntaxNode, map_syntax_node};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatTomlSyntaxNode;

impl FormatRule<TomlSyntaxNode> for FormatTomlSyntaxNode {
    type Context = TomlFormatContext;

    fn fmt(&self, node: &TomlSyntaxNode, f: &mut TomlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<TomlFormatContext> for TomlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatTomlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatTomlSyntaxNode)
    }
}

impl IntoFormat<TomlFormatContext> for TomlSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatTomlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatTomlSyntaxNode)
    }
}
