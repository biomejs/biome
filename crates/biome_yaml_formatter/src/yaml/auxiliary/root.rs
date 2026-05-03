use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlRoot;
impl FormatNodeRule<YamlRoot> for FormatYamlRoot {
    fn fmt_fields(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
