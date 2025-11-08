use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_yaml_syntax::{YamlSyntaxNode, map_syntax_node};

use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatYamlSyntaxNode;

impl FormatRule<YamlSyntaxNode> for FormatYamlSyntaxNode {
    type Context = YamlFormatContext;

    fn fmt(&self, node: &YamlSyntaxNode, f: &mut YamlFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<YamlFormatContext> for YamlSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, Self, FormatYamlSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatYamlSyntaxNode)
    }
}

impl IntoFormat<YamlFormatContext> for YamlSyntaxNode {
    type Format = FormatOwnedWithRule<Self, FormatYamlSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatYamlSyntaxNode)
    }
}
