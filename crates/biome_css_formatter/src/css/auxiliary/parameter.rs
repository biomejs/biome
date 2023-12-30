use crate::prelude::*;
use biome_css_syntax::{CssParameter, CssParameterFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameter;
impl FormatNodeRule<CssParameter> for FormatCssParameter {
    fn fmt_fields(&self, node: &CssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        let CssParameterFields { any_css_expression } = node.as_fields();

        write!(f, [any_css_expression.format()])
    }
}
