use crate::prelude::*;
use biome_css_syntax::{CssSupportsFeatureAtRule, CssSupportsFeatureAtRuleFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsFeatureAtRule;
impl FormatNodeRule<CssSupportsFeatureAtRule> for FormatCssSupportsFeatureAtRule {
    fn fmt_fields(
        &self,
        node: &CssSupportsFeatureAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsFeatureAtRuleFields {
            name,
            l_paren_token,
            at_token,
            at_rule,
            r_paren_token,
        } = node.as_fields();
        let should_insert_space = f.options().delimiter_spacing().value();
        let at_rule = at_rule?;
        // The identifier is one unchanged token, including when suppressed.
        // Its leading comments must precede the complete at-keyword.
        f.comments().mark_suppression_checked(at_rule.syntax());

        write!(
            f,
            [group(&format_args![
                name.format(),
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(
                    &format_args![
                        format_leading_comments(at_rule.syntax()),
                        at_token.format(),
                        at_rule.value_token().format(),
                        format_dangling_comments(at_rule.syntax()),
                        format_trailing_comments(at_rule.syntax()),
                    ],
                    should_insert_space,
                ),
                r_paren_token.format(),
            ])]
        )
    }
}
