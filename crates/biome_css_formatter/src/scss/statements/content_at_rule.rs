use crate::prelude::*;
use crate::utils::scss_statement_at_rule::format_scss_statement_at_rule_semicolon;
use biome_css_syntax::{ScssContentAtRule, ScssContentAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssContentAtRule;

impl FormatNodeRule<ScssContentAtRule> for FormatScssContentAtRule {
    fn fmt_fields(&self, node: &ScssContentAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssContentAtRuleFields {
            content_token,
            arguments,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                content_token.format(),
                arguments.format(),
                format_scss_statement_at_rule_semicolon(semicolon_token)
            ]
        )
    }
}
