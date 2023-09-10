use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_rowan::AstNode;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsSyntaxKind::JS_SEQUENCE_EXPRESSION;
use rome_js_syntax::{
    JsSequenceExpression, JsSequenceExpressionFields, JsSyntaxKind, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsSequenceExpression;

impl FormatNodeRule<JsSequenceExpression> for FormatJsSequenceExpression {
    fn fmt_fields(&self, node: &JsSequenceExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = node.as_fields();

        let mut is_nested = false;
        let mut first_non_sequence_or_paren_parent = None;

        // Skip 1 because ancestor starts with the current node but we're interested in the parent
        for parent in node.syntax().ancestors().skip(1) {
            if parent.kind() == JS_SEQUENCE_EXPRESSION {
                is_nested = true;
            } else {
                first_non_sequence_or_paren_parent = Some(parent);
                break;
            }
        }

        let format_inner = format_with(|f| {
            if let Some(parent) = &first_non_sequence_or_paren_parent {
                if matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_EXPRESSION_STATEMENT | JsSyntaxKind::JS_FOR_STATEMENT
                ) {
                    return write!(
                        f,
                        [
                            left.format(),
                            comma_token.format(),
                            line_suffix_boundary(),
                            indent(&format_args![soft_line_break_or_space(), right.format()])
                        ]
                    );
                }
            }

            write!(
                f,
                [
                    left.format(),
                    comma_token.format(),
                    line_suffix_boundary(),
                    soft_line_break_or_space(),
                    right.format()
                ]
            )
        });

        if is_nested {
            write!(f, [format_inner])
        } else {
            write!(f, [group(&format_inner)])
        }
    }

    fn needs_parentheses(&self, item: &JsSequenceExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsSequenceExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        !matches!(
            parent.kind(),
            JsSyntaxKind::JS_RETURN_STATEMENT |
            // There's a precedence for writing `x++, y++`
            JsSyntaxKind::JS_FOR_STATEMENT |
            JsSyntaxKind::JS_EXPRESSION_STATEMENT |
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION  |
            // Handled as part of the arrow function formatting
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_not_needs_parentheses;
    use rome_js_syntax::JsSequenceExpression;

    #[test]
    fn needs_parentheses() {
        assert_not_needs_parentheses!("function test() { return a, b }", JsSequenceExpression);
        assert_not_needs_parentheses!("for (let i, x; i++, x++;) {}", JsSequenceExpression);
        assert_not_needs_parentheses!("a, b;", JsSequenceExpression);
        assert_not_needs_parentheses!("a, b, c", JsSequenceExpression[0]);
        assert_not_needs_parentheses!("a, b, c", JsSequenceExpression[1]);
        assert_not_needs_parentheses!("a => a, b", JsSequenceExpression);
    }
}
