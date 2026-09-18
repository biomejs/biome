use crate::prelude::*;
use crate::utils::statement_at_rule_ending::FormatStatementAtRuleEnding;
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
                apply_token.format()?.with_text_case(CssCase::Lowercase),
                space(),
                classes.format(),
                FormatStatementAtRuleEnding::new(node.syntax(), semicolon_token)
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _node: &TwApplyAtRule,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
