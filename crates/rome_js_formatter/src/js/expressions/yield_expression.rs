use crate::prelude::*;
use biome_formatter::write;

use crate::js::expressions::await_expression::await_or_yield_needs_parens;
use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxKind, JsYieldExpressionFields};
use biome_js_syntax::{JsSyntaxNode, JsYieldExpression};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsYieldExpression;

impl FormatNodeRule<JsYieldExpression> for FormatJsYieldExpression {
    fn fmt_fields(&self, node: &JsYieldExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsYieldExpressionFields {
            yield_token,
            argument,
        } = node.as_fields();

        write![f, [yield_token.format(), argument.format()]]
    }

    fn needs_parentheses(&self, item: &JsYieldExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsYieldExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_AWAIT_EXPRESSION)
            || await_or_yield_needs_parens(parent, self.syntax())
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsYieldExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!(
            "function* test() { (yield a)`template` }",
            JsYieldExpression
        );
        assert_needs_parentheses!("function* test() { +(yield a) }", JsYieldExpression);

        assert_needs_parentheses!("function* test() { (yield a).b }", JsYieldExpression);
        assert_needs_parentheses!("function* test() { (yield a)[b] }", JsYieldExpression);
        assert_not_needs_parentheses!("function* test() { a[yield b] }", JsYieldExpression);

        assert_needs_parentheses!("function* test() { (yield a)() }", JsYieldExpression);
        assert_needs_parentheses!("function* test() { new (yield a)() }", JsYieldExpression);

        assert_needs_parentheses!("function* test() { (yield a) && b }", JsYieldExpression);
        assert_needs_parentheses!("function* test() { (yield a) + b }", JsYieldExpression);
        assert_needs_parentheses!(
            "function* test() { (yield a) instanceof b }",
            JsYieldExpression
        );
        assert_needs_parentheses!("function* test() { (yield a) in b }", JsYieldExpression);

        assert_needs_parentheses!("function* test() { [...(yield a)] }", JsYieldExpression);
        assert_needs_parentheses!("function* test() { ({...(yield b)}) }", JsYieldExpression);
        assert_needs_parentheses!("function* test() { call(...(yield b)) }", JsYieldExpression);

        assert_needs_parentheses!(
            "function* test() { class A extends (yield b) {} }",
            JsYieldExpression
        );

        assert_needs_parentheses!(
            "function* test() { (yield b) as number }",
            JsYieldExpression
        );
        assert_needs_parentheses!("function* test() { (yield b)! }", JsYieldExpression);

        assert_needs_parentheses!("function* test() { (yield b) ? b : c }", JsYieldExpression);
        assert_not_needs_parentheses!("function* test() { a ? yield b : c }", JsYieldExpression);
        assert_not_needs_parentheses!("function* test() { a ? b : yield c }", JsYieldExpression);
    }
}
