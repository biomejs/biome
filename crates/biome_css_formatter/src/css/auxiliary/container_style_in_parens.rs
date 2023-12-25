use crate::prelude::*;
use biome_css_syntax::{CssContainerStyleInParens, CssContainerStyleInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerStyleInParens;
impl FormatNodeRule<CssContainerStyleInParens> for FormatCssContainerStyleInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerStyleInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerStyleInParensFields {
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                &soft_block_indent(&query.format()),
                r_paren_token.format()
            ])]
        )
    }
}
