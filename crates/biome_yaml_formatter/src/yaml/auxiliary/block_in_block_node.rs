use crate::prelude::*;
use crate::utils::block_scalar::is_bare_block_scalar_node;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockInBlockNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockInBlockNode;
impl FormatNodeRule<YamlBlockInBlockNode> for FormatYamlBlockInBlockNode {
    fn fmt_fields(&self, node: &YamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        if !is_bare_block_scalar_node(node) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }
        let fields = node.as_fields();
        biome_formatter::write!(f, [fields.properties.format(), fields.content?.format()])
    }
}
