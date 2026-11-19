use crate::prelude::*;
use crate::utils::statement_at_rule_ending::FormatStatementAtRuleEnding;
use biome_css_syntax::{ScssExtendAtRule, ScssExtendAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExtendAtRule;
impl FormatNodeRule<ScssExtendAtRule> for FormatScssExtendAtRule {
    fn fmt_fields(&self, node: &ScssExtendAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExtendAtRuleFields {
            extend_token,
            css_selector_list,
            optional_modifier,
            semicolon_token,
        } = node.as_fields();

        let target = format_once(|f| {
            write!(f, [group(&css_selector_list.format())])?;

            if let Some(optional_modifier) = optional_modifier {
                write!(f, [space(), optional_modifier.format()])?;
            }

            Ok(())
        });

        write!(
            f,
            [
                extend_token.format()?.with_text_case(CssCase::Lowercase),
                space(),
                group(&target),
                FormatStatementAtRuleEnding::new(node.syntax(), semicolon_token)
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _node: &ScssExtendAtRule,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
