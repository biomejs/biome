use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsBooleanLiteralExpression;
use biome_js_syntax::JsBooleanLiteralExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBooleanLiteralExpression;

impl FormatNodeRule<JsBooleanLiteralExpression> for FormatJsBooleanLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsBooleanLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBooleanLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsBooleanLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
