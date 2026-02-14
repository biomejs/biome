use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlPropertiesAnchorFirst;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlPropertiesAnchorFirst;
impl FormatNodeRule<YamlPropertiesAnchorFirst> for FormatYamlPropertiesAnchorFirst {
    fn fmt_fields(
        &self,
        node: &YamlPropertiesAnchorFirst,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
