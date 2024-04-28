use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    is_in_boolean_context, is_negation, AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsCallArgumentList, JsCallArguments, JsCallExpression, JsLogicalExpression, JsLogicalOperator, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, T
};
use biome_rowan::{
    chain_trivia_pieces, AstNode, AstSeparatedList, BatchMutationExt, SyntaxTriviaPiece,
    TriviaPiece, TriviaPieceKind,
};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce explicitly comparing the `length`, `size`, `byteLength` or `byteOffset` property of a value.
    ///
    /// This rule enforces a specific style length comparisons to make them more clear.
    ///
    /// ## Zero comparison examples
    /// Enforce comparison with === 0 when checking for zero length.
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const isEmpty = !foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmpty = foo.length == 0;
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmpty = foo.length < 1;
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmpty = 0 === foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmpty = 0 == foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmpty = 1 > foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// // Negative style is disallowed too
    /// const isEmpty = !(foo.length > 0);
    /// ```
    /// ```js,expect_diagnostic
    /// const isEmptySet = !foo.size;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const isEmpty = foo.length === 0;
    /// ```
    ///
    /// ## Non-zero comparison examples
    /// Enforce comparison with > 0 when checking for non-zero length.
    ///
    /// ### Invalid
    /// ```js,expect_diagnostic
    /// const isNotEmpty = foo.length !== 0;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = foo.length != 0;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = foo.length >= 1;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = 0 !== foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = 0 != foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = 1 <= foo.length;
    /// ```
    /// ```js,expect_diagnostic
    /// const isNotEmpty = Boolean(foo.length);
    /// ```
    /// ```js,expect_diagnostic
    /// // Negative style is disallowed too
    /// const isNotEmpty = !(foo.length === 0);
    /// ```
    /// ```js,expect_diagnostic
    /// if (foo.length || bar.length) {}
    /// ```
    /// ```js,expect_diagnostic
    /// const biome = foo.length ? 1 : 2
    /// ```
    /// ```js,expect_diagnostic
    /// while (foo.length) {}
    /// ```
    /// ```js,expect_diagnostic
    /// do {} while (foo.length);
    /// ```
    /// ```js,expect_diagnostic
    /// for (; foo.length; ) {};
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const isNotEmpty = foo.length > 0;
    /// ```
    /// ```js
    /// if (foo.length > 0 || bar.length > 0) {}
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

        if LENGTH_MEMBER_NAMES
            .binary_search(&member_name.as_str())
            .is_err()
        {
            return None;
        }

        // TODO. Handle cases when `length` property is not numeric
        // That requires type inference. Example: `{ length: "not a number" }`

        let member_expr_syntax = node.syntax();
        let parent_syntax = member_expr_syntax.parent()?;

        if let Some((binary_expr, mut len_check, position)) =
            is_invalid_binary_expr_length_check(&parent_syntax)
        {
            return get_boolean_ancestor(binary_expr.syntax())
                .map(|(expr, is_negative)| {
                    if is_negative {
                        len_check = len_check.opposite();
                    }

                    UseExplicitLengthCheckState {
                        check: len_check,
                        suggested_fix: LengthFix::ReplaceWhole(expr),
                        member_name: member_name.clone(),
                    }
                })
                .or_else(|| {
                    Some(UseExplicitLengthCheckState {
                        check: len_check,
                        suggested_fix: LengthFix::ModifyBinaryExpression(
                            binary_expr.clone(),
                            position,
                        ),
                        member_name: member_name.clone(),
                    })
                });
        }

        // `if (foo.length) {}` `while(foo.length) {}` `do {} while(foo.length)` `for(; foo.length; ) {}`
        if is_in_boolean_context(member_expr_syntax).unwrap_or(false) {
            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::NonZero,
                suggested_fix: LengthFix::ReplaceWhole(AnyJsExpression::cast_ref(
                    member_expr_syntax,
                )?),
                member_name,
            });
        }

        if let Some(logical_expr) = is_logical_expr(&parent_syntax) {
            // `const x = foo.length || 0` is valid case
            // TODO. This handles simple cases. To know if right side is a number, we need type inference.
            if logical_expr.operator().ok()? == JsLogicalOperator::LogicalOr
                && logical_expr.right().ok()?.syntax().kind()
                    == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION
            {
                return None;
            }

            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::NonZero,
                suggested_fix: LengthFix::ReplaceWhole(AnyJsExpression::cast_ref(
                    member_expr_syntax,
                )?),
                member_name,
            });
        }

        if let Some((boolean_expr, is_negative)) = get_boolean_ancestor(member_expr_syntax) {
            return Some(UseExplicitLengthCheckState {
                check: LengthCheck::boolean_condition(is_negative),
                suggested_fix: LengthFix::ReplaceWhole(boolean_expr),
                member_name,
            });
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (code, type_text) = match state.check {
            LengthCheck::Zero => ("=== 0", "zero"),
            LengthCheck::NonZero => ("> 0", "not zero"),
        };

        let span = match &state.suggested_fix {
            LengthFix::ReplaceWhole(node) => node.range(),
            LengthFix::ModifyBinaryExpression(node, _) => node.range(),
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
        let member_expr = ctx.query();
        let mut mutation = ctx.root().begin();

        match &state.suggested_fix {
            LengthFix::ReplaceWhole(node) => {
                let operator_syntax = match state.check {
                    LengthCheck::Zero => T![===],
                    LengthCheck::NonZero => T![>],
                };

                let new_binary_expr = make::js_binary_expression(
                    member_expr.clone().trim_trailing_trivia()?.into(),
                    make::token_decorated_with_space(operator_syntax),
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsNumberLiteralExpression(
                            make::js_number_literal_expression(make::js_number_literal("0")),
                        ),
                    ),
                );

                let mut new_node = new_binary_expr.into_syntax();
                let parent = node.syntax().parent()?;
                // In cases like `export default!foo.length` -> `export default foo.length === 0`
                // We need to add a space between keyword and new expression
                if matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                        | JsSyntaxKind::JS_INSTANCEOF_EXPRESSION
                        | JsSyntaxKind::JS_YIELD_EXPRESSION
                        | JsSyntaxKind::JS_RETURN_STATEMENT
                        | JsSyntaxKind::JS_THROW_STATEMENT
                        | JsSyntaxKind::JS_NEW_EXPRESSION
                        | JsSyntaxKind::JS_AWAIT_EXPRESSION    
                        | JsSyntaxKind::JS_IN_EXPRESSION
                        | JsSyntaxKind::JS_FOR_OF_STATEMENT
                        | JsSyntaxKind::JS_FOR_IN_STATEMENT
                        | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                        | JsSyntaxKind::JS_CASE_CLAUSE
                ) || does_unary_expr_needs_space(&parent) {
                    let leading_trivia = make::token_decorated_with_space(T![=])
                        .leading_trivia()
                        .pieces();

                    new_node = new_node
                        .trim_leading_trivia()?
                        .prepend_trivia_pieces(leading_trivia)?;
                }

                dbg!("YEAH", parent.kind());

                mutation.replace_element_discard_trivia(
                    node.clone().into_syntax().into(),
                    new_node.into(),
                );

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::MaybeIncorrect,
                    message: markup! { "Replace with" }.to_owned(),
                    mutation,
                })
            }
            LengthFix::ModifyBinaryExpression(binary_expr, member_position) => {
                let operator_syntax = match state.check {
                    LengthCheck::Zero => T![===],
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
    suggested_fix: LengthFix,
    member_name: String,
}

#[derive(PartialEq, Clone, Copy)]
enum MemberPosition {
    Left,
    Right,
}

#[derive(PartialEq, Clone, Copy)]
enum LengthCheck {
    Zero,
    NonZero,
}

impl LengthCheck {
    fn opposite(&self) -> Self {
        match self {
            LengthCheck::Zero => LengthCheck::NonZero,
            LengthCheck::NonZero => LengthCheck::Zero,
        }
    }
    fn boolean_condition(is_negative: bool) -> Self {
        if is_negative {
            LengthCheck::Zero
        } else {
            LengthCheck::NonZero
        }
    }
}

enum LengthFix {
    ReplaceWhole(AnyJsExpression),
    ModifyBinaryExpression(JsBinaryExpression, MemberPosition),
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

fn is_invalid_binary_expr_length_check(
    node: &JsSyntaxNode,
) -> Option<(JsBinaryExpression, LengthCheck, MemberPosition)> {
    let binary_expr = JsBinaryExpression::cast_ref(node)?;
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
        return Some((binary_expr, LengthCheck::Zero, member_position));
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
        return Some((binary_expr, LengthCheck::NonZero, member_position));
    }

    None
}

/// Get the boolean ancestor of the node
/// ## Example
/// Includes following cases:
/// - `Boolean(x)`
/// - `!!x`
/// - `!x`
/// - `!(x)`
/// - `!(!x)`
/// and combination of them nested
/// ```js
/// !(Boolean(!(!x)))
/// ```
/// Returns ancestor expression and whether it is negated
fn get_boolean_ancestor(node: &JsSyntaxNode) -> Option<(AnyJsExpression, bool)> {
    let mut boolean_node: Option<JsSyntaxNode> = None;
    let mut current_node = node.parent()?;
    let mut is_negative = false;

    loop {
        if let Some(expr) = is_boolean_call(&current_node) {
            let syntax = expr.syntax().clone();
            current_node = syntax.parent()?;
            boolean_node = Some(syntax);
        } else if let Some(expr) = is_negation(&current_node) {
            let syntax = expr.syntax().clone();
            current_node = syntax.parent()?;
            boolean_node = Some(syntax);
            is_negative = !is_negative;
        } else if current_node.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            current_node = current_node.parent()?;
        } else {
            break;
        }
    }

    Some((AnyJsExpression::cast(boolean_node?)?, is_negative))
}

/// Check if the SyntaxNode is a `Boolean` Call Expression
/// ## Example
/// ```js
/// Boolean(x)
/// ```
pub fn is_boolean_call(node: &JsSyntaxNode) -> Option<JsCallExpression> {
    let expr = JsCallArgumentList::cast_ref(node)?
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;

    if expr.has_callee("Boolean") && expr.arguments().ok()?.args().len() < 2 {
        Some(expr)
    } else {
        None
    }
}

fn is_logical_expr(node: &JsSyntaxNode) -> Option<JsLogicalExpression> {
    let expr: JsLogicalExpression = JsLogicalExpression::cast_ref(node)?;

    match expr.operator().ok()? {
        JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr => Some(expr),
        _ => None,
    }
}

fn does_unary_expr_needs_space(node: &JsSyntaxNode) -> bool {
    JsUnaryExpression::cast_ref(node).and_then(|expr| expr.operator().ok()).and_then(|operator| match operator {
        JsUnaryOperator::Typeof | JsUnaryOperator::Void | JsUnaryOperator::Delete => Some(()),
        _ => None,
    }).is_some()
}

#[test]
fn test_order() {
    for items in LENGTH_MEMBER_NAMES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
