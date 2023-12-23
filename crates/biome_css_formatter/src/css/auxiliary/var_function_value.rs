use crate::prelude::*;
use biome_css_syntax::{CssVarFunctionValue, CssVarFunctionValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssVarFunctionValue;
impl FormatNodeRule<CssVarFunctionValue> for FormatCssVarFunctionValue {
    fn fmt_fields(&self, node: &CssVarFunctionValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssVarFunctionValueFields { comma_token, value } = node.as_fields();

        write!(
            f,
            [
                comma_token.format(),
                soft_line_break_or_space(),
                value.format()
            ]
        )
    }
}
