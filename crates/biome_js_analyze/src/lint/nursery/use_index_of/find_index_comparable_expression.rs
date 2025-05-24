use biome_js_syntax::{
    AnyJsFunctionBody, JsAssignmentExpression, JsBinaryExpression, JsLogicalExpression,
    JsReturnStatement, JsSyntaxNode, JsVariableDeclaration, T,
};
use biome_rowan::AstNode;

fn extract_simple_compare_match(
    expression: &JsBinaryExpression,
    parameter_name: &String,
) -> Option<JsSyntaxNode> {
    if expression.operator_token().ok()?.kind() != T![===] {
        return None;
    }

    let (left, right) = (expression.left().ok()?, expression.right().ok()?);

    let matching_side = if left.syntax().to_string().trim() == *parameter_name {
        right
    } else if right.syntax().to_string().trim() == *parameter_name {
        left
    } else {
        return None;
    };

    Some(matching_side.into_syntax())
}

pub fn find_index_comparable_expression(
    body: &AnyJsFunctionBody,
    parameter_name: &String,
    return_statement_required: bool,
) -> Option<JsSyntaxNode> {
    let invalid_expressions: Vec<_> = body
        .syntax()
        .descendants()
        .filter(|node| {
            JsAssignmentExpression::can_cast(node.kind())
                || JsVariableDeclaration::can_cast(node.kind())
                || JsLogicalExpression::can_cast(node.kind())
        })
        .collect();

    if !invalid_expressions.is_empty() {
        return None;
    }

    let binary_expressions: Vec<_> = body
        .syntax()
        .descendants()
        .filter_map(JsBinaryExpression::cast)
        .collect();

    if binary_expressions.len() != 1 {
        return None;
    }

    let return_statements: Vec<_> = body
        .syntax()
        .descendants()
        .filter_map(JsReturnStatement::cast)
        .collect();

    if return_statements.len() > 1 {
        return None;
    }

    if return_statement_required && return_statements.len() != 1 {
        return None;
    }

    binary_expressions
        .into_iter()
        .find_map(|expression| extract_simple_compare_match(&expression, parameter_name))
}
