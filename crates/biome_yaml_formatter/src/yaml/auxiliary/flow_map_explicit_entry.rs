use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFlowMapExplicitEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapExplicitEntry;
impl FormatNodeRule<YamlFlowMapExplicitEntry> for FormatYamlFlowMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlFlowMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
