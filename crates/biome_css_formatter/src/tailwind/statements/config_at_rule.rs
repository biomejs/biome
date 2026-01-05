use crate::prelude::*;
use biome_css_syntax::{TwConfigAtRule, TwConfigAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwConfigAtRule;
impl FormatNodeRule<TwConfigAtRule> for FormatTwConfigAtRule {
    fn fmt_fields(&self, node: &TwConfigAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwConfigAtRuleFields {
            config_token,
            path,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                config_token.format(),
                space(),
                path.format(),
                semicolon_token.format()
            ]
        )
    }
}
