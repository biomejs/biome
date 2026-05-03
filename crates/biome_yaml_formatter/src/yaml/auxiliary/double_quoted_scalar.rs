use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlDoubleQuotedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDoubleQuotedScalar;
impl FormatNodeRule<YamlDoubleQuotedScalar> for FormatYamlDoubleQuotedScalar {
    fn fmt_fields(&self, node: &YamlDoubleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
