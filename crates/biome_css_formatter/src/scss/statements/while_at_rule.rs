use crate::prelude::*;
use crate::utils::scss_control_condition::ScssControlConditionLayout;
use biome_css_syntax::{ScssWhileAtRule, ScssWhileAtRuleFields};
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
        let condition = condition?;
        let header_group_id = f.group_id("scss_while_header");
        let condition_layout = ScssControlConditionLayout::from_condition(&condition);
        let formatted_condition = format_with(|f| {
            if condition_layout.should_indent_condition() {
                write!(
                    f,
                    [indent_if_group_breaks(&condition.format(), header_group_id)]
                )
            } else {
                write!(f, [condition.format()])
            }
        });
        let block_separator = format_with(|f| {
            if condition_layout.should_keep_block_on_same_line() {
                write!(f, [space()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        write!(
            f,
            [
                while_token.format(),
                space(),
                group(&format_args![formatted_condition, block_separator])
                    .with_group_id(Some(header_group_id)),
                block.format()
            ]
        )
    }
}
