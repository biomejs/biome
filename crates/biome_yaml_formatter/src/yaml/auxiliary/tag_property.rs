use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlTagProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlTagProperty;
impl FormatNodeRule<YamlTagProperty> for FormatYamlTagProperty {
    fn fmt_fields(&self, node: &YamlTagProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
