use crate::prelude::*;
use biome_css_syntax::{
    CssListOfComponentValuesExpression, CssListOfComponentValuesExpressionFields, CssParameterList,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssListOfComponentValuesExpression;
impl FormatNodeRule<CssListOfComponentValuesExpression>
    for FormatCssListOfComponentValuesExpression
{
    fn fmt_node(
        &self,
        node: &CssListOfComponentValuesExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if is_function_argument_with_leading_comments(node, f) {
            // Keep the comment and function aligned. Only breaks inside the
            // function inherit the additional argument-body indent.
            write!(
                f,
                [
                    format_leading_comments(node.syntax()).with_following_content(|f| {
                        let formatted = format_with(|f| self.fmt_fields(node, f));
                        write!(f, [indent(&formatted)])
                    })
                ]
            )
        } else {
            self.fmt_fields(node, f)
        }
    }

    fn fmt_fields(
        &self,
        node: &CssListOfComponentValuesExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssListOfComponentValuesExpressionFields {
            css_component_value_list,
        } = node.as_fields();

        write!(f, [css_component_value_list.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &CssListOfComponentValuesExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if is_function_argument_with_leading_comments(node, f) {
            Ok(())
        } else {
            write!(f, [format_leading_comments(node.syntax())])
        }
    }
}

fn is_function_argument_with_leading_comments(
    node: &CssListOfComponentValuesExpression,
    f: &CssFormatter,
) -> bool {
    if node.parent::<CssParameterList>().is_none()
        || !f.comments().has_leading_comments(node.syntax())
    {
        return false;
    }

    let mut values = node.css_component_value_list().iter();
    values
        .next()
        .is_some_and(|value| value.as_any_css_function().is_some())
        && values.next().is_none()
}
