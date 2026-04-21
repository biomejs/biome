use crate::prelude::*;
use crate::utils::scss_map::scss_map_context;
use biome_css_syntax::{ScssListExpression, ScssListExpressionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpression;

impl FormatNodeRule<ScssListExpression> for FormatScssListExpression {
    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssListExpressionFields { elements } = node.as_fields();

        if scss_map_context(node).is_some_and(|context| context.is_outer_parenthesized_value_list) {
            let group_id = f.group_id("scss_list_expression");
            let comma = token(",");
            let trailing_comma = if_group_breaks(&comma).with_group_id(Some(group_id));

            // Format the list in `key: (a, b)`. The surrounding parentheses are
            // handled by `FormatScssParenthesizedExpression`.
            write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break(),
                    elements.format(),
                    trailing_comma
                ]))
                .with_group_id(Some(group_id))]
            )
        } else {
            write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break(),
                    elements.format()
                ]))]
            )
        }
    }
}
