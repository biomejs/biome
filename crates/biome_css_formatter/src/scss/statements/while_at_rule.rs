use crate::prelude::*;
use biome_css_syntax::{ScssWhileAtRule, ScssWhileAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWhileAtRule;

impl FormatNodeRule<ScssWhileAtRule> for FormatScssWhileAtRule {
    fn fmt_fields(&self, node: &ScssWhileAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWhileAtRuleFields {
            while_token,
            condition,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                while_token.format(),
                space(),
                condition.format(),
                space(),
                block.format()
            ]
        )
    }
}
