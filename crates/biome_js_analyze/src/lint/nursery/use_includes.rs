use crate::{JsRuleAction, services::typed::Typed};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsCallArgument, AnyJsExpression,
    AnyJsFunctionBody, JsBinaryExpression, JsBinaryOperator, JsCallExpression, JsFunctionBody,
    JsParameters, JsReferenceIdentifier, JsSyntaxToken, T,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::use_includes::UseIncludesOptions;

declare_lint_rule! {
    /// Prefer `Array#includes()` over `Array#indexOf()`, `Array#lastIndexOf()`, and `Array#some()` when checking for existence or non-existence.
    ///
    /// `Array#indexOf()` and `Array#lastIndexOf()` return a numeric index and are commonly compared
    /// against `-1` to check for the presence of an element. `Array#some()` is sometimes used with a
    /// strict-equality callback for the same purpose. `Array#includes()` is more readable and
    /// expressive, and avoids off-by-one mistakes with the comparison operator.
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
    /// arr.some((item) => item === 1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const arr = [1, 2, 3];
    ///
    /// arr.includes(1);
    ///
    /// !arr.includes(1);
    ///
    /// // Positional use of indexOf is fine
    /// const pos = arr.indexOf(1);
    /// ```
    ///
    pub UseIncludes {
        version: "2.5.0",
        name: "useIncludes",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("prefer-includes").inspired(), RuleSource::EslintUnicorn("prefer-includes").inspired(), RuleSource::EslintE18e("prefer-includes").inspired()],
        domains: &[RuleDomain::Types],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseIncludes {
    type Query = Typed<AnyUseIncludesQuery>;
    type State = UseIncludesState;
    type Signals = Option<Self::State>;
    type Options = UseIncludesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyUseIncludesQuery::JsBinaryExpression(binary) => detect_index_of_pattern(ctx, binary),
            AnyUseIncludesQuery::JsCallExpression(call) => detect_some_pattern(ctx, call),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state.method {
            SourceMethod::Some => RuleDiagnostic::new(
                rule_category!(),
                state.node.range(),
                markup! {
                    "Using "<Emphasis>{state.method}</Emphasis>" with a strict-equality callback to test for presence."
                },
            )
            .note(markup! {
                "Use "<Emphasis>"includes()"</Emphasis>" instead, which directly expresses the intent and returns a boolean."
            }),
            SourceMethod::IndexOf | SourceMethod::LastIndexOf => {
                let preferred = match state.kind {
                    CheckKind::Includes => "includes()",
                    CheckKind::NotIncludes => "!...includes()",
                };
                RuleDiagnostic::new(
                    rule_category!(),
                    state.node.range(),
                    markup! {
                        "Checking the result of "<Emphasis>{state.method}</Emphasis>" against "<Emphasis>"-1"</Emphasis>" to test for presence."
                    },
                )
                .note(markup! {
                    <Emphasis>{state.method}</Emphasis>" returns a numeric index, not a boolean. Comparing it against "<Emphasis>"-1"</Emphasis>" is error-prone and harder to read."
                })
                .note(markup! {
                    "Use "<Emphasis>{preferred}</Emphasis>" instead, which directly expresses the intent and returns a boolean."
                })
            }
        };

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let includes_call = make::js_call_expression(
            make::js_static_member_expression(
                state.object.clone(),
                make::token(T![.]),
                make::js_name(make::ident("includes")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list([state.search_value.clone()], []),
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
        mutation.replace_node(state.node.clone(), replacement);

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

declare_node_union! {
    pub AnyUseIncludesQuery = JsBinaryExpression | JsCallExpression
}

#[derive(Clone, Copy)]
pub enum CheckKind {
    /// `arr.indexOf(x) !== -1` → `arr.includes(x)`
    Includes,
    /// `arr.indexOf(x) === -1` → `!arr.includes(x)`
    NotIncludes,
}

#[derive(Clone, Copy)]
pub enum SourceMethod {
    IndexOf,
    LastIndexOf,
    Some,
}

impl Display for SourceMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::io::Result<()> {
        f.write_str(match self {
            Self::IndexOf => "indexOf()",
            Self::LastIndexOf => "lastIndexOf()",
            Self::Some => "some()",
        })
    }
}

pub struct UseIncludesState {
    /// The expression to replace with an `includes()` call.
    node: AnyJsExpression,
    /// The receiver of the resulting `includes()` call.
    object: AnyJsExpression,
    /// The value passed to `includes()`.
    search_value: AnyJsCallArgument,
    /// The method being replaced, used by the diagnostic.
    method: SourceMethod,
    kind: CheckKind,
}

/// Attempts to detect the pattern `expr.indexOf(value) OP literal` or the
/// reversed form `literal OP expr.indexOf(value)` and normalise it so that
/// the left-hand side is always the `indexOf`/`lastIndexOf` call.
fn detect_index_of_pattern(
    ctx: &RuleContext<UseIncludes>,
    binary: &JsBinaryExpression,
) -> Option<UseIncludesState> {
    let operator = binary.operator().ok()?;
    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    // Try both orientations: `indexOf OP literal` and `literal OP indexOf`.
    let (call, method, other, normalized_op) = if let Some((call, method)) = as_index_of_call(&left)
    {
        (call, method, right, operator)
    } else if let Some((call, method)) = as_index_of_call(&right) {
        // Swap the operator direction so the rest of the logic stays symmetric.
        (call, method, left, swap_operator(operator)?)
    } else {
        return None;
    };

    let kind = match_operator(normalized_op, &other)?;

    if !ensure_known_includes_type(ctx, &call) {
        return None;
    }

    let (object, search_value) = index_of_object_and_argument(&call)?;

    Some(UseIncludesState {
        node: AnyJsExpression::JsBinaryExpression(binary.clone()),
        object,
        search_value,
        method,
        kind,
    })
}

/// Returns `Some((call, method))` when `expr` is a call to
/// `something.indexOf(...)` or `something.lastIndexOf(...)`.
fn as_index_of_call(expr: &AnyJsExpression) -> Option<(JsCallExpression, SourceMethod)> {
    let binding = expr.clone().omit_parentheses();
    let call = binding.as_js_call_expression()?.clone();
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    let js_name = name.as_js_name()?;
    let method = match js_name.value_token().ok()?.text_trimmed() {
        "indexOf" => SourceMethod::IndexOf,
        "lastIndexOf" => SourceMethod::LastIndexOf,
        _ => return None,
    };
    // Must have exactly one argument (the search value). If a `fromIndex` is
    // supplied we leave it alone because `includes(value, fromIndex)` has
    // different semantics from `indexOf(value, fromIndex) !== -1` when
    // `fromIndex` is negative. A spread argument is rejected because it can
    // expand into a `fromIndex` too, e.g. `lastIndexOf(...[value, fromIndex])`.
    let args = call.arguments().ok()?;
    let mut args = args.args().iter();
    let first = args.next()?.ok()?;
    if args.next().is_some() || first.as_js_spread().is_some() {
        return None;
    }
    Some((call, method))
}

/// Extracts the receiver and search argument from an `indexOf`/`lastIndexOf` call.
fn index_of_object_and_argument(
    call: &JsCallExpression,
) -> Option<(AnyJsExpression, AnyJsCallArgument)> {
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let object = member.object().ok()?;
    let args = call.arguments().ok()?;
    let search_arg = args.args().iter().next()?.ok()?;
    Some((object, search_arg))
}

/// Given the comparison operator (already normalised so the call is on the
/// left) and the other operand, decide whether the comparison is a presence
/// check, an absence check, or something we should not touch.
fn match_operator(operator: JsBinaryOperator, other: &AnyJsExpression) -> Option<CheckKind> {
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

    Some(kind)
}

/// Attempts to detect the pattern `expr.some((item) => item === value)` and
/// rewrite it into `expr.includes(value)`.
fn detect_some_pattern(
    ctx: &RuleContext<UseIncludes>,
    call: &JsCallExpression,
) -> Option<UseIncludesState> {
    let callee = call.callee().ok()?;
    let member = callee.as_js_static_member_expression()?;
    let name = member.member().ok()?;
    if name.as_js_name()?.value_token().ok()?.text_trimmed() != "some" {
        return None;
    }

    let args = call.arguments().ok()?;
    if args.args().len() != 1 {
        return None;
    }
    let callback = args.args().iter().next()?.ok()?;
    let callback = callback.as_any_js_expression()?;

    let callback_info = extract_some_callback(callback)?;
    let param = callback_info.param.text_trimmed();

    let left = callback_info.comparison.left().ok()?.omit_parentheses();
    let right = callback_info.comparison.right().ok()?.omit_parentheses();

    // Exactly one side of `===` must be the callback parameter; the other side
    // is the value we search for.
    let search_value = if is_reference_to(&left, param) {
        right
    } else if is_reference_to(&right, param) {
        left
    } else {
        return None;
    };

    // Bail if the searched value also references the parameter, e.g.
    // `arr.some((x) => x === f(x))`, which is not equivalent to `includes`.
    if references_name(&search_value, param) {
        return None;
    }

    // Bail if the searched value references a binding that resolves differently
    // at the `some()` call site, such as a named function expression's own name
    // or `this`/`arguments` inside a plain function callback.
    if references_rebound_binding(&search_value, &callback_info) {
        return None;
    }

    // `some()` may evaluate the search expression zero or many times, while the
    // generated `includes()` call evaluates it exactly once. Only rewrite when
    // the expression is side-effect-free, so the change in evaluation count is
    // not observable.
    if !is_repeatable_search_value(&search_value) {
        return None;
    }

    // `some()` compares with strict equality and skips holes, while
    // `includes()` uses SameValueZero and treats holes as `undefined`. When the
    // search value is `NaN` or `undefined`, the two are not equivalent, so bail.
    if is_nan_or_undefined(&search_value) {
        return None;
    }

    let object = member.object().ok()?;
    if !ctx
        .type_of_expression(&object)
        .is_some_and(|ty| ty.is_all_array_or_tuple())
    {
        return None;
    }

    let search_value = AnyJsExpression::cast(search_value.into_syntax().trim_trivia()?)?;

    Some(UseIncludesState {
        node: AnyJsExpression::JsCallExpression(call.clone()),
        object,
        search_value: AnyJsCallArgument::AnyJsExpression(search_value),
        method: SourceMethod::Some,
        kind: CheckKind::Includes,
    })
}

/// A `some()` callback recognised as a simple strict-equality comparison,
/// together with the bindings it introduces.
struct SomeCallback {
    /// The callback parameter compared against the search value.
    param: JsSyntaxToken,
    /// The strict-equality comparison forming the callback body.
    comparison: JsBinaryExpression,
    /// Name of a named function expression, in scope only inside the callback.
    self_name: Option<JsSyntaxToken>,
    /// Plain functions rebind `this` and `arguments`; arrows inherit them.
    rebinds_receiver: bool,
}

/// Recognises a callback shaped like `(item) => item === value` or its
/// block-bodied and function-expression equivalents.
fn extract_some_callback(callback: &AnyJsExpression) -> Option<SomeCallback> {
    match callback {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
            if arrow.async_token().is_some() {
                return None;
            }
            let param = match arrow.parameters().ok()? {
                AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                    binding_name_token(&binding)?
                }
                AnyJsArrowFunctionParameters::JsParameters(params) => single_param_token(&params)?,
            };
            let comparison = match arrow.body().ok()? {
                AnyJsFunctionBody::AnyJsExpression(expr) => as_strict_equality(&expr)?,
                AnyJsFunctionBody::JsFunctionBody(body) => single_return_equality(&body)?,
            };
            Some(SomeCallback {
                param,
                comparison,
                self_name: None,
                rebinds_receiver: false,
            })
        }
        AnyJsExpression::JsFunctionExpression(func) => {
            if func.async_token().is_some() || func.star_token().is_some() {
                return None;
            }
            let param = single_param_token(&func.parameters().ok()?)?;
            let comparison = single_return_equality(&func.body().ok()?)?;
            let self_name = func.id().as_ref().and_then(binding_name_token);
            Some(SomeCallback {
                param,
                comparison,
                self_name,
                rebinds_receiver: true,
            })
        }
        _ => None,
    }
}

/// Returns the name token of the single identifier parameter, or `None` when
/// there is not exactly one identifier parameter.
fn single_param_token(params: &JsParameters) -> Option<JsSyntaxToken> {
    let items = params.items();
    if items.len() != 1 {
        return None;
    }
    let param = items.iter().next()?.ok()?;
    let formal = param
        .as_any_js_formal_parameter()?
        .as_js_formal_parameter()?;
    // A default value changes which value the parameter holds, so the callback
    // is no longer a plain identity comparison against the visited element.
    if formal.initializer().is_some() {
        return None;
    }
    let binding = formal.binding().ok()?;
    binding_name_token(binding.as_any_js_binding()?)
}

fn binding_name_token(binding: &AnyJsBinding) -> Option<JsSyntaxToken> {
    binding.as_js_identifier_binding()?.name_token().ok()
}

/// Returns the comparison when `expr` is a strict-equality binary expression.
fn as_strict_equality(expr: &AnyJsExpression) -> Option<JsBinaryExpression> {
    let binary = expr.clone().omit_parentheses();
    let binary = binary.as_js_binary_expression()?.clone();
    if binary.operator().ok()? != JsBinaryOperator::StrictEquality {
        return None;
    }
    Some(binary)
}

/// Returns the strict-equality comparison of a body containing a single
/// `return <comparison>;` statement.
fn single_return_equality(body: &JsFunctionBody) -> Option<JsBinaryExpression> {
    let statements = body.statements();
    if statements.len() != 1 {
        return None;
    }
    let return_statement = statements.iter().next()?;
    let argument = return_statement.as_js_return_statement()?.argument()?;
    as_strict_equality(&argument)
}

/// Whether `expr` is an identifier reference to `name`.
fn is_reference_to(expr: &AnyJsExpression, name: &str) -> bool {
    expr.as_js_identifier_expression()
        .and_then(|ident| ident.name().ok())
        .and_then(|reference| reference.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == name)
}

/// Whether `expr` contains any identifier reference to `name`.
fn references_name(expr: &AnyJsExpression, name: &str) -> bool {
    expr.syntax().descendants().any(|node| {
        JsReferenceIdentifier::cast(node)
            .and_then(|reference| reference.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == name)
    })
}

/// Whether `expr` references a binding that resolves differently once the
/// expression is moved to the `some()` call site: a named function expression's
/// own name, or `this`/`arguments` inside a plain function callback.
fn references_rebound_binding(expr: &AnyJsExpression, callback: &SomeCallback) -> bool {
    expr.syntax().descendants().any(|node| {
        if callback.rebinds_receiver
            && matches!(
                node.kind(),
                biome_js_syntax::JsSyntaxKind::JS_THIS_EXPRESSION
                    | biome_js_syntax::JsSyntaxKind::JS_SUPER_EXPRESSION
            )
        {
            return true;
        }
        JsReferenceIdentifier::cast(node)
            .and_then(|reference| reference.value_token().ok())
            .is_some_and(|token| {
                let name = token.text_trimmed();
                callback
                    .self_name
                    .as_ref()
                    .is_some_and(|self_name| self_name.text_trimmed() == name)
                    || (callback.rebinds_receiver && name == "arguments")
            })
    })
}

/// Whether `expr` can be moved into an `includes()` call that evaluates it
/// exactly once. `some()` evaluates the callback body once per visited element
/// (or not at all for an empty array), so only side-effect-free expressions,
/// whose evaluation count is unobservable, are safe to rewrite.
fn is_repeatable_search_value(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(_)
        | AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsThisExpression(_) => true,
        AnyJsExpression::JsUnaryExpression(unary) => {
            // `delete` mutates its operand, so it is not side-effect-free.
            let is_delete = unary
                .operator_token()
                .is_ok_and(|token| token.kind() == biome_js_syntax::JsSyntaxKind::DELETE_KW);
            !is_delete
                && unary
                    .argument()
                    .is_ok_and(|argument| is_repeatable_search_value(&argument))
        }
        AnyJsExpression::JsStaticMemberExpression(member) => member
            .object()
            .is_ok_and(|object| is_repeatable_search_value(&object)),
        AnyJsExpression::JsComputedMemberExpression(member) => {
            member
                .object()
                .is_ok_and(|object| is_repeatable_search_value(&object))
                && member
                    .member()
                    .is_ok_and(|inner| is_repeatable_search_value(&inner))
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            // A tag is a call; interpolations may hold arbitrary expressions.
            template.tag().is_none()
                && template.elements().iter().all(|element| {
                    element
                        .as_js_template_element()
                        .and_then(|element| element.expression().ok())
                        .is_none_or(|expr| is_repeatable_search_value(&expr))
                })
        }
        _ => false,
    }
}

/// Whether `expr` may evaluate to `NaN` or `undefined`, values for which
/// `some()` with strict equality and `includes()` with SameValueZero (and its
/// hole handling) are not equivalent.
fn is_nan_or_undefined(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();
    match &expr {
        // `undefined`, `NaN`
        AnyJsExpression::JsIdentifierExpression(ident) => ident
            .name()
            .ok()
            .and_then(|reference| reference.value_token().ok())
            .is_some_and(|token| matches!(token.text_trimmed(), "undefined" | "NaN")),
        // `Number.NaN`
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let is_nan_member = member
                .member()
                .ok()
                .and_then(|name| name.as_js_name()?.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "NaN");
            let is_number_object = member
                .object()
                .ok()
                .and_then(|object| {
                    object
                        .as_js_identifier_expression()?
                        .name()
                        .ok()?
                        .value_token()
                        .ok()
                })
                .is_some_and(|token| token.text_trimmed() == "Number");
            is_nan_member && is_number_object
        }
        // `void <expr>` always evaluates to `undefined`.
        AnyJsExpression::JsUnaryExpression(unary) => unary
            .operator_token()
            .is_ok_and(|token| token.kind() == biome_js_syntax::JsSyntaxKind::VOID_KW),
        _ => false,
    }
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
        .is_ok_and(|t| t.kind() == biome_js_syntax::JsSyntaxKind::MINUS);
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
    let member = callee
        .as_ref()
        .and_then(|c| c.as_js_static_member_expression());
    let object = member.and_then(|m| m.object().ok());

    let Some(object) = object else {
        return false;
    };

    ctx.type_of_expression(&object)
        .is_some_and(|ty| ty.is_all_string_array_or_tuple())
}
