use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlIndentationIndicator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlIndentationIndicator;
impl FormatNodeRule<YamlIndentationIndicator> for FormatYamlIndentationIndicator {
    fn fmt_fields(
        &self,
        node: &YamlIndentationIndicator,
        f: &mut YamlFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
