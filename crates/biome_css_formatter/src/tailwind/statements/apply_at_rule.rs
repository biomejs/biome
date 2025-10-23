use crate::prelude::*;
use biome_css_syntax::{TwApplyAtRule, TwApplyAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyAtRule;
impl FormatNodeRule<TwApplyAtRule> for FormatTwApplyAtRule {
    fn fmt_fields(&self, node: &TwApplyAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwApplyAtRuleFields {
            apply_token,
            classes,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                apply_token.format(),
                space(),
                classes.format(),
                semicolon_token.format()
            ]
        )
    }
}
