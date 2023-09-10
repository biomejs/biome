use crate::prelude::*;
use crate::utils::{AnyJsConditional, ConditionalJsxChain};
use rome_formatter::FormatRuleWithOptions;

use crate::parentheses::{
    is_binary_like_left_or_right, is_conditional_test, is_spread,
    update_or_lower_expression_needs_parentheses, NeedsParentheses,
};
use biome_js_syntax::{JsConditionalExpression, JsSyntaxKind, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConditionalExpression {
    jsx_chain: ConditionalJsxChain,
}

impl FormatRuleWithOptions<JsConditionalExpression> for FormatJsConditionalExpression {
    type Options = ConditionalJsxChain;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.jsx_chain = options;
        self
    }
}

impl FormatNodeRule<JsConditionalExpression> for FormatJsConditionalExpression {
    fn fmt_fields(
        &self,
        node: &JsConditionalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsConditional::from(node.clone())
            .format()
            .with_options(self.jsx_chain)
            .fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsConditionalExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsConditionalExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION => true,

            _ => {
                is_conditional_test(self.syntax(), parent)
                    || update_or_lower_expression_needs_parentheses(self.syntax(), parent)
                    || is_binary_like_left_or_right(self.syntax(), parent)
                    || is_spread(self.syntax(), parent)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsConditionalExpression, JsFileSource};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("((a ? b : c)).member", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c).member", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c)[member]", JsConditionalExpression);
        assert_not_needs_parentheses!("object[(a ? b : c)]", JsConditionalExpression);

        assert_needs_parentheses!("new (true ? A : B)()", JsConditionalExpression);
        assert_not_needs_parentheses!("new A(a ? b : c)", JsConditionalExpression);

        assert_needs_parentheses!("(true ? A : B)()", JsConditionalExpression);
        assert_not_needs_parentheses!("call(a ? b : c)", JsConditionalExpression);

        assert_needs_parentheses!(
            "(a ? b : c)`tagged template literal`",
            JsConditionalExpression
        );
        assert_not_needs_parentheses!("tag`content ${a ? b : c}`", JsConditionalExpression);

        assert_needs_parentheses!("-(a ? b : c)", JsConditionalExpression);

        assert_needs_parentheses!("[...(a ? b : c)]", JsConditionalExpression);
        assert_needs_parentheses!("({...(a ? b : c)})", JsConditionalExpression);
        assert_needs_parentheses!("call(...(a ? b : c))", JsConditionalExpression);

        assert_needs_parentheses!("a + (b ? c : d)", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c) + d", JsConditionalExpression);

        assert_needs_parentheses!("a instanceof (b ? c : d)", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c) instanceof d", JsConditionalExpression);

        assert_needs_parentheses!("a in (b ? c : d)", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c) in d", JsConditionalExpression);

        assert_needs_parentheses!("a && (b ? c : d)", JsConditionalExpression);
        assert_needs_parentheses!("(a ? b : c) && d", JsConditionalExpression);

        assert_needs_parentheses!("await (a ? b : c)", JsConditionalExpression);
        assert_needs_parentheses!(
            "<a {...(a ? b : c)} />",
            JsConditionalExpression,
            JsFileSource::tsx()
        );

        assert_needs_parentheses!("(a ? b : c) as number;", JsConditionalExpression);
        assert_needs_parentheses!("<number>(a ? b : c);", JsConditionalExpression);

        assert_needs_parentheses!("class Test extends (a ? B : C) {}", JsConditionalExpression);
        assert_needs_parentheses!("(a ? B : C)!", JsConditionalExpression);

        assert_not_needs_parentheses!("a ? b : c", JsConditionalExpression);
        assert_not_needs_parentheses!("(a ? b : c)", JsConditionalExpression);

        assert_needs_parentheses!("({ a: 'test' } ? B : C)!", JsConditionalExpression);
        assert_not_needs_parentheses!(
            "console.log({ a: 'test' } ? B : C )",
            JsConditionalExpression
        );
        assert_not_needs_parentheses!("a ? b : c", JsConditionalExpression);
    }
}
