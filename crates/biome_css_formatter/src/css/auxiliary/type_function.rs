use crate::prelude::*;
use biome_css_syntax::{CssTypeFunction, CssTypeFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTypeFunction;

impl FormatNodeRule<CssTypeFunction> for FormatCssTypeFunction {
    fn fmt_fields(&self, node: &CssTypeFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssTypeFunctionFields {
            name_token,
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&ty.format()),
                    r_paren_token.format(),
                ])
            ]
        )
    }
}
