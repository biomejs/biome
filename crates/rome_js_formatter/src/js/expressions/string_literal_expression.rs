use crate::prelude::*;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_js_syntax::{JsExpressionStatement, JsSyntaxKind};
use rome_js_syntax::{JsStringLiteralExpression, JsSyntaxNode};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStringLiteralExpression;

impl FormatNodeRule<JsStringLiteralExpression> for FormatJsStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsStringLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let formatted =
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression);

        formatted.fmt(f)
    }

    fn needs_parentheses(&self, item: &JsStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsStringLiteralExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if let Some(expression_statement) = JsExpressionStatement::cast_ref(parent) {
            return expression_statement
                .syntax()
                .parent()
                .map_or(false, |grand_parent| {
                    matches!(
                        grand_parent.kind(),
                        JsSyntaxKind::JS_STATEMENT_LIST | JsSyntaxKind::JS_MODULE_ITEM_LIST
                    )
                });
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{JsFileSource, JsStringLiteralExpression, ModuleKind};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("{ 'test'; }", JsStringLiteralExpression);
        assert_needs_parentheses!(
            r#"
            {
                console.log(5);
                'test';
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            r#"
            function Test () {
                ('test');
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            r#"
            class A {
                static {
                    ('test');
                }
            }
            "#,
            JsStringLiteralExpression
        );
        assert_needs_parentheses!(
            "('test');",
            JsStringLiteralExpression,
            JsFileSource::ts().with_module_kind(ModuleKind::Module)
        );

        assert_not_needs_parentheses!("console.log('a')", JsStringLiteralExpression);
    }
}
