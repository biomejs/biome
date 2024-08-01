use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsImportCallExpression;
use biome_js_syntax::JsImportCallExpressionFields;

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
