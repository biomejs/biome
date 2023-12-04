use crate::parentheses::{is_first_in_statement, FirstInStatementMode, NeedsParentheses};
use crate::prelude::*;
use crate::utils::JsObjectLike;
use biome_formatter::write;
use biome_js_syntax::{JsObjectExpression, JsSyntaxKind, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectExpression;

impl FormatNodeRule<JsObjectExpression> for FormatJsObjectExpression {
    fn fmt_fields(&self, node: &JsObjectExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &JsObjectExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsObjectExpression,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `JsObjectLike`
        Ok(())
    }
}

impl NeedsParentheses for JsObjectExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
            || is_first_in_statement(
                self.clone().into(),
                FirstInStatementMode::ExpressionStatementOrArrow,
            )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsObjectExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class A extends ({}) {}", JsObjectExpression);
        assert_needs_parentheses!("({a: 5})", JsObjectExpression);
        assert_needs_parentheses!("a => ({ a: 5})", JsObjectExpression);

        assert_needs_parentheses!("a => ({ a: 'test' })`template`", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }).member", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' })[member]", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' })()", JsObjectExpression);
        assert_needs_parentheses!("new ({ a: 'test' })()", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }) as number", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' })!", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }), b, c", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }) + 5", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }) && true", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }) instanceof A", JsObjectExpression);
        assert_needs_parentheses!("({ a: 'test' }) in B", JsObjectExpression);

        assert_not_needs_parentheses!("() => ({}, a);", JsObjectExpression);
        assert_not_needs_parentheses!("() => (a = {});", JsObjectExpression);
        assert_not_needs_parentheses!("() => ({}.prop = 0);", JsObjectExpression);
        assert_not_needs_parentheses!("() => ({}['prop'] = 0);", JsObjectExpression);
    }
}
