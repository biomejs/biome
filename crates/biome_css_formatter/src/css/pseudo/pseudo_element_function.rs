use crate::prelude::*;
use biome_css_syntax::{CssPseudoElementFunction, CssPseudoElementFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunction;
impl FormatNodeRule<CssPseudoElementFunction> for FormatCssPseudoElementFunction {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementFunctionFields {
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
