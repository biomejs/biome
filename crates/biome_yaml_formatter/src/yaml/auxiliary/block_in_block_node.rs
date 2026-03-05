use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockInBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockInBlockNode;
impl FormatNodeRule<YamlBlockInBlockNode> for FormatYamlBlockInBlockNode {
    fn fmt_fields(&self, node: &YamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
