use crate::prelude::*;
use biome_css_syntax::{CssSupportsAtRule, CssSupportsAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRule;
impl FormatNodeRule<CssSupportsAtRule> for FormatCssSupportsAtRule {
    fn fmt_fields(&self, node: &CssSupportsAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsAtRuleFields {
            supports_token,
            condition,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                supports_token.format(),
                space(),
                group(&indent(&condition.format())),
                space(),
                block.format()
            ]
        )
    }
}
