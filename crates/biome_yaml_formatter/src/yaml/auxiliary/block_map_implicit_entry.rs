use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockMapImplicitEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapImplicitEntry;
impl FormatNodeRule<YamlBlockMapImplicitEntry> for FormatYamlBlockMapImplicitEntry {
    fn fmt_fields(
        &self,
        node: &YamlBlockMapImplicitEntry,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
