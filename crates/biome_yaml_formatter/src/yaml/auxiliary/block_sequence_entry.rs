use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockSequenceEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequenceEntry;
impl FormatNodeRule<YamlBlockSequenceEntry> for FormatYamlBlockSequenceEntry {
    fn fmt_fields(&self, node: &YamlBlockSequenceEntry, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
