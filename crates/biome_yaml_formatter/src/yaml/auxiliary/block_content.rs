use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockContent;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockContent;
impl FormatNodeRule<YamlBlockContent> for FormatYamlBlockContent {
    fn fmt_fields(&self, node: &YamlBlockContent, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
