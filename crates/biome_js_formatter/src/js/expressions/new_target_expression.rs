use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsNewTargetExpressionFields;
use biome_js_syntax::{JsNewTargetExpression, JsSyntaxNode};

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

impl NeedsParentheses for JsNewTargetExpression {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
