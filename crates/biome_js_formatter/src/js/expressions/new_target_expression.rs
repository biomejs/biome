use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsNewTargetExpression;
use biome_js_syntax::JsNewTargetExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNewTargetExpression;

impl FormatNodeRule<JsNewTargetExpression> for FormatJsNewTargetExpression {
    fn fmt_fields(&self, node: &JsNewTargetExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNewTargetExpressionFields {
            new_token,
            dot_token,
            target_token,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                dot_token.format(),
                target_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &JsNewTargetExpression) -> bool {
        item.needs_parentheses()
    }
}
