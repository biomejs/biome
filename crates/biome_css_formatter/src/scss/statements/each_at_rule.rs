use crate::prelude::*;
use crate::utils::scss_expression::single_expression_item;
use biome_css_syntax::{ScssEachAtRule, ScssEachAtRuleFields, ScssExpression};
use biome_formatter::{format_args, write};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachAtRule;

impl FormatNodeRule<ScssEachAtRule> for FormatScssEachAtRule {
    fn fmt_fields(&self, node: &ScssEachAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachAtRuleFields {
            each_token,
            bindings,
            in_token,
            iterable,
            block,
        } = node.as_fields();

        let in_token = in_token?;
        let iterable = iterable?;

        if is_each_list_iterable(&iterable) && bindings.len() > 0 {
            return write!(
                f,
                [
                    each_token.format(),
                    group(&format_args![
                        space(),
                        indent(&group(&iterable.format())),
                        soft_line_break_or_space()
                    ]),
                    block.format()
                ]
            );
        }

        write!(
            f,
            [
                each_token.format(),
                space(),
                bindings.format(),
                space(),
                in_token.format(),
                space(),
                iterable.format(),
                space(),
                block.format()
            ]
        )
    }
}

/// Returns `true` for list iterables that need the shared `@each` fill group.
fn is_each_list_iterable(iterable: &ScssExpression) -> bool {
    single_expression_item(iterable).is_some_and(|item| item.as_scss_list_expression().is_some())
}
