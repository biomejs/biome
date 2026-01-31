use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFoldedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFoldedScalar;
impl FormatNodeRule<YamlFoldedScalar> for FormatYamlFoldedScalar {
    fn fmt_fields(&self, node: &YamlFoldedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
