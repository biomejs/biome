use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsImportCallExpressionFields;
use biome_js_syntax::{JsImportCallExpression, JsSyntaxKind, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCallExpression;

impl FormatNodeRule<JsImportCallExpression> for FormatJsImportCallExpression {
    fn fmt_fields(&self, node: &JsImportCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallExpressionFields {
            import_token,
            arguments,
        } = node.as_fields();

        write![f, [import_token.format(), arguments.format()]]
    }

    fn needs_parentheses(&self, item: &JsImportCallExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsImportCallExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_NEW_EXPRESSION)
    }
}
