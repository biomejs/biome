use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyTsType,
    JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsConditionalExpression, JsExportDefaultExpressionClause, JsExpressionStatement,
    JsGetterClassMember, JsGetterObjectMember, JsInitializerClause, JsLogicalExpression,
    JsNewExpression, JsObjectExpression, JsParenthesizedExpression, JsPropertyClassMember,
    JsPropertyObjectMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsSyntaxNode, JsVariableDeclarator, TsTypeAnnotation,
};
use biome_js_type_info::{AnyTypeKind, InferredType};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
use biome_rule_options::no_unsafe_return::NoUnsafeReturnOptions;

declare_lint_rule! {
    /// Disallow returning a value typed as `any` from a function.
    ///
    /// TypeScript doesn't check how values typed as `any` are used. When a
    /// function returns such a value, every caller receives it without type
    /// checking, which can hide bugs far away from where the `any` came from.
    ///
    /// This rule reports functions that return a value typed as `any` or
    /// `any[]`, and `async` functions that return a `Promise<any>`. It also
    /// reports returned values that use `any` in a type argument where the
    /// return type expects a specific type, such as returning a `Set<any>`
    /// from a function whose return type is `Set<string>`.
    ///
    /// Returning `any` is allowed when the return type is declared as `any`
    /// or `unknown`, and returning `any[]` is allowed when the return type is
    /// `unknown[]`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-any.ts
    /// function parse(text: string) {
    ///     return JSON.parse(text);
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-any-array.ts
    /// const getItems = () => [] as any[];
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-promise-any.ts
    /// declare const pending: Promise<any>;
    /// async function load() {
    ///     return pending;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-type-argument.ts
    /// function getNames(): Set<string> {
    ///     return new Set<any>([1]);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// function parse(text: string): unknown {
    ///     return JSON.parse(text);
    /// }
    ///
    /// function getNames(): Set<string> {
    ///     return new Set<string>(["a"]);
    /// }
    ///
    /// const getItems = () => ["a"];
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow writing the `any` type, see [`noExplicitAny`](https://biomejs.dev/linter/rules/no-explicit-any/).
    pub NoUnsafeReturn {
        version: "2.6.3",
        name: "noUnsafeReturn",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-unsafe-return").same()],
        recommended: false,
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    /// A `return` statement, or an arrow function whose body is an expression.
    pub AnyJsReturnSite = JsReturnStatement | JsArrowFunctionExpression
}

/// How a returned value leaks `any`.
pub enum UnsafeReturn {
    /// The returned value is `any`, `any[]`, or `Promise<any>`.
    Value(AnyTypeKind),
    /// The returned value uses `any` in a type argument that the return type
    /// expects to be specific.
    TypeArgument,
}

impl Rule for NoUnsafeReturn {
    type Query = Typed<AnyJsReturnSite>;
    type State = UnsafeReturn;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let (expression, function) = match ctx.query() {
            AnyJsReturnSite::JsReturnStatement(statement) => (
                statement.argument()?,
                statement
                    .syntax()
                    .ancestors()
                    .skip(1)
                    .find_map(AnyReturningFunction::cast)?,
            ),
            AnyJsReturnSite::JsArrowFunctionExpression(arrow) => {
                let AnyJsFunctionBody::AnyJsExpression(body) = arrow.body().ok()? else {
                    return None;
                };
                (
                    body,
                    AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::AnyJsFunction(
                        AnyJsFunction::JsArrowFunctionExpression(arrow.clone()),
                    )),
                )
            }
        };

        let returned = ctx.type_of_expression(&expression)?;
        let kind = returned.any_type_kind();
        let (is_async, is_generator) = match &function {
            AnyReturningFunction::AnyFunctionLike(function) => {
                (function.is_async(), function.is_generator())
            }
            _ => (false, false),
        };
        // The value returned from an `async` function is what its Promise
        // resolves to.
        let returned = if is_async {
            returned.awaited_type()
        } else {
            returned
        };
        if kind.is_none() && !returned.has_type_arguments() {
            return None;
        }

        // A generator returns its value through the `Generator` object, so its
        // declared return type doesn't describe the returned value.
        let expected = if is_generator {
            ExpectedReturn::Unconstrained
        } else {
            expected_return_type(ctx, &function)
        };
        let (is_annotated, expected) = match expected {
            ExpectedReturn::Annotated(ty) => (true, Some(ty)),
            ExpectedReturn::Contextual(ty) => (false, Some(ty)),
            ExpectedReturn::Unconstrained => (false, None),
            ExpectedReturn::Unresolved => return None,
        };
        let expected = if is_async {
            expected.map(InferredType::awaited_type)
        } else {
            expected
        };
        if is_annotated && expected.is_some_and(InferredType::is_any_or_unknown_keyword) {
            return None;
        }

        let Some(kind) = kind else {
            let expected = expected?;
            if is_empty_map_construction(&expression)
                || !returned.has_unsafe_any_type_argument_for(expected)
            {
                return None;
            }
            return Some(UnsafeReturn::TypeArgument);
        };

        if kind == AnyTypeKind::PromiseAny && !is_async {
            return None;
        }
        if let Some(expected) = expected {
            let is_allowed = match kind {
                AnyTypeKind::Any | AnyTypeKind::PromiseAny => expected.is_unknown_keyword(),
                AnyTypeKind::AnyArray => expected.is_unknown_array(),
            };
            // An annotation that names the same unsafe type is intentional.
            if is_allowed || (is_annotated && expected.any_type_kind() == Some(kind)) {
                return None;
            }
        }
        Some(UnsafeReturn::Value(kind))
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            AnyJsReturnSite::JsReturnStatement(statement) => statement.range(),
            AnyJsReturnSite::JsArrowFunctionExpression(arrow) => arrow.body().ok()?.range(),
        };
        let note = markup! {
            "TypeScript doesn't check values of type "<Emphasis>"any"</Emphasis>", so callers can misuse the returned value without any error."
        };
        Some(match state {
            UnsafeReturn::Value(kind) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This function returns a value of type "<Emphasis>{AnyTypeName(*kind)}</Emphasis>"."
                },
            )
            .note(note)
            .note(markup! {
                "Give the returned value a specific type. If the type isn't known in advance, use "<Emphasis>"unknown"</Emphasis>" instead, which requires callers to check the value before using it."
            }),
            UnsafeReturn::TypeArgument => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This function returns a value that uses "<Emphasis>"any"</Emphasis>" where its return type expects a specific type."
                },
            )
            .note(note)
            .note(markup! {
                "Replace "<Emphasis>"any"</Emphasis>" in the type arguments of the returned value with the types that the return type expects."
            }),
        })
    }
}

