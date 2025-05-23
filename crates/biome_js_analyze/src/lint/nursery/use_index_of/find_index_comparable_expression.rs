use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, JsAssignmentExpression, JsBinaryExpression,
    JsLogicalExpression, JsReturnStatement, JsSyntaxNode, JsVariableDeclaration,
};
use biome_rowan::AstNode;

fn node_is_undefined(node: &AnyJsExpression) -> bool {
    node.syntax().text_trimmed() == "undefined"
}

fn extract_simple_compare_match(
    expression: &JsBinaryExpression,
    parameter_name: &String,
) -> Option<JsSyntaxNode> {
    if expression.operator_token().ok()?.text_trimmed() != "===" {
        return None;
    }

    let (left, right) = (expression.left().unwrap(), expression.right().unwrap());

    let matching_side = if left.to_trimmed_string() == *parameter_name {
        right
    } else if right.to_trimmed_string() == *parameter_name {
        left
    } else {
        return None;
    };

    if node_is_undefined(&matching_side) {
        return None;
    }

    Some(matching_side.syntax().clone())
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

    let binary_expressions: Vec<_> = body
        .syntax()
        .descendants()
        .filter_map(JsBinaryExpression::cast)
        .collect();

    if binary_expressions.len() != 1 || !invalid_expressions.is_empty() {
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
