use biome_js_semantic::{BindingExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement,
    JsAssignmentOperator, JsLanguage, JsLogicalOperator, JsSyntaxNode, JsSyntaxToken,
    JsUnaryOperator,
};
use biome_rowan::{AstNode, AstSeparatedList, TriviaPiece};

/// Add any leading and trailing trivia from given source node to the token.
///
/// Adds whitespace trivia if needed for safe replacement of source node.
pub fn token_with_source_trivia<T>(token: &JsSyntaxToken, source: &T) -> JsSyntaxToken
where
    T: AstNode<Language = JsLanguage>,
{
    let mut text = String::new();
    let node = source.syntax();
    let mut leading = vec![];
    let mut trailing = vec![];

    add_leading_trivia(&mut leading, &mut text, node);
    text.push_str(token.text());
    add_trailing_trivia(&mut trailing, &mut text, node);

    JsSyntaxToken::new_detached(token.kind(), &text, leading, trailing)
}

fn add_leading_trivia(trivia: &mut Vec<TriviaPiece>, text: &mut String, node: &JsSyntaxNode) {
    let Some(token) = node.first_token() else {
        return;
    };
    for t in token.leading_trivia().pieces() {
        text.push_str(t.text());
        trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
    }
    if !trivia.is_empty() {
        return;
    }
    let Some(token) = token.prev_token() else {
        return;
    };
    if !token.kind().is_punct() && token.trailing_trivia().pieces().next().is_none() {
        text.push(' ');
        trivia.push(TriviaPiece::whitespace(1));
    }
}

fn add_trailing_trivia(trivia: &mut Vec<TriviaPiece>, text: &mut String, node: &JsSyntaxNode) {
    let Some(token) = node.last_token() else {
        return;
    };
    for t in token.trailing_trivia().pieces() {
        text.push_str(t.text());
        trivia.push(TriviaPiece::new(t.kind(), t.text_len()));
    }
    if !trivia.is_empty() {
        return;
    }
    let Some(token) = token.next_token() else {
        return;
    };
    if !token.kind().is_punct() && token.leading_trivia().pieces().next().is_none() {
        text.push(' ');
        trivia.push(TriviaPiece::whitespace(1));
    }
}

pub fn is_constant_condition(
    test: AnyJsExpression,
    in_boolean_position: bool,
    model: &SemanticModel,
) -> Option<()> {
    use AnyJsExpression::*;

    match test.omit_parentheses() {
        AnyJsLiteralExpression(_)
        | JsObjectExpression(_)
        | JsFunctionExpression(_)
        | JsArrowFunctionExpression(_)
        | JsClassExpression(_) => Some(()),
        JsUnaryExpression(node) => {
            use JsUnaryOperator::*;

            let op = node.operator().ok()?;
            if op == Void || op == Typeof && in_boolean_position {
                return Some(());
            }
            if op == LogicalNot {
                return is_constant_condition(node.argument().ok()?, true, model);
            }
            is_constant_condition(node.argument().ok()?, false, model)
        }
        JsBinaryExpression(node) => is_constant_condition(node.left().ok()?, false, model)
            .and_then(|_| is_constant_condition(node.right().ok()?, false, model)),
        JsLogicalExpression(node) => {
            let left = node.left().ok()?;
            let right = node.right().ok()?;
            let op = node.operator().ok()?;
            let is_left_constant =
                is_constant_condition(left.clone(), in_boolean_position, model).is_some();
            let is_right_constant =
                is_constant_condition(right.clone(), in_boolean_position, model).is_some();

            let is_left_short_circuit = is_left_constant && is_logical_identity(left, op);
            let is_right_short_circuit =
                in_boolean_position && is_right_constant && is_logical_identity(right, op);

            if (is_left_constant && is_right_constant)
                || is_left_short_circuit
                || is_right_short_circuit
            {
                Some(())
            } else {
                None
            }
        }
        JsSequenceExpression(node) => {
            is_constant_condition(node.right().ok()?, in_boolean_position, model)
        }
        JsIdentifierExpression(node) => {
            if node.name().ok()?.binding(model).is_some() {
                // This is any_js_stmt edge case. Modern browsers don't allow to redeclare `undefined` but ESLint handle this so we do
                return None;
            }
            let is_named_undefined = node.name().ok()?.is_undefined();
            is_named_undefined.then_some(())
        }
        JsArrayExpression(node) => {
            if !in_boolean_position {
                node.elements()
                    .into_iter()
                    .all(|js_statement| {
                        if let Ok(element) = js_statement {
                            match element {
                                AnyJsArrayElement::JsArrayHole(_) => true,
                                AnyJsArrayElement::JsSpread(node) => {
                                    if let Ok(argument) = node.argument() {
                                        is_constant_condition(argument, in_boolean_position, model)
                                            .is_some()
                                    } else {
                                        false
                                    }
                                }
                                _ => element
                                    .as_any_js_expression()
                                    .and_then(|node| {
                                        is_constant_condition(node.clone(), false, model)
                                    })
                                    .is_some(),
                            }
                        } else {
                            false
                        }
                    })
                    .then_some(())
            } else {
                Some(())
            }
        }
        JsNewExpression(_) => in_boolean_position.then_some(()),
        JsCallExpression(node) => {
            if node.has_callee("Boolean") {
                let callee = node.callee().ok()?;
                let ident = callee.as_js_identifier_expression()?.name().ok()?;
                let binding = ident.binding(model);
                if binding.is_some() {
                    return None;
                }

                let args = node.arguments().ok()?.args();
                if args.is_empty() {
                    return Some(());
                }
                return is_constant_condition(
                    args.first()?.ok()?.as_any_js_expression()?.clone(),
                    true,
                    model,
                );
            }

            None
        }
        JsAssignmentExpression(node) => {
            use JsAssignmentOperator::*;

            let operator = node.operator().ok()?;
            if operator == Assign {
                return is_constant_condition(node.right().ok()?, in_boolean_position, model);
            }

            if matches!(operator, LogicalOrAssign | LogicalAndAssign) && in_boolean_position {
                let new_op = match operator {
                    LogicalAndAssign => JsLogicalOperator::LogicalAnd,
                    LogicalOrAssign => JsLogicalOperator::LogicalOr,
                    _ => unreachable!(),
                };

                return is_logical_identity(node.right().ok()?, new_op).then_some(());
            }
            None
        }
        JsTemplateExpression(node) => {
            let is_tag = node.tag().is_some();
            let elements = node.elements();
            let has_truthy_quasi = !is_tag
                && elements.clone().into_iter().any(|element| match element {
                    AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                        if let Ok(quasi) = element.template_chunk_token() {
                            !quasi.text_trimmed().is_empty()
                        } else {
                            false
                        }
                    }
                    AnyJsTemplateElement::JsTemplateElement(_) => false,
                });
            if has_truthy_quasi && in_boolean_position {
                return Some(());
            }

            elements
                .into_iter()
                .all(|element| match element {
                    AnyJsTemplateElement::JsTemplateChunkElement(_) => !is_tag,
                    AnyJsTemplateElement::JsTemplateElement(element) => {
                        if let Ok(expr) = element.expression() {
                            is_constant_condition(expr, false, model).is_some()
                        } else {
                            false
                        }
                    }
                })
                .then_some(())
        }
        _ => None,
    }
}

