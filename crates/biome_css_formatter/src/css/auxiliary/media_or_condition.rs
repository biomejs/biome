use crate::prelude::*;
use biome_css_syntax::{CssMediaOrCondition, CssMediaOrConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaOrCondition;
impl FormatNodeRule<CssMediaOrCondition> for FormatCssMediaOrCondition {
    fn fmt_fields(&self, node: &CssMediaOrCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaOrConditionFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                or_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
