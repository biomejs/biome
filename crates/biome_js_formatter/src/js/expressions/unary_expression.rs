use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{AnyJsExpression, JsUnaryExpression};
use biome_js_syntax::{JsUnaryExpressionFields, JsUnaryOperator};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnaryExpression;

impl FormatNodeRule<JsUnaryExpression> for FormatJsUnaryExpression {
    fn fmt_fields(&self, node: &JsUnaryExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsUnaryExpressionFields {
            operator_token,
            argument,
        } = node.as_fields();

        let operation = node.operator()?;
        let operator_token = operator_token?;
        let argument = argument?;

        write!(f, [operator_token.format()])?;

        let is_keyword_operator = matches!(
            operation,
            JsUnaryOperator::Delete | JsUnaryOperator::Void | JsUnaryOperator::Typeof
        );

        if is_keyword_operator {
            write!(f, [space()])?;
        }

        // Add delimiter spacing after logical not operator (!) when enabled
        // Only add space after the LAST ! in a chain (e.g., !! a, not ! ! a)
        let should_insert_space = f.options().delimiter_spacing().value();
        let is_logical_not = operation == JsUnaryOperator::LogicalNot;

        // Check if the argument is also a logical not - if so, don't add space here
        let argument_is_logical_not = matches!(
            &argument,
            AnyJsExpression::JsUnaryExpression(unary) if unary.operator() == Ok(JsUnaryOperator::LogicalNot)
        );

        // Only add delimiter spacing if this is logical not AND arg is NOT another logical not
        let add_delimiter_space = is_logical_not && should_insert_space && !argument_is_logical_not;

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            if add_delimiter_space {
                write!(
                    f,
                    [group(&format_args![
                        space(),
                        token("("),
                        soft_block_indent_with_maybe_space(&argument.format(), should_insert_space),
                        token(")")
                    ])]
                )
            } else {
                write!(
                    f,
                    [group(&format_args![
                        token("("),
                        soft_block_indent(&argument.format()),
                        token(")")
                    ])]
                )
            }
        } else if add_delimiter_space {
            // Add space after logical not operator
            write![f, [space(), argument.format()]]
        } else {
            write![f, [argument.format()]]
        }
    }

    fn needs_parentheses(&self, item: &JsUnaryExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsUnaryExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class A extends (!B) {}", JsUnaryExpression);

        assert_needs_parentheses!("(!foo) instanceof Bar", JsUnaryExpression);
        assert_needs_parentheses!("(!foo) instanceof Bar", JsUnaryExpression);
        assert_needs_parentheses!("(~foo) in bar", JsUnaryExpression);
        assert_needs_parentheses!("(~foo) in bar", JsUnaryExpression);

        assert_needs_parentheses!("(+a).b", JsUnaryExpression);
        assert_needs_parentheses!("(+a)[b]", JsUnaryExpression);
        assert_not_needs_parentheses!("a[+b]", JsUnaryExpression);

        assert_needs_parentheses!("(+a)`template`", JsUnaryExpression);

        assert_needs_parentheses!("(+a)()", JsUnaryExpression);
        assert_needs_parentheses!("new (+a)()", JsUnaryExpression);

        assert_needs_parentheses!("(+a)!", JsUnaryExpression);

        assert_needs_parentheses!("(+a) ** 3", JsUnaryExpression);
        assert_not_needs_parentheses!("(+a) + 3", JsUnaryExpression);
    }
}
