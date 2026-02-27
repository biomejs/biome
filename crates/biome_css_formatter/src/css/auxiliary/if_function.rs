use crate::prelude::*;
use biome_css_syntax::{CssIfFunction, CssIfFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfFunction;

impl FormatNodeRule<CssIfFunction> for FormatCssIfFunction {
    fn fmt_fields(&self, node: &CssIfFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIfFunctionFields {
            if_token,
            l_paren_token,
            css_if_branch_list,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                if_token.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&css_if_branch_list.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
