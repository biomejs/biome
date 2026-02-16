use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFlowJsonNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowJsonNode;
impl FormatNodeRule<YamlFlowJsonNode> for FormatYamlFlowJsonNode {
    fn fmt_fields(&self, node: &YamlFlowJsonNode, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
