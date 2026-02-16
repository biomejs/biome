use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFlowInBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowInBlockNode;
impl FormatNodeRule<YamlFlowInBlockNode> for FormatYamlFlowInBlockNode {
    fn fmt_fields(&self, node: &YamlFlowInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
