use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsBooleanLiteralExpressionFields;
use biome_js_syntax::{JsBooleanLiteralExpression, JsSyntaxNode};

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

impl NeedsParentheses for JsBooleanLiteralExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
