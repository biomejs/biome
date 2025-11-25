use crate::prelude::*;
use biome_css_syntax::CssFunctionParameter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameter;
impl FormatNodeRule<CssFunctionParameter> for FormatCssFunctionParameter {
    fn fmt_fields(&self, node: &CssFunctionParameter, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
