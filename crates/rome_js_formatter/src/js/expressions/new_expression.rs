use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsNewExpression, JsSyntaxKind};
use biome_js_syntax::{JsNewExpressionFields, JsSyntaxNode};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNewExpression;

impl FormatNodeRule<JsNewExpression> for FormatJsNewExpression {
    fn fmt_fields(&self, node: &JsNewExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNewExpressionFields {
            new_token,
            callee,
            type_arguments,
            arguments,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                space(),
                callee.format(),
                type_arguments.format(),
            ]
        ]?;

        match arguments {
            Some(arguments) => {
                write!(f, [arguments.format()])
            }
            None => {
                write!(f, [text("("), text(")")])
            }
        }
    }

    fn needs_parentheses(&self, item: &JsNewExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsNewExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_EXTENDS_CLAUSE)
    }
}
