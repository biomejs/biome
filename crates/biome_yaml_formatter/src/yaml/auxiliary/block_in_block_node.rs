use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlBlockInBlockNode, YamlBlockInBlockNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockInBlockNode;
impl FormatNodeRule<YamlBlockInBlockNode> for FormatYamlBlockInBlockNode {
    fn fmt_fields(&self, node: &YamlBlockInBlockNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockInBlockNodeFields {
            properties,
            content,
        } = node.as_fields();

        if properties.len() > 0 {
            // TODO: Implement formatting for block nodes with tag or anchor properties.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        write!(f, [content.format()])
    }
}
