use crate::prelude::*;
use biome_css_syntax::{ScssDebugAtRule, ScssDebugAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssDebugAtRule;

impl FormatNodeRule<ScssDebugAtRule> for FormatScssDebugAtRule {
    fn fmt_fields(&self, node: &ScssDebugAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssDebugAtRuleFields {
            debug_token,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                debug_token.format(),
                space(),
                value.format(),
                semicolon_token.format()
            ]
        )
    }
}
