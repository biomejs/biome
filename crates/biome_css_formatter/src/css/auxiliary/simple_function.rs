use crate::prelude::*;
use biome_css_syntax::{CssSimpleFunction, CssSimpleFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSimpleFunction;
impl FormatNodeRule<CssSimpleFunction> for FormatCssSimpleFunction {
    fn fmt_fields(&self, node: &CssSimpleFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSimpleFunctionFields {
            name,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&items.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
