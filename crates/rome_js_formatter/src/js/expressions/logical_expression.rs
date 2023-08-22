use crate::prelude::*;
use crate::utils::{needs_binary_like_parentheses, AnyJsBinaryLikeExpression};

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{JsLogicalExpression, JsSyntaxNode};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLogicalExpression;

impl FormatNodeRule<JsLogicalExpression> for FormatJsLogicalExpression {
    fn fmt_fields(
        &self,
        node: &JsLogicalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsBinaryLikeExpression::JsLogicalExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsLogicalExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsLogicalExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if let Some(parent) = JsLogicalExpression::cast_ref(parent) {
            parent.operator() != self.operator()
        } else {
            needs_binary_like_parentheses(&AnyJsBinaryLikeExpression::from(self.clone()), parent)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{JsFileSource, JsLogicalExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class X extends (a && b) {}", JsLogicalExpression);

        assert_needs_parentheses!("(a && b) as number", JsLogicalExpression);
        assert_needs_parentheses!("<number>(a && b)", JsLogicalExpression);
        assert_needs_parentheses!("!(a && b)", JsLogicalExpression);
        assert_needs_parentheses!("await (a && b)", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)!", JsLogicalExpression);

        assert_needs_parentheses!("(a && b)()", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)?.()", JsLogicalExpression);
        assert_needs_parentheses!("new (a && b)()", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)`template`", JsLogicalExpression);
        assert_needs_parentheses!("[...(a && b)]", JsLogicalExpression);
        assert_needs_parentheses!("({...(a && b)})", JsLogicalExpression);
        assert_needs_parentheses!(
            "<test {...(a && b)} />",
            JsLogicalExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(a && b)}</test>",
            JsLogicalExpression,
            JsFileSource::tsx()
        );

        assert_needs_parentheses!("(a && b).member", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)[member]", JsLogicalExpression);
        assert_not_needs_parentheses!("object[a && b]", JsLogicalExpression);

        assert_needs_parentheses!("(a && b) || c", JsLogicalExpression[1]);
        assert_needs_parentheses!("(a && b) in c", JsLogicalExpression);
        assert_needs_parentheses!("(a && b) instanceof c", JsLogicalExpression);
        assert_needs_parentheses!("(a && b) + c", JsLogicalExpression);

        assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[0]);
        assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[1]);
    }
}
