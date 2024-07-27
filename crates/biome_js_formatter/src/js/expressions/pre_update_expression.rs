use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsPreUpdateExpression;
use biome_js_syntax::JsPreUpdateExpressionFields;

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
