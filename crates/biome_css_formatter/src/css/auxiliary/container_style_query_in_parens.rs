use crate::prelude::*;
use biome_css_syntax::{CssContainerStyleQueryInParens, CssContainerStyleQueryInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleQueryInParens;
impl FormatNodeRule<CssContainerStyleQueryInParens> for FormatCssContainerStyleQueryInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleQueryInParensFields {
            style_token,
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                style_token.format(),
                space(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&query.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
