use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlAliasNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlAliasNode;
impl FormatNodeRule<YamlAliasNode> for FormatYamlAliasNode {
    fn fmt_fields(&self, node: &YamlAliasNode, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
