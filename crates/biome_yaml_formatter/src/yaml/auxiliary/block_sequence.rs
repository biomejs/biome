use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockSequence;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequence;
impl FormatNodeRule<YamlBlockSequence> for FormatYamlBlockSequence {
    fn fmt_fields(&self, node: &YamlBlockSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
