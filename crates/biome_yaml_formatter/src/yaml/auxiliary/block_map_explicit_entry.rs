use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockMapExplicitEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapExplicitEntry;
impl FormatNodeRule<YamlBlockMapExplicitEntry> for FormatYamlBlockMapExplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapExplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
