use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssCustomPropertyFunction;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyFunction;
impl FormatNodeRule<CssCustomPropertyFunction> for FormatCssCustomPropertyFunction {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_node(node.syntax()).fmt(f)
    }
}