declare_node_union! {
    /// A function-like node that owns the `return` statements in its body.
    pub AnyReturningFunction =
        AnyFunctionLike
        | JsGetterClassMember
        | JsGetterObjectMember
        | JsSetterClassMember
        | JsSetterObjectMember
}

struct AnyTypeName(AnyTypeKind);

impl biome_console::fmt::Display for AnyTypeName {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            AnyTypeKind::Any => "any",
            AnyTypeKind::AnyArray => "any[]",
            AnyTypeKind::PromiseAny => "Promise<any>",
        })
    }
}

/// The type that the values returned from a function are expected to have.
enum ExpectedReturn<'a> {
    /// The type written in the function's return type annotation.
    Annotated(InferredType<'a>),
    /// The return type of the function type expected where a function or
    /// arrow function expression is written, such as the type of an annotated
    /// variable it's assigned to.
    Contextual(InferredType<'a>),
    /// Nothing constrains the returned values.
    Unconstrained,
    /// The returned values are constrained by a type that couldn't be
    /// resolved.
    Unresolved,
}

fn expected_return_type<'a>(
    ctx: &'a RuleContext<NoUnsafeReturn>,
    function: &AnyReturningFunction,
) -> ExpectedReturn<'a> {
    let annotated = |ty: Option<InferredType<'a>>| match ty {
        Some(ty) if ty.is_inferred() => ExpectedReturn::Annotated(ty),
        _ => ExpectedReturn::Unresolved,
    };
    match function {
        AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::AnyJsFunction(function)) => {
            if function.return_type_annotation().is_some() {
                annotated(ctx.return_type_of_function(function))
            } else if let Some(expression) = AnyJsExpression::cast_ref(function.syntax()) {
                contextual_return_type(ctx, &expression)
            } else {
                ExpectedReturn::Unconstrained
            }
        }
        AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::JsMethodClassMember(method)) => {
            if method.return_type_annotation().is_some() {
                annotated(method.name().ok().and_then(|name| {
                    ctx.return_type_of_member(method.syntax(), name.name()?.text())
                }))
            } else {
                ExpectedReturn::Unconstrained
            }
        }
        AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::JsMethodObjectMember(method)) => {
            if method.return_type_annotation().is_some() {
                annotated(method.name().ok().and_then(|name| {
                    ctx.return_type_of_member(method.syntax(), name.name()?.text())
                }))
            } else {
                object_member_expected_return(ctx, method.syntax())
            }
        }
        AnyReturningFunction::JsGetterClassMember(getter) => {
            match getter.return_type() {
                Some(annotation) if is_function_type_annotation(&annotation) => {
                    ExpectedReturn::Unresolved
                }
                Some(_) => annotated(getter.name().ok().and_then(|name| {
                    ctx.return_type_of_member(getter.syntax(), name.name()?.text())
                })),
                None => ExpectedReturn::Unconstrained,
            }
        }
        AnyReturningFunction::JsGetterObjectMember(getter) => {
            match getter.return_type() {
                Some(annotation) if is_function_type_annotation(&annotation) => {
                    ExpectedReturn::Unresolved
                }
                Some(_) => annotated(getter.name().ok().and_then(|name| {
                    ctx.return_type_of_member(getter.syntax(), name.name()?.text())
                })),
                None => object_member_expected_return(ctx, getter.syntax()),
            }
        }
        AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::JsConstructorClassMember(_))
        | AnyReturningFunction::JsSetterClassMember(_)
        | AnyReturningFunction::JsSetterObjectMember(_) => ExpectedReturn::Unconstrained,
    }
}

