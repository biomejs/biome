use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsYieldExpression;
use biome_js_syntax::JsYieldExpressionFields;

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
