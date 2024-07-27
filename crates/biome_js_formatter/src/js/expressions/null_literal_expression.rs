use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsNullLiteralExpression;
use biome_js_syntax::JsNullLiteralExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNullLiteralExpression;

impl FormatNodeRule<JsNullLiteralExpression> for FormatJsNullLiteralExpression {
    fn fmt_fields(&self, node: &JsNullLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &JsNullLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsNullLiteralExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
}
