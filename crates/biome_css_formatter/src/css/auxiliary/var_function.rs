use crate::prelude::*;
use biome_css_syntax::{CssVarFunction, CssVarFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssVarFunction;
impl FormatNodeRule<CssVarFunction> for FormatCssVarFunction {
    fn fmt_fields(&self, node: &CssVarFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssVarFunctionFields {
            var_token,
            l_paren_token,
            property,
            value,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                var_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    property.format(),
                    // `value` takes care of formatting both the comma and the
                    // default value. Since it's an `Option`, it can just
                    // always be formatted inline and will have no effect if
                    // it is `None` instead.
                    value.format(),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
