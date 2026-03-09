use crate::prelude::*;
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
                semicolon_token.format()
            ]
        )
    }
}
