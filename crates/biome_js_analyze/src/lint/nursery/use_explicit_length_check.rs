use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{self, token};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsConditionalExpression,
    JsDoWhileStatement, JsForStatement, JsIfStatement, JsNewExpression, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, JsWhileStatement, T,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind};

use crate::JsRuleAction;

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub UseExplicitLengthCheck {
        version: "next",
        name: "useExplicitLengthCheck",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("explicit-length-check")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExplicitLengthCheck {
    type Query = Ast<JsStaticMemberExpression>;
    type State = UseExplicitLengthCheckState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let member_name = node.member().ok()?.text();

        if !LENGTH_MEMBER_NAMES
            .binary_search(&member_name.as_str())
            .is_ok()
        {
            return None;
        }

        let syntax = node.syntax();
        let parent_syntax = syntax.parent()?;

        if is_in_boolean_context(syntax).is_some()
            || is_boolean_call(&parent_syntax).unwrap_or(false)
            || is_boolean_constructor_call(&parent_syntax).unwrap_or(false)
        {
            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::NonZero,
                suggested_fix: LengthFixKind::ReplaceWhole(AnyJsExpression::cast(syntax.clone())?),
                member_name,
            });
        }

                    // || is_negation(&parent_syntax).is_some()

        let parent_syntax = syntax.parent()?;
        let binary_expr = match parent_syntax.kind() {
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                Some(JsBinaryExpression::cast_ref(&parent_syntax).unwrap())
            }
            _ => None,
        }?;

        let (member_position, literal) = extract_binary_position_and_literal(&binary_expr)?;

        let condition = &(
            member_position,
            binary_expr.operator().ok()?,
            literal.as_js_number_literal_expression()?.as_number()?,
        );

        let invalid_zero_len_checks = [
            // `foo.length == 0`
            (MemberPosition::Right, JsBinaryOperator::Equality, 0.0),
            // `0 == foo.length`
            (MemberPosition::Left, JsBinaryOperator::Equality, 0.0),
            // `foo.length < 1`
            (MemberPosition::Right, JsBinaryOperator::LessThan, 1.0),
            // `1 > foo.length`
            (MemberPosition::Left, JsBinaryOperator::GreaterThan, 1.0),
        ];

        if matches_numeric_binary_condition(condition, &invalid_zero_len_checks) {
            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::Zero,
                suggested_fix: LengthFixKind::ModifyBinary(binary_expr, member_position),
                member_name,
            });
        }

        let invalid_non_zero_len_checks = [
            // `foo.length !== 0`
            (
                MemberPosition::Right,
                JsBinaryOperator::StrictInequality,
                0.0,
            ),
            // `0 !== foo.length`
            (
                MemberPosition::Left,
                JsBinaryOperator::StrictInequality,
                0.0,
            ),
            // `foo.length != 0`
            (MemberPosition::Right, JsBinaryOperator::Inequality, 0.0),
            // `0 != foo.length`
            (MemberPosition::Left, JsBinaryOperator::Inequality, 0.0),
            // `foo.length >= 1`
            (
                MemberPosition::Right,
                JsBinaryOperator::GreaterThanOrEqual,
                1.0,
            ),
            // `1 <= foo.length`
            (MemberPosition::Left, JsBinaryOperator::LessThanOrEqual, 1.0),
        ];

        if matches_numeric_binary_condition(condition, &invalid_non_zero_len_checks) {
            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::NonZero,
                suggested_fix: LengthFixKind::ModifyBinary(binary_expr, member_position),
                member_name,
            });
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        // let node = ctx.query();

        let (code, type_text) = match state.check {
            LengthCheck::Zero => ("=== 0", "zero"),
            LengthCheck::NonZero => ("> 0", "not zero"),
        };

        let span = match &state.suggested_fix {
            LengthFixKind::ReplaceWhole(node) => node.range(),
            LengthFixKind::ModifyBinary(node, _) => node.range(),
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Use "<Emphasis>"."{state.member_name}" "{code}</Emphasis>" with "<Emphasis>"."{state.member_name}</Emphasis>" is "{type_text}"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match &state.suggested_fix {
            LengthFixKind::ReplaceWhole(node) => {
                let operator_syntax = match state.check {
                    LengthCheck::Zero => T![===],
                    LengthCheck::NonZero => T![>],
                };

                let new_binary_expr = make::js_binary_expression(
                    node.clone().into(),
                    make::token_decorated_with_space(operator_syntax),
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsNumberLiteralExpression(
                            make::js_number_literal_expression(make::js_number_literal("0")),
                        ),
                    ),
                );

                mutation.replace_node_discard_trivia::<AnyJsExpression>(node.clone().into(), new_binary_expr.into());

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Replace with" }.to_owned(),
                    mutation,
                })
            }
            LengthFixKind::ModifyBinary(binary_expr, member_position) => {
                let operator_syntax = match state.check {
                    LengthCheck::Zero => match member_position {
                        MemberPosition::Left => T![===],
                        MemberPosition::Right => T![===],
                    },
                    LengthCheck::NonZero => match member_position {
                        MemberPosition::Left => T![>],
                        MemberPosition::Right => T![<],
                    },
                };

                let new_binary_expr = make::js_binary_expression(
                    binary_expr.left().ok()?.clone().trim_trailing_trivia()?,
                    make::token_decorated_with_space(operator_syntax),
                    binary_expr.right().ok()?.clone().trim_leading_trivia()?,
                );

                mutation.replace_node::<AnyJsExpression>(
                    binary_expr.clone().into(),
                    new_binary_expr.into(),
                );

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Add "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
                    mutation,
                })
            }
        }
    }
}

