use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlAnchorProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlAnchorProperty;
impl FormatNodeRule<YamlAnchorProperty> for FormatYamlAnchorProperty {
    fn fmt_fields(&self, node: &YamlAnchorProperty, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
