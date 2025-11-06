use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlLiteralScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlLiteralScalar;
impl FormatNodeRule<YamlLiteralScalar> for FormatYamlLiteralScalar {
    fn fmt_fields(&self, node: &YamlLiteralScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
