use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{AnyJsComputedMember, JsAwaitExpression};
use biome_js_syntax::{JsAwaitExpressionFields, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAwaitExpression;

impl FormatNodeRule<JsAwaitExpression> for FormatJsAwaitExpression {
    fn fmt_fields(&self, node: &JsAwaitExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = node.as_fields();

        let format_inner =
            format_with(|f| write![f, [await_token.format(), space(), argument.format()]]);

        let parent = node.syntax().parent();

        if let Some(parent) = parent {
            let is_callee_or_object = match parent.kind() {
                // Callee
                JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_NEW_EXPRESSION
                // Static member
                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => true,
                _ => AnyJsComputedMember::cast_ref(&parent)
                    .and_then(|member| member.object().ok())
                    .is_some_and(|object| object.syntax() == node.syntax()),
            };
            if is_callee_or_object {
                let ancestor_await_or_block = parent.ancestors().skip(1).find(|ancestor| {
                    matches!(
                        ancestor.kind(),
                        JsSyntaxKind::JS_AWAIT_EXPRESSION
                            // Stop at statement boundaries.
                            | JsSyntaxKind::JS_STATEMENT_LIST
                            | JsSyntaxKind::JS_MODULE_ITEM_LIST
                    )
                });

                let indented = format_with(|f| write!(f, [soft_block_indent(&format_inner)]));

                return if let Some(parent_js_await_expression) =
                    ancestor_await_or_block.and_then(JsAwaitExpression::cast)
                {
                    if !parent_js_await_expression.needs_parentheses() {
                        return write!(f, [group(&indented)]);
                    }

                    write!(f, [indented])
                } else {
                    write!(f, [group(&indented)])
                };
            }
        }

        write!(f, [format_inner])
    }

    fn needs_parentheses(&self, item: &JsAwaitExpression) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsAwaitExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(await a)`template`", JsAwaitExpression);
        assert_needs_parentheses!("+(await a)", JsAwaitExpression);

        assert_needs_parentheses!("(await a).b", JsAwaitExpression);
        assert_needs_parentheses!("(await a)[b]", JsAwaitExpression);
        assert_not_needs_parentheses!("a[await b]", JsAwaitExpression);

        assert_needs_parentheses!("(await a)()", JsAwaitExpression);
        assert_needs_parentheses!("new (await a)()", JsAwaitExpression);

        assert_needs_parentheses!("(await a) && b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) + b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) instanceof b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) in b", JsAwaitExpression);

        assert_needs_parentheses!("[...(await a)]", JsAwaitExpression);
        assert_needs_parentheses!("({...(await b)})", JsAwaitExpression);
        assert_needs_parentheses!("call(...(await b))", JsAwaitExpression);

        assert_needs_parentheses!("class A extends (await b) {}", JsAwaitExpression);

        assert_needs_parentheses!("(await b) as number", JsAwaitExpression);
        assert_needs_parentheses!("(await b)!", JsAwaitExpression);

        assert_needs_parentheses!("(await b) ? b : c", JsAwaitExpression);
        assert_not_needs_parentheses!("a ? await b : c", JsAwaitExpression);
        assert_not_needs_parentheses!("a ? b : await c", JsAwaitExpression);
    }
}
