use biome_js_semantic::{BindingExtensions, SemanticModel};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsTemplateElement, JsArrowFunctionExpression, JsAssignmentExpression, JsAssignmentOperator,
    JsAwaitExpression, JsCallExpression, JsCaseClause, JsComputedMemberExpression,
    JsConstructorParameters, JsDoWhileStatement, JsElseClause, JsExportDefaultExpressionClause,
    JsExpressionStatement, JsForInStatement, JsForOfStatement, JsForStatement, JsFormalParameter,
    JsFunctionBody, JsFunctionExpression, JsIfStatement, JsImportCallExpression, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalOperator, JsModule, JsNewExpression, JsParameters,
    JsPostUpdateExpression, JsPreUpdateExpression, JsReturnStatement, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsTemplateExpression, JsThrowStatement,
    JsUnaryExpression, JsUnaryOperator, JsWhileStatement, JsWithStatement, JsYieldArgument,
    JsYieldExpression, T, TsNonNullAssertionExpression,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TriviaPiece, WalkEvent};

pub(crate) mod dom;

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

/// Returns whether an identifier-like replacement needs leading whitespace at this expression.
///
/// JavaScript permits punctuation-prefixed expressions to touch a preceding keyword, as in
/// `return!value`. If a replacement starts with an identifier-like token, omitting the whitespace
/// would instead produce a different token such as `returnvalue`. This also applies to keyword
/// operators and to expression positions following `yield`, `else`, `do`, or `case`.
pub(crate) fn needs_space_before_identifier_expression_replacement(
    expression: &AnyJsExpression,
) -> bool {
    let Some(parent) = expression.syntax().parent() else {
        return false;
    };

    if EXPRESSION_REPLACEMENT_SEPARATOR_PARENT_KINDS.matches(parent.kind())
        || JsUnaryExpression::cast_ref(&parent).is_some_and(|expression| {
            matches!(
                expression.operator(),
                Ok(JsUnaryOperator::Typeof | JsUnaryOperator::Void | JsUnaryOperator::Delete)
            )
        })
    {
        return true;
    }

    if let Some(argument) = JsYieldArgument::cast_ref(&parent) {
        return argument.star_token().is_none()
            && parent.parent().is_some_and(|parent| {
                EXPRESSION_REPLACEMENT_SEPARATOR_PARENT_KINDS.matches(parent.kind())
            });
    }

    JsExpressionStatement::can_cast(parent.kind())
        && parent.parent().is_some_and(|parent| {
            EXPRESSION_REPLACEMENT_SEPARATOR_PARENT_KINDS.matches(parent.kind())
        })
}

const EXPRESSION_REPLACEMENT_SEPARATOR_PARENT_KINDS: SyntaxKindSet<JsLanguage> =
    JsExportDefaultExpressionClause::KIND_SET
        .union(JsInstanceofExpression::KIND_SET)
        .union(JsYieldExpression::KIND_SET)
        .union(JsReturnStatement::KIND_SET)
        .union(JsThrowStatement::KIND_SET)
        .union(JsNewExpression::KIND_SET)
        .union(JsAwaitExpression::KIND_SET)
        .union(JsInExpression::KIND_SET)
        .union(JsForOfStatement::KIND_SET)
        .union(JsForInStatement::KIND_SET)
        .union(JsDoWhileStatement::KIND_SET)
        .union(JsElseClause::KIND_SET)
        .union(JsCaseClause::KIND_SET);

/// Returns `true` if `expression` begins an expression statement, and the token
/// before it doesn't end the previous statement.
///
/// A replacement for such an expression can't start with `(` or `-`, because
/// automatic semicolon insertion doesn't separate the two statements. For example,
/// replacing `x * -1` below with `-x` produces `foo - x`:
///
/// ```js
/// foo
/// x * -1
/// ```
///
/// Tokens that end a statement include `;`, `{`, the `}` of a block or function
/// body, and the `)` that closes the condition of an `if` statement or a loop.
pub(crate) fn starts_unterminated_statement(expression: &AnyJsExpression) -> bool {
    let Some(first_token) = expression.syntax().first_token() else {
        return false;
    };
    let is_statement_start = expression
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().first_token())
        .is_some_and(|token| token == first_token);
    if !is_statement_start {
        return false;
    }
    let Some(previous) = first_token.prev_token() else {
        return false;
    };
    !terminates_statement(&previous)
}

const STATEMENT_TERMINATORS: TokenSet<JsSyntaxKind> =
    token_set![T![;], T!['{'], T![:], T![else], T![do]];

const CONTROL_FLOW_STATEMENT_KINDS: SyntaxKindSet<JsLanguage> = JsIfStatement::KIND_SET
    .union(JsWhileStatement::KIND_SET)
    .union(JsDoWhileStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsWithStatement::KIND_SET);

