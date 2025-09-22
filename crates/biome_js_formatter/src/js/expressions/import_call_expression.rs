use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportCallExpression;
use biome_js_syntax::JsImportCallExpressionFields;
use biome_js_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportCallExpression;

impl FormatNodeRule<JsImportCallExpression> for FormatJsImportCallExpression {
    fn fmt_fields(&self, node: &JsImportCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportCallExpressionFields {
            import_token,
            dot_token,
            phase,
            arguments,
        } = node.as_fields();

        write!(f, [import_token.format()])?;

        if let Some(dot_token) = dot_token {
            write!(f, [dot_token.format()])?;
        }

        if let Some(phase) = phase {
            write!(f, [phase.format()])?;
        }

        write![f, [arguments.format()]]
    }

    fn needs_parentheses(&self, item: &JsImportCallExpression) -> bool {
        item.needs_parentheses()
    }
}
