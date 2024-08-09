use crate::prelude::*;

use biome_js_syntax::binary_like_expression::AnyJsBinaryLikeExpression;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsInstanceofExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsInstanceofExpression;

impl FormatNodeRule<JsInstanceofExpression> for FormatJsInstanceofExpression {
    fn fmt_fields(
        &self,
        node: &JsInstanceofExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsBinaryLikeExpression::JsInstanceofExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsInstanceofExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsFileSource, JsInstanceofExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!(
            "class X extends (a instanceof b) {}",
            JsInstanceofExpression
        );

        assert_needs_parentheses!("(a instanceof B) as number", JsInstanceofExpression);
        assert_needs_parentheses!("<number>(a instanceof B)", JsInstanceofExpression);
        assert_needs_parentheses!("!(a instanceof B)", JsInstanceofExpression);
        assert_needs_parentheses!("await (a instanceof B)", JsInstanceofExpression);
        assert_needs_parentheses!("(a instanceof B)!", JsInstanceofExpression);

        assert_needs_parentheses!("(a instanceof B)()", JsInstanceofExpression);
        assert_needs_parentheses!("(a instanceof B)?.()", JsInstanceofExpression);
        assert_needs_parentheses!("new (a instanceof B)()", JsInstanceofExpression);
        assert_needs_parentheses!("(a instanceof B)`template`", JsInstanceofExpression);
        assert_needs_parentheses!("[...(a instanceof B)]", JsInstanceofExpression);
        assert_needs_parentheses!("({...(a instanceof B)})", JsInstanceofExpression);
        assert_needs_parentheses!(
            "<test {...(a instanceof B)} />",
            JsInstanceofExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(a instanceof B)}</test>",
            JsInstanceofExpression,
            JsFileSource::tsx()
        );

        assert_needs_parentheses!("(a instanceof B).member", JsInstanceofExpression);
        assert_needs_parentheses!("(a instanceof B)[member]", JsInstanceofExpression);
        assert_not_needs_parentheses!("object[a instanceof B]", JsInstanceofExpression);

        assert_needs_parentheses!("(a instanceof B) + c", JsInstanceofExpression);

        assert_not_needs_parentheses!("a instanceof B > c", JsInstanceofExpression);
        assert_not_needs_parentheses!("a instanceof B in c", JsInstanceofExpression);
        assert_not_needs_parentheses!("a instanceof B instanceof c", JsInstanceofExpression[0]);
        assert_not_needs_parentheses!("a instanceof B instanceof c", JsInstanceofExpression[1]);
    }
}
