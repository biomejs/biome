use crate::prelude::*;
use biome_css_syntax::{CssCounterStyleAtRule, CssCounterStyleAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCounterStyleAtRule;
impl FormatNodeRule<CssCounterStyleAtRule> for FormatCssCounterStyleAtRule {
    fn fmt_fields(&self, node: &CssCounterStyleAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCounterStyleAtRuleFields {
            counter_style_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                counter_style_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
