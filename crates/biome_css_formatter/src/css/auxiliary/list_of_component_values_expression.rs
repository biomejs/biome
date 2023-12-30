use crate::prelude::*;
use biome_css_syntax::{
    CssListOfComponentValuesExpression, CssListOfComponentValuesExpressionFields,
};
use biome_formatter::write;

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
        let CssListOfComponentValuesExpressionFields {
            css_component_value_list,
        } = node.as_fields();

        write!(f, [css_component_value_list.format()])
    }
}
