use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsDeclarationClause, AnyJsExportClause, AnyJsExportDefaultDeclaration,
    AnyJsExpression, AnyJsName, AnyTsName, AnyTsReturnType, AnyTsType, JsExport,
    JsExpressionStatement, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration, JsSyntaxKind,
    JsSyntaxNode, T, TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration,
    TsDeclareStatement,
    binding_ext::AnyJsBindingDeclaration,
    parameter_ext::{AnyJsParameterList, AnyParameter},
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind, declare_node_union,
};
use biome_rule_options::no_floating_promises::NoFloatingPromisesOptions;

use crate::{JsRuleAction, ast_utils::is_in_async_function, services::typed::Typed};

declare_lint_rule! {
    /// Require Promise-like statements to be handled appropriately.
    ///
    /// A "floating" `Promise` is one that is created without any code set up to
    /// handle any errors it might throw. Floating Promises can lead to several
    /// issues, including improperly sequenced operations, unhandled Promise
    /// rejections, and other unintended consequences.
    ///
    /// This rule will report Promise-valued statements that are not treated in
    /// one of the following ways:
    /// - Calling its `.then()` method with two arguments
    /// - Calling its `.catch()` method with one argument
    /// - `await`-ing it
    /// - `return`-ing it
    /// - `void`-ing it
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=async-fn.ts
    /// async function returnsPromise(): Promise<string> {
    ///   return 'value';
    /// }
    /// returnsPromise().then(() => {});
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-fn2.ts
    /// const returnsPromise = async (): Promise<string> => {
    ///   return 'value';
    /// }
    /// async function returnsPromiseInAsyncFunction() {
    ///   returnsPromise().then(() => {});
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=new-promise.js
    /// const promise = new Promise((resolve) => resolve('value'));
    /// promise.then(() => { }).finally(() => { });
    /// ```
    ///
    /// ```js,expect_diagnostic,file=promise-all.js
    /// Promise.all([p1, p2, p3])
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-method.ts
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    ///   async someMethod() {
    ///     this.returnsPromise();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-super-method.ts
    /// class Parent {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    /// }
    ///
    /// class Child extends Parent {
    ///   async someMethod() {
    ///     this.returnsPromise();
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-method2.ts
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    /// }
    /// const api = new Api();
    /// api.returnsPromise().then(() => {}).finally(() => {});
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-object-method.ts
    /// const obj = {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   },
    /// };
    ///
    /// obj.returnsPromise();
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=async-prop.ts
    /// type Props = {
    ///   returnsPromise: () => Promise<void>;
    /// };
    ///
    /// async function testCallingReturnsPromise(props: Props) {
    ///   props.returnsPromise();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-examples.ts
    /// async function returnsPromise(): Promise<string> {
    ///   return 'value';
    /// }
    ///
    /// await returnsPromise();
    ///
    /// void returnsPromise();
    ///
    /// // Calling .then() with two arguments
    /// returnsPromise().then(
    ///   () => {},
    ///   () => {},
    /// );
    ///
    /// // Calling .catch() with one argument
    /// returnsPromise().catch(() => {});
    ///
    /// await Promise.all([p1, p2, p3])
    ///
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    ///   async someMethod() {
    ///     await this.returnsPromise();
    ///   }
    /// }
    ///
    /// type Props = {
    ///   returnsPromise: () => Promise<void>;
    /// };
    ///
    /// async function testCallingReturnsPromise(props: Props) {
    ///   return props.returnsPromise();
    /// }
    /// ```
    ///
    pub NoFloatingPromises {
        version: "2.0.0",
        name: "noFloatingPromises",
        language: "ts",
        recommended: true,
        sources: &[RuleSource::EslintTypeScript("no-floating-promises").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

pub enum NoFloatingPromisesState {
    ArrayOfPromises,
    UnhandledPromise,
}

impl Rule for NoFloatingPromises {
    type Query = Typed<JsExpressionStatement>;
    type State = NoFloatingPromisesState;
    type Signals = Option<Self::State>;
    type Options = NoFloatingPromisesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let expression = node.expression().ok()?;
        let ty = ctx.type_of_expression(&expression);

        if ty.is_array_of(|ty| ty.is_promise_instance()) {
            return Some(NoFloatingPromisesState::ArrayOfPromises);
        }

        let is_maybe_promise =
            ty.is_promise_instance() || ty.has_variant(|ty| ty.is_promise_instance());
        if !is_maybe_promise {
            return None;
        }

        if expression
            .as_js_call_expression()
            .and_then(|call| matching_overload_returns_promise_like(ctx, &call))
            .is_some_and(|returns_promise| !returns_promise)
        {
            return None;
        }

        if is_handled_promise(expression).unwrap_or_default() {
            return None;
        }

        Some(NoFloatingPromisesState::UnhandledPromise)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            NoFloatingPromisesState::ArrayOfPromises => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "An array of Promises was found, meaning they are not "
                        "properly handled and could lead to ignored errors or "
                        "unexpected behavior."
                    },
                )
                .note(markup! {
                    "This happens when an array of Promises is not wrapped "
                    "with Promise.all() or a similar method, and is not "
                    "explicitly ignored using the `void` operator."
                }),
            ),
            NoFloatingPromisesState::UnhandledPromise => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "A \"floating\" Promise was found, meaning it is not "
                        "properly handled and could lead to ignored errors or "
                        "unexpected behavior."
                    },
                )
                .note(markup! {
                    "This happens when a Promise is not awaited, lacks a "
                    "`.catch` or `.then` rejection handler, or is not "
                    "explicitly ignored using the `void` operator."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if !is_in_async_function(node.syntax()) {
            return None;
        }

        let expression = node.expression().ok()?;
        let mut mutation = ctx.root().begin();
        match state {
            NoFloatingPromisesState::ArrayOfPromises => {
                let callee_expression =
                    AnyJsExpression::JsStaticMemberExpression(make::js_static_member_expression(
                        AnyJsExpression::JsIdentifierExpression(make::js_identifier_expression(
                            make::js_reference_identifier(make::ident("Promise")),
                        )),
                        make::token(T![.]),
                        AnyJsName::JsName(make::js_name(make::ident("all"))),
                    ));

                let call_expression = AnyJsExpression::JsCallExpression(
                    make::js_call_expression(
                        callee_expression,
                        make::js_call_arguments(
                            make::token(T!['(']),
                            make::js_call_argument_list(
                                [AnyJsCallArgument::AnyJsExpression(
                                    expression.clone().trim_trivia()?,
                                )],
                                [],
                            ),
                            make::token(T![')']),
                        ),
                    )
                    .build(),
                );

                let await_expression =
                    AnyJsExpression::JsAwaitExpression(make::js_await_expression(
                        make::token(JsSyntaxKind::AWAIT_KW)
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        call_expression,
                    ));

                mutation.replace_node(expression, await_expression);
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Wrap in Promise.all() and add await operator." }.to_owned(),
                    mutation,
                ))
            }
            NoFloatingPromisesState::UnhandledPromise => {
                let await_expression =
                    AnyJsExpression::JsAwaitExpression(make::js_await_expression(
                        make::token(JsSyntaxKind::AWAIT_KW)
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        expression.clone().trim_comments_and_trivia()?,
                    ));

                mutation.replace_node_transfer_trivia(expression, await_expression);
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Add await operator." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

/// Checks if a JS `expression` is a handled Promise-like expression.
///
/// A `Promise` is considered handled if:
/// - It calls its `.then()` method with two arguments, _or_
/// - It calls its `.catch()` method with one argument.
///
/// # Returns
///
/// * `Some(true)` if the expression is a handled Promise-like expression.
/// * `Some(false)` if the expression is not a handled Promise-like expression.
/// * `None` if there is an error in the process.
///
/// # Examples
///
/// Example TypeScript code that would return `Some(true)`:
/// ```typescript
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa", () => null).finally(() => null)
///
/// const promise: Promise<unknown> = new Promise((resolve, reject) => resolve('value'));
/// promise.then(() => "aaa").catch(() => null).finally(() => null)
/// ```
fn is_handled_promise(expression: AnyJsExpression) -> Option<bool> {
    let js_call_expression = match expression.omit_parentheses() {
        AnyJsExpression::JsCallExpression(js_call_expression) => js_call_expression,
        AnyJsExpression::JsAssignmentExpression(_) => {
            // We consider assignments to be handled, otherwise any attempt to
            // assign a promise will be flagged by this rule.
            return Some(true);
        }
        _ => return None,
    };

    let expr = js_call_expression.callee().ok()?;
    let static_member_expr = expr.as_js_static_member_expression()?;
    let member = static_member_expr.member().ok()?;
    let js_name = member.as_js_name()?;
    let value_token = js_name.value_token().ok()?;
    let name = value_token.text_trimmed();

    if name == "finally" {
        let expr = static_member_expr.object().ok()?;
        return is_handled_promise(expr);
    }
    if name == "catch" {
        let call_args = js_call_expression.arguments().ok()?;
        // just checking if there are any arguments, not if it's a function for simplicity
        return Some(call_args.args().len() > 0);
    }
    if name == "then" {
        let call_args = js_call_expression.arguments().ok()?;
        // just checking arguments have a reject function from length
        return Some(call_args.args().len() >= 2);
    }

    Some(false)
}

fn matching_overload_returns_promise_like(
    ctx: &RuleContext<NoFloatingPromises>,
    call: &biome_js_syntax::JsCallExpression,
) -> Option<bool> {
    let model = ctx.get_service::<SemanticModel>()?;
    let reference = call
        .callee()
        .ok()?
        .omit_parentheses()
        .as_js_identifier_expression()?
        .name()
        .ok()?;
    let declaration = model.binding(&reference)?.tree().declaration()?;
    let overloads = collect_adjacent_function_overloads(&declaration)?;

    overloads
        .into_iter()
        .filter(|overload| !overload.has_body())
        .find(|overload| function_overload_matches_call(ctx, call, overload))
        .and_then(|overload| overload.return_type_annotation())
        .and_then(|return_type| return_type.as_any_ts_type().cloned())
        .map(ts_type_is_promise_like)
}

fn collect_adjacent_function_overloads(
    declaration: &AnyJsBindingDeclaration,
) -> Option<Vec<AnyPotentialFunctionOverloadSignature>> {
    let declaration = AnyPotentialFunctionOverloadSignature::from_binding_declaration(declaration)?;
    let name = declaration.name()?;

    let mut first = declaration.clone();
    while let Some(previous) = first
        .prev_sibling()
        .and_then(AnyPotentialFunctionOverloadSignature::cast)
    {
        if previous.name().as_deref() != Some(name.as_str()) {
            break;
        }
        first = previous;
    }

    let mut overloads = vec![first.clone()];
    let mut next_sibling = first.next_sibling();
    while let Some(next) = next_sibling {
        let Some(overload) = AnyPotentialFunctionOverloadSignature::cast(next.clone()) else {
            break;
        };
        if overload.name().as_deref() != Some(name.as_str()) {
            break;
        }
        next_sibling = overload.next_sibling();
        overloads.push(overload);
    }

    (overloads.len() > 1 && overloads.iter().any(|overload| !overload.has_body()))
        .then_some(overloads)
}

fn function_overload_matches_call(
    ctx: &RuleContext<NoFloatingPromises>,
    call: &biome_js_syntax::JsCallExpression,
    overload: &AnyPotentialFunctionOverloadSignature,
) -> bool {
    let Ok(arguments) = call.arguments() else {
        return false;
    };
    let Some(parameters) = overload.parameters() else {
        return false;
    };

    let parameters: Vec<_> = parameters
        .iter()
        .filter_map(|parameter| parameter.ok())
        .filter(|parameter| {
            !matches!(
                parameter,
                AnyParameter::AnyJsParameter(biome_js_syntax::AnyJsParameter::TsThisParameter(_))
            )
        })
        .collect();

    let required_parameter_count = parameters
        .iter()
        .filter(|parameter| !parameter_is_optional(parameter) && !parameter_is_rest(parameter))
        .count();
    if arguments.args().len() < required_parameter_count {
        return false;
    }

    let rest_parameter = parameters
        .last()
        .filter(|parameter| parameter_is_rest(parameter));
    if rest_parameter.is_none() && arguments.args().len() > parameters.len() {
        return false;
    }

    arguments
        .args()
        .iter()
        .enumerate()
        .all(|(index, argument)| {
            let Ok(argument) = argument else {
                return false;
            };
            parameters
                .get(index)
                .or(rest_parameter)
                .is_some_and(|parameter| argument_matches_parameter(ctx, &argument, parameter))
        })
}

fn parameter_is_optional(parameter: &AnyParameter) -> bool {
    match parameter {
        AnyParameter::AnyJsConstructorParameter(parameter) => match parameter {
            biome_js_syntax::AnyJsConstructorParameter::AnyJsFormalParameter(parameter) => {
                parameter.as_js_formal_parameter().is_some_and(|parameter| {
                    parameter.question_mark_token().is_some() || parameter.initializer().is_some()
                })
            }
            biome_js_syntax::AnyJsConstructorParameter::JsRestParameter(_) => false,
            biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(parameter) => parameter
                .formal_parameter()
                .ok()
                .and_then(|parameter| parameter.as_js_formal_parameter().cloned())
                .is_some_and(|parameter| {
                    parameter.question_mark_token().is_some() || parameter.initializer().is_some()
                }),
        },
        AnyParameter::AnyJsParameter(parameter) => match parameter {
            biome_js_syntax::AnyJsParameter::AnyJsFormalParameter(parameter) => {
                parameter.as_js_formal_parameter().is_some_and(|parameter| {
                    parameter.question_mark_token().is_some() || parameter.initializer().is_some()
                })
            }
            biome_js_syntax::AnyJsParameter::JsRestParameter(_) => false,
            biome_js_syntax::AnyJsParameter::TsThisParameter(_) => false,
        },
    }
}

fn parameter_is_rest(parameter: &AnyParameter) -> bool {
    matches!(
        parameter,
        AnyParameter::AnyJsConstructorParameter(
            biome_js_syntax::AnyJsConstructorParameter::JsRestParameter(_)
        ) | AnyParameter::AnyJsParameter(biome_js_syntax::AnyJsParameter::JsRestParameter(_))
    )
}

fn argument_matches_parameter(
    ctx: &RuleContext<NoFloatingPromises>,
    argument: &AnyJsCallArgument,
    parameter: &AnyParameter,
) -> bool {
    let Some(expected_callback_returns_promise) = parameter_callback_returns_promise(parameter)
    else {
        return true;
    };
    let AnyJsCallArgument::AnyJsExpression(argument) = argument else {
        return false;
    };
    let Some(actual_callback_returns_promise) =
        expression_function_returns_promise_like(ctx, argument)
    else {
        return false;
    };

    expected_callback_returns_promise == actual_callback_returns_promise
}

fn expression_function_returns_promise_like(
    ctx: &RuleContext<NoFloatingPromises>,
    expression: &AnyJsExpression,
) -> Option<bool> {
    let expression_type = ctx.type_of_expression(expression);
    let function = expression_type.as_function()?;
    let return_type = function.return_type.as_type()?;
    let return_type = expression_type.resolve(return_type)?;

    Some(
        return_type.is_promise_instance() || return_type.has_variant(|ty| ty.is_promise_instance()),
    )
}

fn parameter_callback_returns_promise(parameter: &AnyParameter) -> Option<bool> {
    let annotation = parameter.type_annotation()?.ty().ok()?;
    callback_type_returns_promise(annotation)
}

fn callback_type_returns_promise(ty: AnyTsType) -> Option<bool> {
    let ty = ty.omit_parentheses();
    let function = ty.as_ts_function_type()?;
    let return_type = function.return_type().ok()?;
    let return_type = return_type.as_any_ts_type()?.clone();
    Some(ts_type_is_promise_like(return_type))
}

fn ts_type_is_promise_like(ty: AnyTsType) -> bool {
    match ty.omit_parentheses() {
        AnyTsType::TsReferenceType(reference) => reference
            .name()
            .ok()
            .is_some_and(|name| ts_name_is_promise(&name)),
        AnyTsType::TsUnionType(union) => union
            .types()
            .into_iter()
            .filter_map(|ty| ty.ok())
            .any(ts_type_is_promise_like),
        _ => false,
    }
}

fn ts_name_is_promise(name: &AnyTsName) -> bool {
    name.as_js_reference_identifier().is_some_and(|name| {
        name.value_token()
            .ok()
            .is_some_and(|token| token.text_trimmed() == "Promise")
    })
}

declare_node_union! {
    AnyPotentialFunctionOverloadSignature =
        JsFunctionDeclaration
        | JsFunctionExportDefaultDeclaration
        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration
}

impl AnyPotentialFunctionOverloadSignature {
    fn from_binding_declaration(declaration: &AnyJsBindingDeclaration) -> Option<Self> {
        match declaration {
            AnyJsBindingDeclaration::JsFunctionDeclaration(declaration) => {
                Some(Self::JsFunctionDeclaration(declaration.clone()))
            }
            AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(declaration) => Some(
                Self::JsFunctionExportDefaultDeclaration(declaration.clone()),
            ),
            AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(declaration) => {
                Some(Self::TsDeclareFunctionDeclaration(declaration.clone()))
            }
            AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                Some(Self::TsDeclareFunctionExportDefaultDeclaration(
                    declaration.clone(),
                ))
            }
            _ => None,
        }
    }

    fn name(&self) -> Option<String> {
        match self {
            Self::JsFunctionDeclaration(declaration) => {
                function_binding_name(declaration.id().ok()?)
            }
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                function_binding_name(declaration.id()?)
            }
            Self::TsDeclareFunctionDeclaration(declaration) => {
                function_binding_name(declaration.id().ok()?)
            }
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                function_binding_name(declaration.id()?)
            }
        }
    }

    fn parameters(&self) -> Option<AnyJsParameterList> {
        Some(match self {
            Self::JsFunctionDeclaration(function) => function.parameters().ok()?.items().into(),
            Self::JsFunctionExportDefaultDeclaration(function) => {
                function.parameters().ok()?.items().into()
            }
            Self::TsDeclareFunctionDeclaration(function) => {
                function.parameters().ok()?.items().into()
            }
            Self::TsDeclareFunctionExportDefaultDeclaration(function) => {
                function.parameters().ok()?.items().into()
            }
        })
    }

    fn return_type_annotation(&self) -> Option<AnyTsReturnType> {
        match self {
            Self::JsFunctionDeclaration(function) => function
                .return_type_annotation()
                .and_then(|annotation| annotation.ty().ok()),
            Self::JsFunctionExportDefaultDeclaration(function) => function
                .return_type_annotation()
                .and_then(|annotation| annotation.ty().ok()),
            Self::TsDeclareFunctionDeclaration(function) => function
                .return_type_annotation()
                .and_then(|annotation| annotation.ty().ok()),
            Self::TsDeclareFunctionExportDefaultDeclaration(function) => function
                .return_type_annotation()
                .and_then(|annotation| annotation.ty().ok()),
        }
    }

    fn has_body(&self) -> bool {
        match self {
            Self::JsFunctionDeclaration(function) => function.body().ok().is_some(),
            Self::JsFunctionExportDefaultDeclaration(function) => function.body().ok().is_some(),
            Self::TsDeclareFunctionDeclaration(_)
            | Self::TsDeclareFunctionExportDefaultDeclaration(_) => false,
        }
    }

    fn wrapper_syntax(&self) -> JsSyntaxNode {
        match self {
            Self::JsFunctionDeclaration(function) => {
                if let Some(parent) = function.syntax().parent()
                    && matches!(parent.kind(), JsSyntaxKind::JS_EXPORT)
                {
                    return parent;
                }
            }
            Self::JsFunctionExportDefaultDeclaration(function) => {
                if let Some(parent) = function.syntax().parent()
                    && matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE
                    )
                    && let Some(export) = parent.parent()
                {
                    return export;
                }
            }
            Self::TsDeclareFunctionDeclaration(function) => {
                if let Some(parent) = function.syntax().parent()
                    && matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_EXPORT | JsSyntaxKind::TS_DECLARE_STATEMENT
                    )
                {
                    return parent;
                }
            }
            Self::TsDeclareFunctionExportDefaultDeclaration(function) => {
                if let Some(parent) = function.syntax().parent()
                    && matches!(
                        parent.kind(),
                        JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE
                    )
                    && let Some(export) = parent.parent()
                {
                    return export;
                }
            }
        }

        self.syntax().clone()
    }

    fn get_sibling_syntax(syntax: JsSyntaxNode) -> Option<JsSyntaxNode> {
        if let Some(export) = JsExport::cast_ref(&syntax) {
            return match export.export_clause().ok()? {
                AnyJsExportClause::AnyJsDeclarationClause(declaration) => match declaration {
                    AnyJsDeclarationClause::JsFunctionDeclaration(function) => {
                        Some(function.syntax().clone())
                    }
                    AnyJsDeclarationClause::TsDeclareFunctionDeclaration(function) => {
                        Some(function.syntax().clone())
                    }
                    _ => None,
                },
                AnyJsExportClause::JsExportDefaultDeclarationClause(default_clause) => {
                    match default_clause.declaration().ok()? {
                        AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(
                            function,
                        ) => Some(function.syntax().clone()),
                        AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(
                            function,
                        ) => Some(function.syntax().clone()),
                        _ => None,
                    }
                }
                _ => None,
            };
        }

        if let Some(declare_statement) = TsDeclareStatement::cast_ref(&syntax) {
            return match declare_statement.declaration().ok()? {
                AnyJsDeclarationClause::TsDeclareFunctionDeclaration(function) => {
                    Some(function.syntax().clone())
                }
                _ => None,
            };
        }

        Some(syntax)
    }

    fn prev_sibling(&self) -> Option<JsSyntaxNode> {
        Self::get_sibling_syntax(self.wrapper_syntax().prev_sibling()?)
    }

    fn next_sibling(&self) -> Option<JsSyntaxNode> {
        Self::get_sibling_syntax(self.wrapper_syntax().next_sibling()?)
    }
}

fn function_binding_name(binding: biome_js_syntax::AnyJsBinding) -> Option<String> {
    binding
        .as_js_identifier_binding()?
        .name_token()
        .ok()
        .map(|token| token.text_trimmed().to_string())
}
