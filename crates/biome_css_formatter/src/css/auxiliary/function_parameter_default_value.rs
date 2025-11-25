use crate::prelude::*;
use biome_css_syntax::CssFunctionParameterDefaultValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameterDefaultValue;
impl FormatNodeRule<CssFunctionParameterDefaultValue> for FormatCssFunctionParameterDefaultValue {
    fn fmt_fields(
        &self,
        node: &CssFunctionParameterDefaultValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
