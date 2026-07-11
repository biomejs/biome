use crate::JsRuleAction;
use crate::services::typed::Typed;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    JsArrowFunctionExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression,
    JsFunctionBody, JsFunctionExpression, JsIdentifierBinding, JsParameters, JsReferenceIdentifier,
    T,
};
use biome_js_type_info::{ResolvedTypeData, Type, TypeData};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union};

declare_lint_rule! {
    /// Prefer `Array#includes()` over `Array#indexOf()` checks.
    ///
    /// `Array#indexOf()` and `Array#lastIndexOf()` return a numeric index and are commonly
    /// compared against `-1` to check for the presence of an element. Similarly,
    /// `Array#some()` is sometimes used with a strict equality comparison to test for
    /// presence. `Array#includes()` is more readable and expressive for this purpose,
    /// and avoids off-by-one mistakes with the comparison operator.
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
    /// ```ts,expect_diagnostic,file=invalid4.ts
    /// const arr = [1, 2, 3];
    /// arr.lastIndexOf(1) !== -1;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid5.ts
    /// const arr = [1, 2, 3];
    /// arr.some(x => x === 1);
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
    /// ```ts
    /// const arr = [1, 2, 3];
    /// // Non-equality predicates cannot be expressed with includes()
    /// arr.some(x => x > 1);
    /// ```
    ///
    pub UseIncludes {
        version: "2.5.0",
        name: "useIncludes",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintTypeScript("prefer-includes").inspired(),
            RuleSource::EslintUnicorn("prefer-includes").inspired(),
        ],
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseIncludesExpression = JsBinaryExpression | JsCallExpression
}

/// Whether the pattern represents a presence or absence check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckKind {
    /// `arr.indexOf(x) !== -1` → `arr.includes(x)`
    Includes,
    /// `arr.indexOf(x) === -1` → `!arr.includes(x)`
    NotIncludes,
}

/// Which index-returning method is being compared against `-1`/`0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexOfMethod {
    IndexOf,
    LastIndexOf,
}

impl IndexOfMethod {
    const fn as_str(self) -> &'static str {
        match self {
            Self::IndexOf => "indexOf()",
            Self::LastIndexOf => "lastIndexOf()",
        }
    }
}

pub enum UseIncludesState {
    /// `expr.indexOf(x) OP literal` (or `lastIndexOf`)
    IndexOf {
        /// The full binary expression to replace.
        binary: JsBinaryExpression,
        /// The inner `indexOf(...)`/`lastIndexOf(...)` call expression.
        call: JsCallExpression,
        method: IndexOfMethod,
        kind: CheckKind,
    },
    /// `expr.some(item => item === search)`
    Some {
        /// The full `some(...)` call expression to replace.
        call: JsCallExpression,
        /// The value compared against the callback parameter.
        search: AnyJsExpression,
    },
}

use biome_rule_options::use_includes::UseIncludesOptions;

