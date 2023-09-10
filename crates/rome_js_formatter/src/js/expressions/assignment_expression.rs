use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;

use crate::parentheses::{
    is_arrow_function_body, is_first_in_statement, FirstInStatementMode, NeedsParentheses,
};
use rome_formatter::write;

use biome_rowan::{match_ast, AstNode};
use rome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsForInitializer, JsArrowFunctionExpression, JsAssignmentExpression,
    JsComputedMemberName, JsExpressionStatement, JsForStatement, JsSequenceExpression,
    JsSyntaxKind, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAssignmentExpression;

impl FormatNodeRule<JsAssignmentExpression> for FormatJsAssignmentExpression {
    fn fmt_fields(&self, node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [AnyJsAssignmentLike::from(node.clone())]]
    }

    fn needs_parentheses(&self, item: &JsAssignmentExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsAssignmentExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match_ast! {
            match parent {
                JsAssignmentExpression(_) => false,
                // `[a = b]`
                JsComputedMemberName(_) => false,

                JsArrowFunctionExpression(_) => {
                    is_arrow_function_body(self.syntax(), parent)
                },

                JsForStatement(for_statement) => {
                     let is_initializer = match for_statement.initializer() {
                        Some(AnyJsForInitializer::AnyJsExpression(expression)) => {
                            expression.syntax() == self.syntax()
                        }
                        None | Some(_) => false,
                    };

                    let is_update = for_statement
                        .update()
                        .map(AstNode::into_syntax)
                        .as_ref()
                        == Some(self.syntax());

                    !(is_initializer || is_update)
                },
                JsExpressionStatement(_) => {
                    // Parenthesize `{ a } = { a: 5 }`
                    is_first_in_statement(
                        self.clone().into(),
                        FirstInStatementMode::ExpressionStatementOrArrow,
                    ) && matches!(
                        self.left(),
                        Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_))
                    )
                },
                JsSequenceExpression(_) => {
                     let mut child = parent.clone();

                    for ancestor in parent.ancestors().skip(1) {
                        match ancestor.kind() {
                            JsSyntaxKind::JS_SEQUENCE_EXPRESSION
                            | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => child = ancestor,
                            JsSyntaxKind::JS_FOR_STATEMENT => {
                                let for_statement = JsForStatement::unwrap_cast(ancestor);

                                let is_initializer = match for_statement.initializer() {
                                    Some(AnyJsForInitializer::AnyJsExpression(expression)) => {
                                        expression.syntax() == &child
                                    }
                                    None | Some(_) => false,
                                };

                                let is_update =
                                    for_statement.update().map(AstNode::into_syntax).as_ref()
                                        == Some(&child);

                                return !(is_initializer || is_update);
                            }
                            _ => break,
                        }
                    }

                    true
                },
                _ => {
                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsAssignmentExpression;

    #[test]
    fn needs_parentheses() {
        assert_not_needs_parentheses!("({ [a = 3]: value })", JsAssignmentExpression);
        assert_not_needs_parentheses!("class Test { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("type Test  = { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("interface Test { [a = 3]: value }", JsAssignmentExpression);

        assert_needs_parentheses!("a => (a = 3)", JsAssignmentExpression);
        assert_not_needs_parentheses!("a => { a = 3 }", JsAssignmentExpression);

        assert_not_needs_parentheses!("for(a = 3;;) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(a = 3, b = 2;;) {}", JsAssignmentExpression[1]);
        assert_not_needs_parentheses!("for(a = 3, b = 2, c= 3;;) {}", JsAssignmentExpression[2]);
        assert_needs_parentheses!("for(; a = 3; ) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(;;a = 3) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("for ((a, a = 3);;) {}", JsAssignmentExpression);
        assert_needs_parentheses!("for (; (a, a = 3);) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for (;;(a, a = 3)) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("a = 3", JsAssignmentExpression);
        assert_needs_parentheses!("({ a } = { a: 3 })", JsAssignmentExpression);
    }
}
