use crate::JsRuleAction;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression, T,
};
use biome_js_type_info::{ResolvedTypeData, Type, TypeData};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use crate::services::typed::Typed;

declare_lint_rule! {
    /// Prefer `Array#includes()` over `Array#indexOf()` checks.
    ///
    /// `Array#indexOf()` returns a numeric index and is commonly compared against `-1` to check
    /// for the presence of an element. `Array#includes()` is more readable and expressive for
    /// this purpose, and avoids off-by-one mistakes with the comparison operator.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid1.ts
    /// const arr = [1, 2, 3];
    /// arr.indexOf(1) !== -1;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid2.ts
    /// const arr = [1, 2, 3];
    /// arr.indexOf(1) >= 0;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid3.ts
    /// const arr = [1, 2, 3];
    /// arr.indexOf(1) === -1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const arr = [1, 2, 3];
    /// arr.includes(1);
    /// ```
    ///
    /// ```ts
    /// const arr = [1, 2, 3];
    /// // Positional use of indexOf is fine
    /// const pos = arr.indexOf(1);
    /// ```
    ///
    pub UseIncludes {
        version: "next",
        name: "useIncludes",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("prefer-includes").inspired()],
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Unsafe,
    }
}

/// Whether the pattern represents a presence or absence check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckKind {
    /// `arr.indexOf(x) !== -1` → `arr.includes(x)`
    Includes,
    /// `arr.indexOf(x) === -1` → `!arr.includes(x)`
    NotIncludes,
}

pub struct UseIncludesState {
    /// The full binary expression to replace.
    pub binary: JsBinaryExpression,
    /// The inner `indexOf(...)` call expression.
    pub index_of_call: JsCallExpression,
    pub kind: CheckKind,
}

use biome_rule_options::use_includes::UseIncludesOptions;

