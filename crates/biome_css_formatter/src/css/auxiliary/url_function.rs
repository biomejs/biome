use crate::prelude::*;
use biome_css_syntax::{CssUrlFunction, CssUrlFunctionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlFunction;
impl FormatNodeRule<CssUrlFunction> for FormatCssUrlFunction {
    fn fmt_fields(&self, node: &CssUrlFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlFunctionFields {
            url_token,
            l_paren_token,
            any_css_url_value,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                url_token.format(),
                l_paren_token.format(),
                any_css_url_value.format(),
                r_paren_token.format()
            ]
        )
    }
}
