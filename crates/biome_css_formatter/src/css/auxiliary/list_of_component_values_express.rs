use crate::prelude::*;
use biome_css_syntax::CssListOfComponentValuesExpress;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssListOfComponentValuesExpress;
impl FormatNodeRule<CssListOfComponentValuesExpress> for FormatCssListOfComponentValuesExpress {
    fn fmt_fields(
        &self,
        node: &CssListOfComponentValuesExpress,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
