use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsAwaitExpression, JsBinaryExpression,
    JsBinaryOperator, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsExtendsClause, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalExpression, JsLogicalOperator, JsNewExpression,
    JsParenthesizedExpression, JsSpread, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsTemplateExpression, JsUnaryExpression, JsUnaryOperator, JsxSpreadAttribute, JsxSpreadChild,
    T, TsAsExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
    TsTypeAssertionExpression,
    binary_like_expression::{AnyJsBinaryLikeExpression, BinaryLikeOperator},
    is_in_boolean_context,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, TokenText};
use biome_rule_options::use_explicit_length_check::UseExplicitLengthCheckOptions;

use crate::{
    JsRuleAction, ast_utils::needs_space_before_identifier_expression_replacement,
};

declare_lint_rule! {
    /// Enforce explicitly comparing the `length`, `size`, `byteLength` or `byteOffset` property of a value.
    ///
    /// This rule enforces a specific style length comparisons to make them more clear.
    ///
    /// ## Zero comparison examples
    ///
    /// Enforce comparison with `=== 0` when checking for zero length.
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
    ///
    /// Enforce comparison with `> 0` when checking for non-zero length.
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
    /// if (foo.length) {}
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
    /// ## Caveats
    ///
    /// This rule assumes that the `length`/`size` property is always numeric, even if it actually is not.
    /// In the example below the rule will trigger a warning, even though the `size` property is a string.
    ///
    /// ```js,expect_diagnostic
    /// const foo1 = { size: "small" }; if (foo1.size) {}
    /// ```
    ///
    /// To properly handle this case, type inference would be required, which is not supported by Biome at the moment.
    /// We recommend disabling this rule when working with non-numeric `length`/`size` properties.
    ///
    pub UseExplicitLengthCheck {
        version: "1.7.3",
        name: "useExplicitLengthCheck",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        sources: &[RuleSource::EslintUnicorn("explicit-length-check").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseExplicitLengthCheck {
    type Query = Ast<JsStaticMemberExpression>;
    type State = UseExplicitLengthCheckState;
    type Signals = Option<Self::State>;
    type Options = UseExplicitLengthCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member_expr = ctx.query();
        let member_name = member_expr.member().ok()?;
        let member_name = member_name
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed();

        if !LENGTH_MEMBER_NAMES.contains(&member_name.text()) {
            return None;
        }

        if member_expr.is_optional_chain() {
            return None;
        }

        // TODO. Handle cases when `length` property is not numeric
        // That requires type inference. Example: `{ length: "not a number" }`

        let member_expression = AnyJsExpression::from(member_expr.clone());
        let member_expr_syntax = member_expression.syntax();

        if let Some((binary_expr, mut len_check, is_possibly_valid)) =
            member_expr
                .parent::<JsBinaryExpression>()
                .and_then(|binary_expr| is_binary_expr_length_check(&binary_expr))
        {
            return get_boolean_ancestor(&AnyJsExpression::from(binary_expr.clone()))
                .map(|(expr, is_negative)| {
                    if is_negative {
                        len_check = len_check.opposite();
                    }

                    UseExplicitLengthCheckState::new(
                        len_check,
                        expr,
                        member_name.clone(),
                    )
                })
                .or_else(|| {
                    // Binary expression is valid and was not wrapped in a boolean ancestor
                    if is_possibly_valid {
                        return None;
                    }

                    Some(UseExplicitLengthCheckState::new(
                        len_check,
                        AnyJsExpression::from(binary_expr),
                        member_name,
                    ))
                });
        }

        if let Some((boolean_expr, is_negative)) = get_boolean_ancestor(&member_expression) {
            let check = if is_negative {
                LengthCheck::Zero
            } else {
                LengthCheck::NonZero
            };

            return Some(UseExplicitLengthCheckState::new(
                check,
                boolean_expr,
                member_name,
            ));
        }

        if is_in_boolean_context(member_expr_syntax).unwrap_or(false)
            || has_boolean_ancestor_through_logical_expression(&member_expression)
        {
            return Some(UseExplicitLengthCheckState::new(
                LengthCheck::NonZero,
                AnyJsExpression::cast_ref(member_expr_syntax)?,
                member_name,
            ));
        }

        if let Some(logical_expr) = get_parent_logical_expression(&member_expression) {
            if logical_expr.operator().ok()? != JsLogicalOperator::LogicalAnd {
                return None;
            }

            return Some(UseExplicitLengthCheckState::without_fix(
                LengthCheck::NonZero,
                AnyJsExpression::cast_ref(member_expr_syntax)?,
                member_name,
            ));
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (code, type_text) = match state.check {
            LengthCheck::Zero => ("=== 0", "zero"),
            LengthCheck::NonZero => ("> 0", "not zero"),
        };
        let member_name = state.member_name.text();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.node.range(),
            markup! {
                "Use "<Emphasis>"."{member_name}" "{code}</Emphasis>" when checking "<Emphasis>"."{member_name}</Emphasis>" is "{type_text}"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if !state.can_fix {
            return None;
        }

        let member_expr = ctx.query();
        let mut mutation = ctx.root().begin();
        let (operator_kind, operator) = match state.check {
            LengthCheck::Zero => (T![===], JsBinaryOperator::StrictEquality),
            LengthCheck::NonZero => (T![>], JsBinaryOperator::GreaterThan),
        };

        let member_expr = member_expr.clone().trim_trailing_trivia()?;
        let state_starts_before_member =
            state.node.syntax().first_token() != member_expr.syntax().first_token();
        let new_binary_expr = AnyJsExpression::from(make::js_binary_expression(
            member_expr.into(),
            make::token_decorated_with_space(operator_kind),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(
                    make::js_number_literal_expression(make::js_number_literal("0")),
                ),
            ),
        ));

        let mut new_node = if binary_replacement_needs_parentheses(&state.node, operator) {
            make::js_parenthesized_expression(
                make::token(T!['(']),
                new_binary_expr,
                make::token(T![')']),
            )
            .into_syntax()
        } else {
            new_binary_expr.into_syntax()
        };
        if state_starts_before_member {
            new_node = new_node.prepend_trivia_pieces(
                state
                    .node
                    .syntax()
                    .first_token()?
                    .leading_trivia()
                    .pieces(),
            )?;
        }
        // In cases like `export default!foo.length` -> `export default foo.length === 0`
        // we need to add a space between keyword and expression
        if needs_space_before_identifier_expression_replacement(&state.node) {
            // Make fake token to get leading trivia
            let leading_trivia = make::token_decorated_with_space(T![=])
                .leading_trivia()
                .pieces();

            new_node = new_node
                .trim_leading_trivia()?
                .prepend_trivia_pieces(leading_trivia)?;
        }

        mutation.replace_node_discard_trivia(
            state.node.clone(),
            AnyJsExpression::cast(new_node)?,
        );

        let code = match state.check {
            LengthCheck::Zero => "=== 0",
            LengthCheck::NonZero => "> 0",
        };
        let member_name = state.member_name.text();
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>"."{member_name}</Emphasis>" with "<Emphasis>"."{member_name}" "{code}</Emphasis> }.to_owned(),
            mutation,
        ))
    }
}

