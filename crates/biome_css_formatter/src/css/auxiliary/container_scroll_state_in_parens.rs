use crate::prelude::*;
use biome_css_syntax::{CssContainerScrollStateInParens, CssContainerScrollStateInParensFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerScrollStateInParens;
impl FormatNodeRule<CssContainerScrollStateInParens> for FormatCssContainerScrollStateInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerScrollStateInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerScrollStateInParensFields {
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
