use crate::prelude::*;
use crate::utils::scss_statement_at_rule::format_scss_statement_at_rule_semicolon;
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
                format_scss_statement_at_rule_semicolon(semicolon_token)
            ]
        )
    }
}
