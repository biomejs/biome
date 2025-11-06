use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlPropertiesTagFirst;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPropertiesTagFirst;
impl FormatNodeRule<YamlPropertiesTagFirst> for FormatYamlPropertiesTagFirst {
    fn fmt_fields(&self, node: &YamlPropertiesTagFirst, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
