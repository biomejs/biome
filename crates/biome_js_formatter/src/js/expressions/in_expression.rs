use crate::prelude::*;
use crate::utils::{needs_binary_like_parentheses, AnyJsBinaryLikeExpression};

use crate::parentheses::NeedsParentheses;

use biome_js_syntax::{AnyJsStatement, JsForStatement, JsInExpression, JsSyntaxNode};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsInExpression;

impl FormatNodeRule<JsInExpression> for FormatJsInExpression {
    fn fmt_fields(&self, node: &JsInExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        AnyJsBinaryLikeExpression::JsInExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsInExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsInExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if is_in_for_initializer(self) {
            return true;
        }

        needs_binary_like_parentheses(&AnyJsBinaryLikeExpression::from(self.clone()), parent)
    }
}

/// Add parentheses if the `in` is inside of a `for` initializer (see tests).
fn is_in_for_initializer(expression: &JsInExpression) -> bool {
    let mut current = expression.clone().into_syntax();

    while let Some(parent) = current.parent() {
        current = match JsForStatement::try_cast(parent) {
            Ok(for_statement) => {
                return for_statement
                    .initializer()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Some(&current);
            }
            Err(parent) => {
                if AnyJsStatement::can_cast(parent.kind()) {
                    // Don't cross statement boundaries
                    break;
                }

                parent
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{JsFileSource, JsInExpression};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class X extends (a in b) {}", JsInExpression);

        assert_needs_parentheses!("(a in b) as number", JsInExpression);
        assert_needs_parentheses!("<number>(a in b)", JsInExpression);
        assert_needs_parentheses!("!(a in b)", JsInExpression);
        assert_needs_parentheses!("await (a in b)", JsInExpression);
        assert_needs_parentheses!("(a in b)!", JsInExpression);

        assert_needs_parentheses!("(a in b)()", JsInExpression);
        assert_needs_parentheses!("(a in b)?.()", JsInExpression);
        assert_needs_parentheses!("new (a in b)()", JsInExpression);
        assert_needs_parentheses!("(a in b)`template`", JsInExpression);
        assert_needs_parentheses!("[...(a in b)]", JsInExpression);
        assert_needs_parentheses!("({...(a in b)})", JsInExpression);
        assert_needs_parentheses!(
            "<test {...(a in b)} />",
            JsInExpression,
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(a in b)}</test>",
            JsInExpression,
            JsFileSource::tsx()
        );

        assert_needs_parentheses!("(a in b).member", JsInExpression);
        assert_needs_parentheses!("(a in b)[member]", JsInExpression);
        assert_not_needs_parentheses!("object[a in b]", JsInExpression);

        assert_needs_parentheses!("(a in b) + c", JsInExpression);

        assert_not_needs_parentheses!("a in b > c", JsInExpression);
        assert_not_needs_parentheses!("a in b instanceof C", JsInExpression);
        assert_not_needs_parentheses!("a in b in c", JsInExpression[0]);
        assert_not_needs_parentheses!("a in b in c", JsInExpression[1]);
    }

    #[test]
    fn for_in_needs_parentheses() {
        assert_needs_parentheses!("for (let a = (b in c);;);", JsInExpression);
        assert_needs_parentheses!("for (a && (b in c);;);", JsInExpression);
        assert_needs_parentheses!("for (a => (b in c);;);", JsInExpression);
        assert_needs_parentheses!(
            "function* g() {
  for (yield (a in b);;);
}",
            JsInExpression
        );
        assert_needs_parentheses!(
            "async function f() {
  for (await (a in b);;);
}",
            JsInExpression
        );

        assert_not_needs_parentheses!("for (;a in b;);", JsInExpression);
        assert_not_needs_parentheses!("for (;;a in b);", JsInExpression);
        assert_not_needs_parentheses!(
            r#"
        for (function () { a in b }();;);
        "#,
            JsInExpression
        );
    }
}
