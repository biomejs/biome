use crate::prelude::*;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    CssParameterList, ScssExpression, ScssExpressionFields, is_in_scss_include_arguments,
    single_expression_item,
};
use biome_formatter::{FormatResult, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_node(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        if is_commented_function_argument(node, f) {
            // The leading-comment hook emits its break before `fmt_node` runs.
            // Starting the indent here keeps the first token aligned with the
            // comment while nested breaks retain the argument indent.
            let formatted = format_with(|f| self.fmt_fields(node, f));
            write!(f, [indent(&formatted)])
        } else {
            self.fmt_node_with_scss_separator_comments(node, f)
        }
    }

    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExpressionFields { items } = node.as_fields();

        write!(f, [items.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }
}

fn is_commented_function_argument(node: &ScssExpression, f: &CssFormatter) -> bool {
    node.parent::<CssParameterList>().is_some()
        && f.comments().has_leading_comments(node.syntax())
        && single_expression_item(node).is_some_and(|item| {
            item.as_any_css_value()
                .is_some_and(|value| value.as_any_css_function().is_some())
        })
        && !is_in_scss_include_arguments(node.syntax())
}
