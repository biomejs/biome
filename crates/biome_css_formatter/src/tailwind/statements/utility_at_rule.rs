use crate::prelude::*;
use biome_css_syntax::{TwUtilityAtRule, TwUtilityAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwUtilityAtRule;
impl FormatNodeRule<TwUtilityAtRule> for FormatTwUtilityAtRule {
    fn fmt_fields(&self, node: &TwUtilityAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwUtilityAtRuleFields {
            utility_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                utility_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