impl Rule for UseIncludes {
    type Query = Typed<AnyUseIncludesExpression>;
    type State = UseIncludesState;
    type Signals = Option<Self::State>;
    type Options = UseIncludesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyUseIncludesExpression::JsBinaryExpression(binary) => {
                detect_index_of_pattern(ctx, binary)
            }
            AnyUseIncludesExpression::JsCallExpression(call) => detect_some_pattern(ctx, call),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UseIncludesState::IndexOf {
                binary, method, ..
            } => {
                let method = method.as_str();

                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        binary.range(),
                        markup! {
                            "Checking the result of "<Emphasis>{method}</Emphasis>" against "<Emphasis>"-1"</Emphasis>" to test for presence."
                        },
                    )
                    .note(markup! {
                        <Emphasis>{method}</Emphasis>" returns a numeric index, not a boolean. Comparing it against "<Emphasis>"-1"</Emphasis>" is error-prone and harder to read."
                    }),
                )
            }
            UseIncludesState::Some { call, .. } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    call.range(),
                    markup! {
                        "Using "<Emphasis>"some()"</Emphasis>" with an equality comparison to test for presence."
                    },
                )
                .note(markup! {
                    "A callback that only checks equality is harder to read and less efficient than a direct membership test."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match state {
            UseIncludesState::IndexOf {
                binary, call, kind, ..
            } => {
                let args = call.arguments().ok()?;
                let search_arg = args.args().iter().next()?.ok()?;
                let includes_call = make_includes_call(call, search_arg)?;

                let replacement = match kind {
                    CheckKind::Includes => AnyJsExpression::JsCallExpression(includes_call),
                    CheckKind::NotIncludes => {
                        AnyJsExpression::JsUnaryExpression(make::js_unary_expression(
                            make::token(T![!]),
                            AnyJsExpression::JsCallExpression(includes_call),
                        ))
                    }
                };

                mutation.replace_node(
                    AnyJsExpression::JsBinaryExpression(binary.clone()),
                    replacement,
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    match kind {
                        CheckKind::Includes => {
                            markup! { "Use "<Emphasis>"includes()"</Emphasis>" instead." }.to_owned()
                        }
                        CheckKind::NotIncludes => {
                            markup! { "Use "<Emphasis>"!includes()"</Emphasis>" instead." }
                                .to_owned()
                        }
                    },
                    mutation,
                ))
            }
            UseIncludesState::Some { call, search } => {
                let search = search.clone().with_leading_trivia_pieces([])?.with_trailing_trivia_pieces([])?;
                let includes_call = make_includes_call(
                    call,
                    biome_js_syntax::AnyJsCallArgument::AnyJsExpression(search),
                )?;

                mutation.replace_node(
                    AnyJsExpression::JsCallExpression(call.clone()),
                    AnyJsExpression::JsCallExpression(includes_call),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"includes()"</Emphasis>" instead." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

/// Builds `object.includes(search)` where `object` is taken from the callee of
/// `call` and `search` becomes the single argument.
fn make_includes_call(
    call: &JsCallExpression,
    search: AnyJsCallArgument,
) -> Option<JsCallExpression> {
    // The callee may be wrapped in parentheses (e.g. `(arr.some)(...)`); the
    // parentheses are never semantically required around a call's callee.
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    let object = member.object().ok()?;

    Some(
        make::js_call_expression(
            make::js_static_member_expression(
                object,
                make::token(T![.]),
                make::js_name(make::ident("includes")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list([search], []),
                make::token(T![')']),
            ),
        )
        .build(),
    )
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
    if let Some((call, method)) = as_index_of_call(&left) {
        if !ensure_known_includes_type(ctx, &call) {
            return None;
        }
        return try_match_operator(binary, call, method, &right, operator);
    }

    if let Some((call, method)) = as_index_of_call(&right) {
        if !ensure_known_includes_type(ctx, &call) {
            return None;
        }
        // Swap the operator direction so the rest of the logic stays symmetric.
        let swapped = swap_operator(operator)?;
        return try_match_operator(binary, call, method, &left, swapped);
    }

    None
}

/// Attempts to detect the pattern `expr.some(item => item === search)`, where
/// the callback is a single-parameter arrow function or function expression
/// whose body is a strict-equality comparison against the parameter.
fn detect_some_pattern(
    ctx: &RuleContext<UseIncludes>,
    call: &JsCallExpression,
) -> Option<UseIncludesState> {
    // The callee may be parenthesized, e.g. `(arr).some(...)` or `(arr.some)(...)`.
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    if name.as_js_name()?.value_token().ok()?.text_trimmed() != "some" {
        return None;
    }

    // Exactly one argument: the callback. A `thisArg` would be lost in the fix.
    let args = call.arguments().ok()?;
    if args.args().len() != 1 {
        return None;
    }
    let callback = args.args().iter().next()?.ok()?;
    let callback = callback.as_any_js_expression()?.clone().omit_parentheses();

    let (param, comparison) = match &callback {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow_param_and_comparison(arrow)?,
        AnyJsExpression::JsFunctionExpression(function) => function_param_and_comparison(function)?,
        _ => return None,
    };

    let search = comparison_search_value(ctx, &comparison, &param)?;

    // `includes()` only exists on arrays and tuples in this form; `some()`
    // does not exist on strings, so restrict to array-like receivers. The
    // receiver may be parenthesized (`(arr).some(...)`), so unwrap it before
    // resolving its type.
    let object = member.object().ok()?.omit_parentheses();
    if !all_type_variants_match(&ctx.type_of_expression(&object), |current, raw| {
        current.is_array_of(|_| true) || matches!(raw, TypeData::Tuple(_))
    }) {
        return None;
    }

    Some(UseIncludesState::Some {
        call: call.clone(),
        search,
    })
}

/// Extracts the single identifier parameter and the strict-equality comparison
/// from an arrow callback like `x => x === search` or `(x) => { return x === search; }`.
fn arrow_param_and_comparison(
    arrow: &JsArrowFunctionExpression,
) -> Option<(JsIdentifierBinding, JsBinaryExpression)> {
    // An async callback returns a promise, so `some(async ...)` is not a plain
    // membership test and cannot be expressed with `includes()`.
    if arrow.async_token().is_some() {
        return None;
    }
    let param = match arrow.parameters().ok()? {
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
            binding.as_js_identifier_binding()?.clone()
        }
        AnyJsArrowFunctionParameters::JsParameters(params) => single_identifier_parameter(&params)?,
    };
    let comparison = match arrow.body().ok()? {
        AnyJsFunctionBody::AnyJsExpression(expr) => as_strict_equality(&expr)?,
        AnyJsFunctionBody::JsFunctionBody(body) => as_strict_equality(&single_return_value(&body)?)?,
    };
    Some((param, comparison))
}

/// Extracts the single identifier parameter and the strict-equality comparison
/// from a function-expression callback like `function (x) { return x === search; }`.
fn function_param_and_comparison(
    function: &JsFunctionExpression,
) -> Option<(JsIdentifierBinding, JsBinaryExpression)> {
    // Async callbacks return a promise and generator callbacks return an
    // iterator, so neither is a plain membership predicate.
    if function.async_token().is_some() || function.star_token().is_some() {
        return None;
    }
    let param = single_identifier_parameter(&function.parameters().ok()?)?;
    let comparison = as_strict_equality(&single_return_value(&function.body().ok()?)?)?;
    Some((param, comparison))
}

/// Returns the parameter binding when `params` holds exactly one plain
/// identifier parameter without a default value or rest/destructuring.
fn single_identifier_parameter(params: &JsParameters) -> Option<JsIdentifierBinding> {
    let items = params.items();
    if items.len() != 1 {
        return None;
    }
    let param = items.iter().next()?.ok()?;
    let formal = param.as_any_js_formal_parameter()?.as_js_formal_parameter()?;
    if formal.initializer().is_some() {
        return None;
    }
    Some(
        formal
            .binding()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .clone(),
    )
}

/// Returns the returned expression when `body` consists of a single
/// `return <expr>;` statement.
fn single_return_value(body: &JsFunctionBody) -> Option<AnyJsExpression> {
    let statements = body.statements();
    if statements.len() != 1 {
        return None;
    }
    statements
        .iter()
        .next()?
        .as_js_return_statement()?
        .argument()
}

fn as_strict_equality(expr: &AnyJsExpression) -> Option<JsBinaryExpression> {
    let binary = expr
        .clone()
        .omit_parentheses()
        .as_js_binary_expression()?
        .clone();
    // Only `===` is safe to rewrite. `includes()` compares with SameValueZero,
    // which matches `===` (aside from `NaN`), whereas loose `==` performs type
    // coercion: `[1].some(x => x == "1")` is `true` but `[1].includes("1")` is
    // `false`, so `==` must not be rewritten.
    if binary.operator().ok()? == JsBinaryOperator::StrictEquality {
        Some(binary)
    } else {
        None
    }
}

/// Given `param === search` (in either orientation), returns the search value,
/// rejecting comparisons where the search value itself references the parameter.
fn comparison_search_value(
    ctx: &RuleContext<UseIncludes>,
    comparison: &JsBinaryExpression,
    param: &JsIdentifierBinding,
) -> Option<AnyJsExpression> {
    let left = comparison.left().ok()?;
    let right = comparison.right().ok()?;

    let search = if is_reference_to(ctx, &left, param) {
        right
    } else if is_reference_to(ctx, &right, param) {
        left
    } else {
        return None;
    };

    // The search value must not depend on the callback parameter — `includes()`
    // takes a fixed value, so `arr.some(x => x === f(x))` cannot be rewritten.
    if references_binding(ctx, &search, param) {
        return None;
    }
    Some(search)
}

/// Whether `expr` is exactly a reference that resolves to the `param` binding.
fn is_reference_to(
    ctx: &RuleContext<UseIncludes>,
    expr: &AnyJsExpression,
    param: &JsIdentifierBinding,
) -> bool {
    expr.clone()
        .omit_parentheses()
        .as_js_identifier_expression()
        .and_then(|ident| ident.name().ok())
        .is_some_and(|reference| resolves_to(ctx, &reference, param))
}

/// Whether any reference within `expr` resolves to the `param` binding.
fn references_binding(
    ctx: &RuleContext<UseIncludes>,
    expr: &AnyJsExpression,
    param: &JsIdentifierBinding,
) -> bool {
    expr.syntax()
        .descendants()
        .filter_map(JsReferenceIdentifier::cast)
        .any(|reference| resolves_to(ctx, &reference, param))
}

/// Whether `reference` resolves to the `param` binding via the semantic model.
/// A name comparison would misfire on a shadowing binding of the same name.
fn resolves_to(
    ctx: &RuleContext<UseIncludes>,
    reference: &JsReferenceIdentifier,
    param: &JsIdentifierBinding,
) -> bool {
    ctx.binding_of(reference)
        .is_some_and(|binding| &binding == param.syntax())
}

/// Returns `Some((call, method))` when `expr` is a call to
/// `something.indexOf(...)` or `something.lastIndexOf(...)`.
fn as_index_of_call(expr: &AnyJsExpression) -> Option<(JsCallExpression, IndexOfMethod)> {
    let binding = expr.clone().omit_parentheses();
    let call = binding.as_js_call_expression()?.clone();
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    let js_name = name.as_js_name()?;
    let method = match js_name.value_token().ok()?.text_trimmed() {
        "indexOf" => IndexOfMethod::IndexOf,
        "lastIndexOf" => IndexOfMethod::LastIndexOf,
        _ => return None,
    };
    // Must have exactly one argument (the search value). If a `fromIndex` is
    // supplied we leave it alone because `includes(value, fromIndex)` has
    // different semantics from `indexOf(value, fromIndex) !== -1` when
    // `fromIndex` is negative.
    let args = call.arguments().ok()?;
    if args.args().len() != 1 {
        return None;
    }
    Some((call, method))
}

/// Given the `indexOf` call on the left and the literal on the right,
/// decide whether the comparison is a presence check, an absence check, or
/// something we should not touch.
fn try_match_operator(
    binary: &JsBinaryExpression,
    call: JsCallExpression,
    method: IndexOfMethod,
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

    Some(UseIncludesState::IndexOf {
        binary: binary.clone(),
        call,
        method,
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
