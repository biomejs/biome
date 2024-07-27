use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::JsNewExpressionFields;
use biome_js_syntax::{JsNewExpression, JsSyntaxKind};

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
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .is_some_and(|node| node.kind() == JsSyntaxKind::JS_EXTENDS_CLAUSE)
    }
}
