use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFlowMapping;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapping;
impl FormatNodeRule<YamlFlowMapping> for FormatYamlFlowMapping {
    fn fmt_fields(&self, node: &YamlFlowMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
