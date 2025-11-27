use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFlowSequence;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequence;
impl FormatNodeRule<YamlFlowSequence> for FormatYamlFlowSequence {
    fn fmt_fields(&self, node: &YamlFlowSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
