use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlBlockStripIndicator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockStripIndicator;
impl FormatNodeRule<YamlBlockStripIndicator> for FormatYamlBlockStripIndicator {
    fn fmt_fields(
        &self,
        node: &YamlBlockStripIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