/// Sorted list of length properties.
const LENGTH_MEMBER_NAMES: &[&str] = &["byteLength", "byteOffset", "length", "size"];

pub struct UseExplicitLengthCheckState {
    check: LengthCheck,
    suggested_fix: LengthFixKind,
    member_name: String,
}

#[derive(PartialEq, Clone, Copy)]
enum MemberPosition {
    Left,
    Right,
}

enum LengthCheck {
    Zero,
    NonZero,
}

enum LengthFixKind {
    ReplaceWhole(AnyJsExpression),
    ModifyBinary(JsBinaryExpression, MemberPosition),
}

fn extract_binary_position_and_literal(
    node: &JsBinaryExpression,
) -> Option<(MemberPosition, AnyJsLiteralExpression)> {
    match (node.left().ok()?, node.right().ok()?) {
        (
            AnyJsExpression::JsStaticMemberExpression(_),
            AnyJsExpression::AnyJsLiteralExpression(literal),
        ) => Some((MemberPosition::Right, literal)),
        (
            AnyJsExpression::AnyJsLiteralExpression(literal),
            AnyJsExpression::JsStaticMemberExpression(_),
        ) => Some((MemberPosition::Left, literal)),
        _ => None,
    }
}

type NumericBinaryCondition = (MemberPosition, JsBinaryOperator, f64);

fn matches_numeric_binary_condition(
    target_condition: &NumericBinaryCondition,
    conditions: &[NumericBinaryCondition],
) -> bool {
    conditions
        .iter()
        .any(|condition| condition == target_condition)
}

/// Check if this node is in the position of `test` slot of parent syntax node.
/// ## Example
/// ```js
/// if (!!x) {
///     ^^^ this is a boolean context
/// }
/// ```
fn is_in_boolean_context(node: &JsSyntaxNode) -> Option<bool> {
    let parent = node.parent()?;
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => {
            Some(parent.cast::<JsIfStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
            Some(parent.cast::<JsDoWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_WHILE_STATEMENT => {
            Some(parent.cast::<JsWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_FOR_STATEMENT => {
            Some(parent.cast::<JsForStatement>()?.test()?.syntax() == node)
        }
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => Some(
            parent
                .cast::<JsConditionalExpression>()?
                .test()
                .ok()?
                .syntax()
                == node,
        ),
        _ => None,
    }
}

/// Checks if the node is a `Boolean` Constructor Call
/// # Example
/// ```js
/// new Boolean(x);
/// ```
/// The checking algorithm of [JsNewExpression] is a little different from [JsCallExpression] due to
/// two nodes have different shapes
fn is_boolean_constructor_call(node: &JsSyntaxNode) -> Option<bool> {
    let expr = JsCallArgumentList::cast_ref(node)?
        .parent::<JsCallArguments>()?
        .parent::<JsNewExpression>()?;
    Some(expr.has_callee("Boolean"))
}

/// Check if the SyntaxNode is a `Boolean` Call Expression
/// ## Example
/// ```js
/// Boolean(x)
/// ```
fn is_boolean_call(node: &JsSyntaxNode) -> Option<bool> {
    let expr = JsCallArgumentList::cast_ref(node)?
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;
    Some(expr.has_callee("Boolean"))
}

/// Check if the SyntaxNode is a Negate Unary Expression
/// ## Example
/// ```js
/// !!x
/// ```
fn is_negation(node: &JsSyntaxNode) -> Option<JsUnaryExpression> {
    let unary_expr = JsUnaryExpression::cast_ref(node)?;
    if unary_expr.operator().ok()? == JsUnaryOperator::LogicalNot {
        Some(unary_expr)
    } else {
        None
    }
}


/// Check if the SyntaxNode is a Double Negation. Including the edge case
/// ```js
/// !(!x)
/// ```
/// Return [Rule::State] `(JsAnyExpression, ExtraBooleanCastType)` if it is a DoubleNegation Expression
///
fn is_double_negation_ignore_parenthesis(
    syntax: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> Option<(AnyJsExpression, ExtraBooleanCastType)> {
    if let Some(negation_expr) = is_negation(syntax) {
        let argument = negation_expr.argument().ok()?;
        match argument {
            AnyJsExpression::JsUnaryExpression(expr)
                if expr.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                expr.argument()
                    .ok()
                    .map(|argument| (argument, ExtraBooleanCastType::DoubleNegation))
            }
            // Check edge case `!(!xxx)`
            AnyJsExpression::JsParenthesizedExpression(expr) => {
                expr.expression().ok().and_then(|expr| {
                    is_negation(expr.syntax()).and_then(|negation| {
                        Some((
                            negation.argument().ok()?,
                            ExtraBooleanCastType::DoubleNegation,
                        ))
                    })
                })
            }
            _ => None,
        }
    } else {
        None
    }
}

#[test]
fn test_order() {
    for items in LENGTH_MEMBER_NAMES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
