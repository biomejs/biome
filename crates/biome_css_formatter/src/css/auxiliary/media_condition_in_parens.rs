use crate::prelude::*;
use biome_css_syntax::{CssMediaConditionInParens, CssMediaConditionInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaConditionInParens;
impl FormatNodeRule<CssMediaConditionInParens> for FormatCssMediaConditionInParens {
    fn fmt_fields(
        &self,
        node: &CssMediaConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssMediaConditionInParensFields {
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&condition.format()),
                r_paren_token.format()
            ])]
        )
    }
}
