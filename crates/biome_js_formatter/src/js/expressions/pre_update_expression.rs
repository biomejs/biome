use crate::parentheses::{unary_like_expression_needs_parentheses, NeedsParentheses};
use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsPreUpdateExpressionFields;
use biome_js_syntax::{
    JsPreUpdateExpression, JsPreUpdateOperator, JsUnaryExpression, JsUnaryOperator,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPreUpdateExpression;

impl FormatNodeRule<JsPreUpdateExpression> for FormatJsPreUpdateExpression {
    fn fmt_fields(&self, node: &JsPreUpdateExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = node.as_fields();

        write![f, [operator_token.format(), operand.format(),]]
    }

    fn needs_parentheses(&self, item: &JsPreUpdateExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsPreUpdateExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(unary) = self.parent::<JsUnaryExpression>() {
            let parent_operator = unary.operator();
            let operator = self.operator();
            (parent_operator == Ok(JsUnaryOperator::Plus)
                && operator == Ok(JsPreUpdateOperator::Increment))
                || (parent_operator == Ok(JsUnaryOperator::Minus)
                    && operator == Ok(JsPreUpdateOperator::Decrement))
        } else {
            unary_like_expression_needs_parentheses(self.syntax())
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsPreUpdateExpression;

    #[test]
    fn needs_parentheses() {
        // valid, but should become +(++a)
        assert_needs_parentheses!("+ ++a", JsPreUpdateExpression);
        assert_needs_parentheses!("class A extends (++A) {}", JsPreUpdateExpression);

        assert_needs_parentheses!("(++a).b", JsPreUpdateExpression);
        assert_needs_parentheses!("(++a)[b]", JsPreUpdateExpression);
        assert_not_needs_parentheses!("a[++b]", JsPreUpdateExpression);

        assert_needs_parentheses!("(++a)`template`", JsPreUpdateExpression);

        assert_needs_parentheses!("(++a)()", JsPreUpdateExpression);
        assert_needs_parentheses!("new (++a)()", JsPreUpdateExpression);

        assert_needs_parentheses!("(++a)!", JsPreUpdateExpression);

        assert_needs_parentheses!("(++a) ** 3", JsPreUpdateExpression);
        assert_not_needs_parentheses!("(++a) + 3", JsPreUpdateExpression);
    }
}