/// Sorted by how common they are in the wild
const LENGTH_MEMBER_NAMES: [&str; 4] = ["length", "size", "byteLength", "byteOffset"];

pub struct UseExplicitLengthCheckState {
    check: LengthCheck,
    node: AnyJsExpression,
    member_name: TokenText,
    can_fix: bool,
}

impl UseExplicitLengthCheckState {
    fn new(check: LengthCheck, node: AnyJsExpression, member_name: TokenText) -> Self {
        Self {
            check,
            can_fix: !is_unsafe_negation_fix(&node),
            node,
            member_name,
        }
    }

    fn without_fix(check: LengthCheck, node: AnyJsExpression, member_name: TokenText) -> Self {
        Self {
            check,
            node,
            member_name,
            can_fix: false,
        }
    }
}

enum MemberPosition {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum LengthCheck {
    Zero,
    NonZero,
}

impl LengthCheck {
    fn opposite(&self) -> Self {
        match self {
            Self::Zero => Self::NonZero,
            Self::NonZero => Self::Zero,
        }
    }
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

fn is_binary_expr_length_check(
    binary_expr: &JsBinaryExpression,
) -> Option<(JsBinaryExpression, LengthCheck, bool)> {
    let (member_position, literal) = extract_binary_position_and_literal(binary_expr)?;
    let number = literal
        .as_js_number_literal_expression()?
        .as_number()?
        .round() as i64;

    let (length_check, is_valid) = match (member_position, binary_expr.operator().ok()?, number) {
        // Zero length checks
        // -------------------------
        // `0 == foo.length` or `foo.length == 0`
        (MemberPosition::Left | MemberPosition::Right, JsBinaryOperator::Equality, 0) |
        // `foo.length < 1`
        (MemberPosition::Right, JsBinaryOperator::LessThan, 1) |
        // `1 > foo.length`
        (MemberPosition::Left, JsBinaryOperator::GreaterThan, 1) |
        // 0 === foo.length. We prefer right side to be a number
        (MemberPosition::Left, JsBinaryOperator::StrictEquality, 0) => Some((LengthCheck::Zero, false)),
        // `foo.length === 0`. Valid, but might still be wrapped in a boolean ancestor
        (MemberPosition::Right, JsBinaryOperator::StrictEquality, 0) => Some((LengthCheck::Zero, true)),
        // -------------------------
        // Non-zero length checks
        // -------------------------
        // `0 !== foo.length` or `foo.length !== 0` or
        // `0 != foo.length` or `foo.length != 0`
        (
            MemberPosition::Left | MemberPosition::Right,
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality,
            0,
        ) |
        // `foo.length >= 1`
        (MemberPosition::Right, JsBinaryOperator::GreaterThanOrEqual, 1) |
        // `1 <= foo.length`
        (MemberPosition::Left, JsBinaryOperator::LessThanOrEqual, 1) |
        // 0 < foo.length. We prefer right side to be a number
        (MemberPosition::Left, JsBinaryOperator::LessThan, 0) => Some((LengthCheck::NonZero, false)),
        // `foo.length > 0`. Valid, but might still be wrapped in a boolean ancestor
        (MemberPosition::Right, JsBinaryOperator::GreaterThan, 0) => Some((LengthCheck::NonZero, true)),
        _ => None,
    }?;

    Some((binary_expr.clone(), length_check, is_valid))
}

/// Get the boolean ancestor of the node
/// ## Example
/// Includes following cases:
/// - `Boolean(x)`
/// - `!!x`
/// - `!x`
/// - `!(x)`
/// - `!(!x)` and combination of them nested
/// ```js
/// !(Boolean(!(!x)))
/// ```
/// Returns ancestor expression and whether it is negated
fn get_boolean_ancestor(node: &AnyJsExpression) -> Option<(AnyJsExpression, bool)> {
    let mut boolean_node = None;
    let mut current_node = node.clone();
    let mut is_negative = false;

    loop {
        if let Some(expr) = is_boolean_call(&current_node) {
            current_node = expr.into();
            boolean_node = Some(current_node.clone());
        } else if let Some(expr) = current_node
            .parent::<JsUnaryExpression>()
            .filter(|expr| expr.operator() == Ok(JsUnaryOperator::LogicalNot))
        {
            current_node = expr.into();
            boolean_node = Some(current_node.clone());
            is_negative = !is_negative;
        } else if let Some(expr) = current_node.parent::<JsParenthesizedExpression>() {
            current_node = expr.into();
        } else {
            break;
        }
    }

    Some((boolean_node?, is_negative))
}

/// Returns the parent `Boolean` call when the expression is its only argument.
/// ## Example
/// ```js
/// Boolean(x)
/// ```
fn is_boolean_call(node: &AnyJsExpression) -> Option<JsCallExpression> {
    let expr = node
        .parent::<JsCallArgumentList>()?
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;
    (expr.has_callee("Boolean") && expr.arguments().ok()?.args().len() < 2).then_some(expr)
}

fn get_parent_logical_expression(expression: &AnyJsExpression) -> Option<JsLogicalExpression> {
    let logical_expression = expression
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsLogicalExpression::cast)?;
    let is_direct_operand = [
        logical_expression.left().ok()?,
        logical_expression.right().ok()?,
    ]
    .iter()
    .any(|operand| operand.clone().omit_parentheses().syntax() == expression.syntax());

