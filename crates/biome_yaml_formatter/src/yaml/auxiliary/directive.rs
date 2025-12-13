use crate::prelude::*;
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlDirective;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDirective;
impl FormatNodeRule<YamlDirective> for FormatYamlDirective {
    fn fmt_fields(&self, node: &YamlDirective, f: &mut YamlFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
