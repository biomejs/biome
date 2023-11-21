use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression, JsSyntaxNode};
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
        let Ok(name) = self.name().and_then(|x| x.value_token()) else {
            return false;
        };
        // In non-strict mode (sloppy mode), `let` may be a variable name.
        // To disambiguate `let` from a let-declaration,
        // we have to enclose `let` between parentheses in some dge cases.
        match parent.kind() {
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                // `(let)[0];`
                // `(let)[0] = 0;`
                // `for( (let)[0] = 0, b = 0;;;) {}`
                // `for( (let)[0] of []) {}`
                name.text_trimmed() == "let"
                    && parent
                        .ancestors()
                        .find(|x| {
                            !AnyJsExpression::can_cast(x.kind())
                                && !AnyJsMemberAssignment::can_cast(x.kind())
                        })
                        .filter(|x| {
                            matches!(
                                x.kind(),
                                JsSyntaxKind::JS_EXPRESSION_STATEMENT
                                    | JsSyntaxKind::JS_FOR_IN_STATEMENT
                                    | JsSyntaxKind::JS_FOR_OF_STATEMENT
                                    | JsSyntaxKind::JS_FOR_STATEMENT
                            )
                        })
                        .and_then(|x| x.first_child()?.first_token())
                        .is_some_and(|token| token == name)
            }
            JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                // for ( (let).a of [] ) {}
                name.text_trimmed() == "let"
                    && parent
                        .ancestors()
                        .find(|x| {
                            !AnyJsExpression::can_cast(x.kind())
                                && !AnyJsMemberAssignment::can_cast(x.kind())
                        })
                        .filter(|x| x.kind() == JsSyntaxKind::JS_FOR_OF_STATEMENT)
                        .and_then(|x| x.first_child()?.first_token())
                        .is_some_and(|token| token == name)
            }
            JsSyntaxKind::TS_AS_EXPRESSION | JsSyntaxKind::TS_SATISFIES_EXPRESSION => {
                // `(let) as unknown satisfies unknown;`
                // `(type) as unknown satisfies unknown;`
                matches!(
                    name.text_trimmed(),
                    "await" | "interface" | "let" | "module" | "type" | "using" | "yield"
                ) && parent
                    .ancestors()
                    .skip(1)
                    .find(|x| {
                        !matches!(
                            x.kind(),
                            JsSyntaxKind::TS_AS_EXPRESSION | JsSyntaxKind::TS_SATISFIES_EXPRESSION
                        )
                    })
                    .kind()
                    == Some(JsSyntaxKind::JS_EXPRESSION_STATEMENT)
            }
            _ => false,
        }
    }
}
