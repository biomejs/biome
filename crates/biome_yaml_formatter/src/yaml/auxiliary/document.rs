use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlDocument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocument;
impl FormatNodeRule<YamlDocument> for FormatYamlDocument {
    fn fmt_fields(&self, node: &YamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