impl Rule for UseIncludes {
    type Query = Typed<JsBinaryExpression>;
    type State = UseIncludesState;
    type Signals = Option<Self::State>;
    type Options = UseIncludesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binary = ctx.query();
        detect_index_of_pattern(ctx, binary)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let preferred = match state.kind {
            CheckKind::Includes => "includes()",
            CheckKind::NotIncludes => "!...includes()",
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.binary.range(),
                markup! {
                    "Use "<Emphasis>{preferred}</Emphasis>" instead of "<Emphasis>"indexOf()"</Emphasis>" to check for presence."
                },
            )
            .note(markup! {
                <Emphasis>"includes()"</Emphasis>" is more readable and clearly expresses intent."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call = &state.index_of_call;
        let callee = call.callee().ok()?;
        let member = callee.as_js_static_member_expression()?;
        let object = member.object().ok()?;

        let args = call.arguments().ok()?;
        let search_arg = args.args().iter().next()?.ok()?;

        let includes_call = make::js_call_expression(
            make::js_static_member_expression(
                object,
                make::token(T![.]),
                make::js_name(make::ident("includes")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list([search_arg], []),
                make::token(T![')']),
            ),
        )
        .build();

        let replacement = match state.kind {
            CheckKind::Includes => AnyJsExpression::JsCallExpression(includes_call),
            CheckKind::NotIncludes => {
                AnyJsExpression::JsUnaryExpression(make::js_unary_expression(
                    make::token(T![!]),
                    AnyJsExpression::JsCallExpression(includes_call),
                ))
            }
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsBinaryExpression(state.binary.clone()),
            replacement,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state.kind {
                CheckKind::Includes => {
                    markup! { "Replace with "<Emphasis>".includes()"</Emphasis>"." }.to_owned()
                }
                CheckKind::NotIncludes => {
                    markup! { "Replace with "<Emphasis>"!...includes()"</Emphasis>"." }.to_owned()
                }
            },
            mutation,
        ))
    }
}

/// Attempts to detect the pattern `expr.indexOf(value) OP literal` or the
/// reversed form `literal OP expr.indexOf(value)` and normalise it so that
/// the left-hand side is always the `indexOf` call.
fn detect_index_of_pattern(
    ctx: &RuleContext<UseIncludes>,
    binary: &JsBinaryExpression,
) -> Option<UseIncludesState> {
    let operator = binary.operator().ok()?;
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    // Try both orientations: `indexOf OP literal` and `literal OP indexOf`.
    if let Some(call) = as_index_of_call(&left) {
        if !ensure_known_includes_type(ctx, &call) {
            return None;
        }
        let normalized_op = operator;
        return try_match_operator(binary, call, &right, normalized_op);
    }

    if let Some(call) = as_index_of_call(&right) {
        if !ensure_known_includes_type(ctx, &call) {
            return None;
        }
        // Swap the operator direction so the rest of the logic stays symmetric.
        let swapped = swap_operator(operator)?;
        return try_match_operator(binary, call, &left, swapped);
    }

    None
}

/// Returns `Some(call)` when `expr` is a call to `something.indexOf(...)`.
fn as_index_of_call(expr: &AnyJsExpression) -> Option<JsCallExpression> {
    let binding = expr.clone().omit_parentheses();
    let call = binding.as_js_call_expression()?.clone();
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    let js_name = name.as_js_name()?;
    if js_name.value_token().ok()?.text_trimmed() != "indexOf" {
        return None;
    }
    // Must have exactly one argument (the search value). If a `fromIndex` is
    // supplied we leave it alone because `includes(value, fromIndex)` has
    // different semantics from `indexOf(value, fromIndex) !== -1` when
    // `fromIndex` is negative.
    let args = call.arguments().ok()?;
    if args.args().len() != 1 {
        return None;
    }
    Some(call)
}

/// Given the `indexOf` call on the left and the literal on the right,
/// decide whether the comparison is a presence check, an absence check, or
/// something we should not touch.
fn try_match_operator(
    binary: &JsBinaryExpression,
    call: JsCallExpression,
    other: &AnyJsExpression,
    operator: JsBinaryOperator,
) -> Option<UseIncludesState> {
    let kind = match operator {
        // indexOf !== -1 | indexOf != -1 | indexOf >= 0 | indexOf > -1  →  includes
        JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality
            if is_negative_one(other) =>
        {
            CheckKind::Includes
        }
        JsBinaryOperator::GreaterThanOrEqual if is_zero(other) => CheckKind::Includes,
        JsBinaryOperator::GreaterThan if is_negative_one(other) => CheckKind::Includes,

        // indexOf === -1 | indexOf == -1 | indexOf < 0 | indexOf <= -1 →  !includes
        JsBinaryOperator::StrictEquality
        | JsBinaryOperator::Equality
        | JsBinaryOperator::LessThanOrEqual
            if is_negative_one(other) =>
        {
            CheckKind::NotIncludes
        }
        JsBinaryOperator::LessThan if is_zero(other) => CheckKind::NotIncludes,

        _ => return None,
    };

    Some(UseIncludesState {
        binary: binary.clone(),
        index_of_call: call,
        kind,
    })
}

/// Flips a comparison operator for when `indexOf` is on the right-hand side.
fn swap_operator(op: JsBinaryOperator) -> Option<JsBinaryOperator> {
    Some(match op {
        JsBinaryOperator::GreaterThan => JsBinaryOperator::LessThan,
        JsBinaryOperator::GreaterThanOrEqual => JsBinaryOperator::LessThanOrEqual,
        JsBinaryOperator::LessThan => JsBinaryOperator::GreaterThan,
        JsBinaryOperator::LessThanOrEqual => JsBinaryOperator::GreaterThanOrEqual,
        // Equality operators are symmetric.
        JsBinaryOperator::StrictEquality => JsBinaryOperator::StrictEquality,
        JsBinaryOperator::Equality => JsBinaryOperator::Equality,
        JsBinaryOperator::StrictInequality => JsBinaryOperator::StrictInequality,
        JsBinaryOperator::Inequality => JsBinaryOperator::Inequality,
        _ => return None,
    })
}

fn is_negative_one(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();

    // Handle the literal `-1` written directly (parsed as a number literal
    // with value -1.0 in some versions of the parser).
    if let Some(n) = as_number_literal(&expr) {
        return n == -1.0;
    }

    // Handle `-1` written as unary minus applied to `1`.
    let Some(unary) = expr.as_js_unary_expression() else {
        return false;
    };
    let is_minus = unary
        .operator_token()
        .ok()
        .is_some_and(|t| t.kind() == biome_js_syntax::JsSyntaxKind::MINUS);
    if !is_minus {
        return false;
    }
    unary
        .argument()
        .ok()
        .and_then(|arg| as_number_literal(&arg))
        .is_some_and(|n| n == 1.0)
}

fn is_zero(expr: &AnyJsExpression) -> bool {
    as_number_literal(&expr.clone().omit_parentheses()).is_some_and(|n| n == 0.0)
}

fn as_number_literal(expr: &AnyJsExpression) -> Option<f64> {
    expr.clone()
        .omit_parentheses()
        .as_any_js_literal_expression()
        .and_then(|lit| lit.as_js_number_literal_expression().cloned())
        .and_then(|n| n.as_number())
}

fn ensure_known_includes_type(ctx: &RuleContext<UseIncludes>, call: &JsCallExpression) -> bool {
    let callee = call.callee().ok();
    let member = callee.as_ref().and_then(|c| c.as_js_static_member_expression());
    let object = member.and_then(|m| m.object().ok());
    
    let Some(object) = object else {
        return false;
    };

    all_type_variants_match(&ctx.type_of_expression(&object), |current, raw| {
        current.is_string_or_string_literal() || current.is_array_of(|_| true) || matches!(raw, TypeData::Tuple(_))
    })
}

fn all_type_variants_match(ty: &Type, mut predicate: impl FnMut(&Type, &TypeData) -> bool) -> bool {
    let mut saw_variant = false;
    let mut pending = vec![ty.clone()];

    while let Some(current) = pending.pop() {
        if current.is_union() {
            let mut variants = current.flattened_union_variants().peekable();
            if variants.peek().is_none() {
                return false;
            }
            saw_variant = true;
            pending.extend(variants);
            continue;
        }

        let Some(raw) = current.resolved_data().map(ResolvedTypeData::as_raw_data) else {
            return false;
        };

        match raw {
            TypeData::Generic(generic) if generic.constraint.is_known() => {
                let Some(constraint) = current.resolve(&generic.constraint) else {
                    return false;
                };
                pending.push(constraint);
            }
            TypeData::Generic(_) => return false,
            _ if predicate(&current, raw) => saw_variant = true,
            _ => return false,
        }
    }

    saw_variant
}

