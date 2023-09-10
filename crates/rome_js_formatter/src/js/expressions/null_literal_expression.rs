use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::JsNullLiteralExpressionFields;
use biome_js_syntax::{JsNullLiteralExpression, JsSyntaxNode};
use rome_formatter::write;

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
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
