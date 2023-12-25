use crate::prelude::*;
use biome_css_syntax::CssListOfComponentValuesExpression;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssListOfComponentValuesExpression;
impl FormatNodeRule<CssListOfComponentValuesExpression>
    for FormatCssListOfComponentValuesExpression
{
    fn fmt_fields(
        &self,
        node: &CssListOfComponentValuesExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
