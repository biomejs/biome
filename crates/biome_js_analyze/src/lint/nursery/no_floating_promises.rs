use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsName, JsExpressionStatement, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
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
        domains: &[RuleDomain::Project],
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

        // Uncomment the following line for debugging convenience:
        //let printed = format!("type of {expression:?} = {ty:?}");
        if ty.is_array_of(|ty| ty.is_promise_instance()) {
            return Some(NoFloatingPromisesState::ArrayOfPromises);
        }

        let is_maybe_promise =
            ty.is_promise_instance() || ty.has_variant(|ty| ty.is_promise_instance());
        if !is_maybe_promise {
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
                        expression.clone().trim_leading_trivia()?,
                    ));

                mutation.replace_node(expression, await_expression);
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
