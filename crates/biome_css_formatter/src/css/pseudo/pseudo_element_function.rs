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

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&items.format(), should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