fn is_logical_identity(node: AnyJsExpression, operator: JsLogicalOperator) -> bool {
    use AnyJsExpression::*;
    use JsLogicalOperator::*;
    match node.omit_parentheses() {
        AnyJsLiteralExpression(node) => {
            let boolean_value = get_boolean_value(&node);
            operator == LogicalOr && boolean_value || (operator == LogicalAnd && !boolean_value)
        }
        JsUnaryExpression(node) => {
            if operator != LogicalAnd {
                return false;
            }

            if let Ok(node_operator) = node.operator() {
                node_operator == JsUnaryOperator::Void
            } else {
                false
            }
        }
        JsLogicalExpression(node) => {
            if let Ok(node_operator) = node.operator() {
                // handles `any_js_stmt && false || b`
                // `false` is an identity element of `&&` but not `||`
                // so the logical identity of the whole expression can not be defined.
                if operator != node_operator {
                    return false;
                }

                let is_left_logical_identify = node
                    .left()
                    .ok()
                    .is_some_and(|left| is_logical_identity(left, operator));
                if is_left_logical_identify {
                    return true;
                }

                node.right()
                    .ok()
                    .is_some_and(|right| is_logical_identity(right, operator))
            } else {
                false
            }
        }
        JsAssignmentExpression(node) => {
            if let Ok(node_operator) = node.operator() {
                if let Ok(right) = node.right() {
                    let is_valid_logical_assignment = match node_operator {
                        JsAssignmentOperator::LogicalAndAssign
                            if operator == JsLogicalOperator::LogicalAnd =>
                        {
                            true
                        }
                        JsAssignmentOperator::LogicalOrAssign
                            if operator == JsLogicalOperator::LogicalOr =>
                        {
                            true
                        }
                        _ => false,
                    };

                    is_valid_logical_assignment && is_logical_identity(right, operator)
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

fn get_boolean_value(node: &AnyJsLiteralExpression) -> bool {
    use AnyJsLiteralExpression::*;
    match node {
        JsRegexLiteralExpression(_) => true,
        _ => node
            .as_static_value()
            .is_some_and(|value| !value.is_falsy()),
    }
}

#[cfg(test)]
mod tests {
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{AnyJsLiteralExpression, JsFileSource};
    use biome_rowan::SyntaxNodeCast;

    use super::get_boolean_value;

    fn assert_boolean_value(code: &str, value: bool) {
        let source = biome_js_parser::parse(code, JsFileSource::tsx(), JsParserOptions::default());

        if source.has_errors() {
            panic!("syntax error")
        }

        let literal_expression = source
            .syntax()
            .descendants()
            .find_map(|js_statement| js_statement.cast::<AnyJsLiteralExpression>());

        assert_eq!(
            get_boolean_value(&literal_expression.expect("Not found AnyLiteralExpression.")),
            value
        );
    }
    #[test]
    fn test_get_boolean_value() {
        assert_boolean_value("false", false);
        assert_boolean_value("0", false);
        assert_boolean_value("-0", false);
        assert_boolean_value("0n", false);
        assert_boolean_value("let any_js_stmt =\"\"", false);
        assert_boolean_value("let any_js_stmt = ''", false);
        assert_boolean_value("null", false);

        assert_boolean_value("true", true);
        assert_boolean_value("let any_js_stmt = \"0\"", true);
        assert_boolean_value("let any_js_stmt = \"false\"", true);
        assert_boolean_value("-42", true);
        assert_boolean_value("12n", true);
        assert_boolean_value("3.14", true);
        assert_boolean_value("-3.14", true);
    }
}
