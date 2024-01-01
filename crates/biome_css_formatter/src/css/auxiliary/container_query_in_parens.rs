use crate::prelude::*;
use biome_css_syntax::{CssContainerQueryInParens, CssContainerQueryInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerQueryInParens;
impl FormatNodeRule<CssContainerQueryInParens> for FormatCssContainerQueryInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerQueryInParensFields {
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&query.format()),
                r_paren_token.format()
            ])]
        )
    }
}