/// Returns whether a getter's annotation is a function type.
///
/// The member lookup resolves a member whose type is a function to that
/// function's return type, so it can't resolve such a getter's type.
fn is_function_type_annotation(annotation: &TsTypeAnnotation) -> bool {
    annotation
        .ty()
        .is_ok_and(|ty| matches!(ty, AnyTsType::TsFunctionType(_)))
}

/// Returns the return type of the function type expected where the function
/// or arrow function `expression` is written.
fn contextual_return_type<'a>(
    ctx: &'a RuleContext<NoUnsafeReturn>,
    expression: &AnyJsExpression,
) -> ExpectedReturn<'a> {
    let contextual = match expected_type_at(ctx, expression) {
        ExpectedReturn::Contextual(contextual) => contextual,
        expected => return expected,
    };
    if let Some(return_type) = contextual.non_nullish_callable_return_type() {
        ExpectedReturn::Contextual(return_type)
    } else if contextual.is_any_or_unknown_keyword() || contextual.is_callable() == Some(false) {
        ExpectedReturn::Unconstrained
    } else {
        ExpectedReturn::Unresolved
    }
}

/// Returns the type expected where `expression` is written, as
/// [`ExpectedReturn::Contextual`].
///
/// Positions whose expected type isn't looked up, such as properties of a
/// typed object literal or operands of `as` and `satisfies`, are
/// [`ExpectedReturn::Unresolved`].
fn expected_type_at<'a>(
    ctx: &'a RuleContext<NoUnsafeReturn>,
    expression: &AnyJsExpression,
) -> ExpectedReturn<'a> {
    // Type assertions such as `as` and `satisfies` provide an expected type of
    // their own, so only parentheses are skipped.
    let mut node = expression.syntax().clone();
    while let Some(parent) = node
        .parent()
        .filter(|parent| JsParenthesizedExpression::can_cast(parent.kind()))
    {
        node = parent;
    }
    let Some(parent) = node.parent() else {
        return ExpectedReturn::Unconstrained;
    };
    let resolved = |ty: Option<InferredType<'a>>| match ty {
        Some(ty) if ty.is_inferred() => ExpectedReturn::Contextual(ty),
        _ => ExpectedReturn::Unresolved,
    };

    if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
        if let Some(declarator) = initializer.parent::<JsVariableDeclarator>() {
            if declarator.variable_annotation().is_none() {
                return ExpectedReturn::Unconstrained;
            }
            return resolved(declarator.id().ok().and_then(|id| {
                let name = id
                    .as_any_js_binding()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?;
                ctx.type_of_named_value(declarator.range(), name.text_trimmed())
            }));
        }
        if initializer
            .parent::<JsPropertyClassMember>()
            .is_some_and(|property| property.property_annotation().is_none())
        {
            return ExpectedReturn::Unconstrained;
        }
        return ExpectedReturn::Unresolved;
    }

    if let Some(arguments) = JsCallArgumentList::cast_ref(&parent) {
        // Arguments of callees whose parameters can't be resolved, such as
        // callees typed as `any`, are treated as unconstrained.
        return arguments
            .iter()
            .position(|argument| argument.is_ok_and(|argument| argument.syntax() == &node))
            .and_then(|index| {
                let call = arguments.parent::<JsCallArguments>()?.syntax().parent()?;
                if let Some(call) = JsCallExpression::cast_ref(&call) {
                    ctx.expected_argument_type(&call.callee().ok()?, &arguments, index, false)
                } else {
                    let call = JsNewExpression::cast(call)?;
                    ctx.expected_argument_type(&call.callee().ok()?, &arguments, index, true)
                }
            })
            .filter(|ty| ty.is_inferred())
            .map_or(ExpectedReturn::Unconstrained, ExpectedReturn::Contextual);
    }

    // A returned expression is expected to have the return type of the
    // function that returns it.
    let returning_function = if JsReturnStatement::can_cast(parent.kind()) {
        parent
            .ancestors()
            .skip(1)
            .find_map(AnyReturningFunction::cast)
    } else {
        JsArrowFunctionExpression::cast_ref(&parent).map(|arrow| {
            AnyReturningFunction::AnyFunctionLike(AnyFunctionLike::AnyJsFunction(
                AnyJsFunction::JsArrowFunctionExpression(arrow),
            ))
        })
    };
    if let Some(function) = returning_function {
        let (is_async, is_generator) = match &function {
            AnyReturningFunction::AnyFunctionLike(function) => {
                (function.is_async(), function.is_generator())
            }
            _ => (false, false),
        };
        if is_generator {
            return ExpectedReturn::Unresolved;
        }
        return match expected_return_type(ctx, &function) {
            ExpectedReturn::Annotated(ty) | ExpectedReturn::Contextual(ty) => {
                ExpectedReturn::Contextual(if is_async { ty.awaited_type() } else { ty })
            }
            expected => expected,
        };
    }

    if JsPropertyObjectMember::can_cast(parent.kind()) {
        return object_member_expected_return(ctx, &parent);
    }

    // Both branches of a conditional expression and both operands of a
    // logical expression share the expected type of the whole expression.
    if let Some(conditional) = JsConditionalExpression::cast_ref(&parent)
        && conditional.test().is_ok_and(|test| test.syntax() != &node)
    {
        return expected_type_at(ctx, &AnyJsExpression::JsConditionalExpression(conditional));
    }
    if let Some(logical) = JsLogicalExpression::cast_ref(&parent) {
        return expected_type_at(ctx, &AnyJsExpression::JsLogicalExpression(logical));
    }

    let is_unconstrained = JsExpressionStatement::can_cast(parent.kind())
        || JsExportDefaultExpressionClause::can_cast(parent.kind())
        || JsCallExpression::cast_ref(&parent)
            .and_then(|call| call.callee().ok())
            .is_some_and(|callee| callee.syntax() == &node);
    if is_unconstrained {
        ExpectedReturn::Unconstrained
    } else {
        ExpectedReturn::Unresolved
    }
}

