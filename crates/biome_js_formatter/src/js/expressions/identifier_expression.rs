use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::ts::expressions::as_expression::TsAsOrSatisfiesExpression;
use biome_formatter::write;
use biome_js_syntax::{JsIdentifierExpression, JsSyntaxNode};
use biome_js_syntax::{JsIdentifierExpressionFields, JsSyntaxKind};
use biome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsIdentifierExpression;

impl FormatNodeRule<JsIdentifierExpression> for FormatJsIdentifierExpression {
    fn fmt_fields(&self, node: &JsIdentifierExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIdentifierExpressionFields { name } = node.as_fields();

        write![f, [name.format()]]
    }

    fn needs_parentheses(&self, item: &JsIdentifierExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsIdentifierExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        // edge case: handle cases such as
        // `(type) as unknown satisfies unknown`
        if TsAsOrSatisfiesExpression::can_cast(parent.kind())
            && parent
                .ancestors()
                .skip(1)
                .find(|x| !TsAsOrSatisfiesExpression::can_cast(x.kind()))
                .kind()
                == Some(JsSyntaxKind::JS_EXPRESSION_STATEMENT)
        {
            self.name()
                .and_then(|x| x.value_token())
                .map_or(false, |name| {
                    // These keywords are contextually reserved by TypeSCript in strict and sloppy modes.
                    matches!(
                        name.text_trimmed(),
                        "await" | "interface" | "let" | "module" | "type" | "yield" | "using"
                    )
                })
        } else {
            false
        }
    }
}