/// Returns `true` if a `(` or `-` after `token` starts a new statement instead of
/// continuing an expression.
fn terminates_statement(token: &JsSyntaxToken) -> bool {
    if STATEMENT_TERMINATORS.contains(token.kind()) {
        return true;
    }
    match token.kind() {
        T![')'] => token
            .parent()
            .is_some_and(|parent| CONTROL_FLOW_STATEMENT_KINDS.matches(parent.kind())),
        T!['}'] => {
            let Some(mut parent) = token.parent() else {
                return false;
            };
            if parent.kind() == JsSyntaxKind::JS_FUNCTION_BODY
                && let Some(grand_parent) = parent.parent()
            {
                parent = grand_parent;
            }
            !AnyJsExpression::can_cast(parent.kind())
        }
        _ => false,
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
                    .is_ok_and(|left| is_logical_identity(left, operator));
                if is_left_logical_identify {
                    return true;
                }

                node.right()
                    .is_ok_and(|right| is_logical_identity(right, operator))
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

/// Checks if the given node is within an async function.
///
/// Traverses up the syntax tree to find the nearest function-like boundary
/// (including arrow functions, function expressions, function declarations,
/// class methods, object methods, and constructors) and checks if it has the
/// `async` keyword. Also stops at sync-only boundaries like getters, setters,
/// and static initialization blocks.
pub fn is_in_async_function(node: &JsSyntaxNode) -> bool {
    for ancestor in node.ancestors() {
        if let Some(func) = AnyFunctionLike::cast_ref(&ancestor) {
            return func.is_async();
        }
        if is_sync_only_function_boundary(ancestor.kind()) {
            return false;
        }
    }
    false
}

/// Returns `true` if `await` is syntactically valid at the given node position.
///
/// True when inside an `async` function, or at module top-level where
/// top-level await (ES2022+) is supported. Biome unconditionally assumes
/// modules support top-level await.
pub fn is_await_allowed(node: &JsSyntaxNode) -> bool {
    for ancestor in node.ancestors() {
        if let Some(func) = AnyFunctionLike::cast_ref(&ancestor) {
            return func.is_async();
        }
        if is_sync_only_function_boundary(ancestor.kind()) {
            return false;
        }
        if JsModule::can_cast(ancestor.kind()) {
            return true;
        }
    }
    false
}

/// Returns `true` if the node kind is a sync-only function boundary where
/// `async` is never valid (getters, setters, static initialization blocks).
fn is_sync_only_function_boundary(kind: JsSyntaxKind) -> bool {
    matches!(
        kind,
        JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
    )
}

/// Returns `true` if the node kind is any function-like scope boundary.
pub fn is_function_boundary(kind: JsSyntaxKind) -> bool {
    AnyFunctionLike::can_cast(kind) || is_sync_only_function_boundary(kind)
}

/// The nodes between a `new` keyword and an expression that the callee of `new` reads a
/// property from, when the expression is their first child.
const NEW_CALLEE_OBJECT_PARENT_KINDS: SyntaxKindSet<JsLanguage> =
    JsStaticMemberExpression::KIND_SET
        .union(JsTemplateExpression::KIND_SET)
        .union(TsNonNullAssertionExpression::KIND_SET);

/// The nodes that hold the parameters and the body of a function. This code runs when
/// the function is called, not when the expression that creates the function is evaluated.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = JsArrowFunctionExpression::KIND_SET
    .union(JsFunctionExpression::KIND_SET)
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsFunctionBody::KIND_SET);

/// The nodes that [`may_have_side_effects`] looks for, and the [`FUNCTION_KINDS`] that it
/// skips. A unary expression only has a side effect if its operator is `delete`.
const SIDE_EFFECT_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(FUNCTION_KINDS);

/// Returns `true` if evaluating `expression` may do more than produce a value, such as
/// calling a function or assigning to a variable.
///
/// Reading a property and evaluating a template literal are assumed to have no side
/// effects. The parameters and the body of a function that `expression` creates aren't
/// searched, because creating a function doesn't run it.
pub(crate) fn may_have_side_effects(expression: &AnyJsExpression) -> bool {
    let mut preorder = expression
        .syntax()
        .preorder_of_kinds(SIDE_EFFECT_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if FUNCTION_KINDS.matches(node.kind()) {
            preorder.skip_subtree();
        } else if JsUnaryExpression::cast(node)
            .is_none_or(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
        {
            return true;
        }
    }
    false
}

/// Returns `true` if `object` must be wrapped in parentheses before a property can be
/// read from it.
///
/// `new Set` without arguments would read the property from `Set` itself, and
/// `a ? b : c` would read the property from `c` only.
pub(crate) fn needs_parentheses_as_object(object: &AnyJsExpression) -> bool {
    match object {
        AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsParenthesizedExpression(_)
        | AnyJsExpression::TsNonNullAssertionExpression(_) => false,
        AnyJsExpression::JsNewExpression(new) => new.arguments().is_none(),
        _ => true,
    }
}

/// Returns `true` if `node` is the object that a `new` callee reads a property from,
/// such as in `new new Set(a.union(b)).constructor()`, or if it is the callee itself.
///
/// A call in that place needs parentheses, because `new a.union(b).constructor()` calls
/// `a.union` with `new`.
pub(crate) fn is_in_new_callee(node: &JsNewExpression) -> bool {
    let mut child = node.syntax().clone();
    for ancestor in node.syntax().ancestors().skip(1) {
        // The arguments of `new` are wrapped in another node, so `child` is the callee.
        if JsNewExpression::can_cast(ancestor.kind()) {
            return true;
        }
        let is_object = match JsComputedMemberExpression::cast_ref(&ancestor) {
            Some(member) => member
                .object()
                .is_ok_and(|object| object.syntax() == &child),
            None => NEW_CALLEE_OBJECT_PARENT_KINDS.matches(ancestor.kind()),
        };
        if !is_object {
            return false;
        }
        child = ancestor;
    }
    false
}

/// Returns the keyword right before `expression` when no whitespace separates them, as in
/// `return[...set].length`.
///
/// A replacement for `expression` that starts with a name, such as `set`, must not be
/// joined to the keyword.
pub(crate) fn keyword_directly_before(expression: &AnyJsExpression) -> Option<JsSyntaxToken> {
    let first_token = expression.syntax().first_token()?;
    let previous = first_token.prev_token()?;
    (previous.kind().is_keyword()
        && previous.text_trimmed_range().end() == first_token.text_trimmed_range().start())
    .then_some(previous)
}

#[cfg(test)]
mod tests {
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::AnyJsLiteralExpression;
    use biome_languages::JsFileSource;
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