/// Returns what's expected from a member of an object literal: nothing when
/// the object literal itself is unconstrained, and otherwise an unresolved
/// type, because the types of individual members aren't looked up.
fn object_member_expected_return<'a>(
    ctx: &'a RuleContext<NoUnsafeReturn>,
    member: &JsSyntaxNode,
) -> ExpectedReturn<'a> {
    let Some(object) = member
        .ancestors()
        .skip(1)
        .find_map(JsObjectExpression::cast)
    else {
        return ExpectedReturn::Unresolved;
    };
    match expected_type_at(ctx, &AnyJsExpression::JsObjectExpression(object)) {
        ExpectedReturn::Unconstrained => ExpectedReturn::Unconstrained,
        _ => ExpectedReturn::Unresolved,
    }
}

/// Returns whether `expression` is `new Map()` without arguments or type
/// arguments, which TypeScript types as `Map<any, any>`.
fn is_empty_map_construction(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression
        .clone()
        .omit_parentheses()
        .as_js_new_expression()
        .cloned()
    else {
        return false;
    };
    expression.type_arguments().is_none()
        && expression
            .arguments()
            .is_none_or(|arguments| arguments.args().is_empty())
        && expression.callee().is_ok_and(|callee| {
            callee
                .as_js_identifier_expression()
                .and_then(|identifier| identifier.name().ok())
                .is_some_and(|name| name.has_name("Map"))
        })
}
