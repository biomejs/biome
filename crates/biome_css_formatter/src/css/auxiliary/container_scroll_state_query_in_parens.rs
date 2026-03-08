use crate::prelude::*;
use biome_css_syntax::{
    CssContainerScrollStateQueryInParens, CssContainerScrollStateQueryInParensFields,
};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerScrollStateQueryInParens;
impl FormatNodeRule<CssContainerScrollStateQueryInParens>
    for FormatCssContainerScrollStateQueryInParens
{
    fn fmt_fields(
        &self,
        node: &CssContainerScrollStateQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerScrollStateQueryInParensFields {
            name,
            l_paren_token,
            query,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&query.format()),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
