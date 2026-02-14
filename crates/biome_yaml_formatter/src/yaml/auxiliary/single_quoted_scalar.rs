use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlSingleQuotedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlSingleQuotedScalar;
impl FormatNodeRule<YamlSingleQuotedScalar> for FormatYamlSingleQuotedScalar {
    fn fmt_fields(&self, node: &YamlSingleQuotedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
