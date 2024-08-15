use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsAssignmentExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAssignmentExpression;

impl FormatNodeRule<JsAssignmentExpression> for FormatJsAssignmentExpression {
    fn fmt_fields(&self, node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [AnyJsAssignmentLike::from(node.clone())]]
    }

    fn needs_parentheses(&self, item: &JsAssignmentExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsAssignmentExpression;

    #[test]
    fn needs_parentheses() {
        assert_not_needs_parentheses!("({ [a = 3]: value })", JsAssignmentExpression);
        assert_not_needs_parentheses!("class Test { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("type Test  = { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("interface Test { [a = 3]: value }", JsAssignmentExpression);

        assert_needs_parentheses!("a => (a = 3)", JsAssignmentExpression);
        assert_not_needs_parentheses!("a => { a = 3 }", JsAssignmentExpression);

        assert_not_needs_parentheses!("for(a = 3;;) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(a = 3, b = 2;;) {}", JsAssignmentExpression[1]);
        assert_not_needs_parentheses!("for(a = 3, b = 2, c= 3;;) {}", JsAssignmentExpression[2]);
        assert_needs_parentheses!("for(; a = 3; ) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(;;a = 3) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("for ((a, a = 3);;) {}", JsAssignmentExpression);
        assert_needs_parentheses!("for (; (a, a = 3);) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for (;;(a, a = 3)) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("a = 3", JsAssignmentExpression);
        assert_needs_parentheses!("({ a } = { a: 3 })", JsAssignmentExpression);
    }
}
