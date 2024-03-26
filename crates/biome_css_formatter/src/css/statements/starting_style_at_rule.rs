use crate::prelude::*;
use biome_css_syntax::{CssStartingStyleAtRule, CssStartingStyleAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssStartingStyleAtRule;
impl FormatNodeRule<CssStartingStyleAtRule> for FormatCssStartingStyleAtRule {
    fn fmt_fields(&self, node: &CssStartingStyleAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssStartingStyleAtRuleFields {
            starting_style_token,
            block,
        } = node.as_fields();
        write!(f, [starting_style_token.format(), space(), block.format()])
    }
}
