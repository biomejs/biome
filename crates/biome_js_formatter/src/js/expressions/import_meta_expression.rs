use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsImportMetaExpressionFields;
use biome_js_syntax::{JsImportMetaExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportMetaExpression;

impl FormatNodeRule<JsImportMetaExpression> for FormatJsImportMetaExpression {
    fn fmt_fields(&self, node: &JsImportMetaExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportMetaExpressionFields {
            import_token,
            dot_token,
            meta_token,
        } = node.as_fields();

        write![
            f,
            [
                import_token.format(),
                dot_token.format(),
                meta_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &JsImportMetaExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsImportMetaExpression {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
