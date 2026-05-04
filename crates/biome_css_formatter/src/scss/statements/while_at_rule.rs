use crate::prelude::*;
use biome_css_syntax::{ScssWhileAtRule, ScssWhileAtRuleFields, single_expression_item};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssWhileAtRule;

impl FormatNodeRule<ScssWhileAtRule> for FormatScssWhileAtRule {
    fn fmt_fields(&self, node: &ScssWhileAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssWhileAtRuleFields {
            while_token,
            condition,
            block,
        } = node.as_fields();

        let is_parenthesized_condition = condition.as_ref().is_ok_and(|condition| {
            single_expression_item(condition)
                .is_some_and(|item| item.as_scss_parenthesized_expression().is_some())
        });

        if is_parenthesized_condition {
            return write!(
                f,
                [
                    while_token.format(),
                    space(),
                    condition.format(),
                    space(),
                    block.format()
                ]
            );
        }

        let header_group_id = f.group_id("scss_while_header");

        write!(
            f,
            [
                while_token.format(),
                space(),
                group(&format_args![
                    indent_if_group_breaks(&condition.format(), header_group_id),
                    soft_line_break_or_space()
                ])
                .with_group_id(Some(header_group_id)),
                block.format()
            ]
        )
    }
}
