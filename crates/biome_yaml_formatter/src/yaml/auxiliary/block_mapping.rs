use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockMapping;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockMapping;
impl FormatNodeRule<YamlBlockMapping> for FormatYamlBlockMapping {
    fn fmt_fields(&self, node: &YamlBlockMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