    is_direct_operand.then_some(logical_expression)
}

fn has_boolean_ancestor_through_logical_expression(node: &AnyJsExpression) -> bool {
    let Some(logical_expression) = get_parent_logical_expression(node) else {
        return false;
    };
    let mut current_expression = AnyJsExpression::from(logical_expression);

    while let Some(parent) = get_parent_logical_expression(&current_expression) {
        current_expression = parent.into();
    }

    get_boolean_ancestor(&current_expression).is_some()
}

fn is_unsafe_negation_fix(node: &AnyJsExpression) -> bool {
    let Some(unary_expression) = JsUnaryExpression::cast_ref(node.syntax()) else {
        return false;
    };
    if unary_expression.operator() != Ok(JsUnaryOperator::LogicalNot) {
        return false;
    }

    node.syntax()
        .parent()
        .is_some_and(|parent| UNSAFE_NEGATION_FIX_PARENT_KINDS.matches(parent.kind()))
}

fn binary_replacement_needs_parentheses(
    node: &AnyJsExpression,
    replacement_operator: JsBinaryOperator,
) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };

    if BINARY_REPLACEMENT_PARENTHESES_PARENT_KINDS.matches(parent.kind()) {
        return true;
    }

    AnyJsBinaryLikeExpression::cast(parent)
        .and_then(|parent| parent.operator().ok())
        .is_some_and(|parent_operator| {
            parent_operator.precedence()
                >= BinaryLikeOperator::Binary(replacement_operator).precedence()
        })
}

const BINARY_REPLACEMENT_PARENTHESES_PARENT_KINDS: SyntaxKindSet<JsLanguage> =
    JsExtendsClause::KIND_SET
        .union(TsAsExpression::KIND_SET)
        .union(TsSatisfiesExpression::KIND_SET)
        .union(TsTypeAssertionExpression::KIND_SET)
        .union(JsUnaryExpression::KIND_SET)
        .union(JsAwaitExpression::KIND_SET)
        .union(TsNonNullAssertionExpression::KIND_SET)
        .union(JsxSpreadChild::KIND_SET)
        .union(JsSpread::KIND_SET)
        .union(JsxSpreadAttribute::KIND_SET)
        .union(JsCallExpression::KIND_SET)
        .union(JsNewExpression::KIND_SET)
        .union(JsTemplateExpression::KIND_SET)
        .union(JsStaticMemberExpression::KIND_SET)
        .union(JsStaticMemberAssignment::KIND_SET)
        .union(JsComputedMemberExpression::KIND_SET)
        .union(JsComputedMemberAssignment::KIND_SET);

const UNSAFE_NEGATION_FIX_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsBinaryExpression::KIND_SET
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET);
