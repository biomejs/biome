use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassFunctionNth, CssPseudoClassFunctionNthFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassFunctionNth;
impl FormatNodeRule<CssPseudoClassFunctionNth> for FormatCssPseudoClassFunctionNth {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassFunctionNth,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassFunctionNthFields {
            name,
            l_paren_token,
            selector,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&selector.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
