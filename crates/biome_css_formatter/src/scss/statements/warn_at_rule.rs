use crate::prelude::*;
use biome_css_syntax::{ScssWarnAtRule, ScssWarnAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWarnAtRule;

impl FormatNodeRule<ScssWarnAtRule> for FormatScssWarnAtRule {
    fn fmt_fields(&self, node: &ScssWarnAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWarnAtRuleFields {
            warn_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                warn_token.format(),
                space(),
                value.format(),
                semicolon_token.format()
            ]
        )
    }
}
