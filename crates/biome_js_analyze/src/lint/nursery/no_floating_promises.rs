use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsArrowFunctionExpression, JsExpressionStatement, JsFunctionDeclaration,
    JsMethodClassMember, JsMethodObjectMember, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeCast, TriviaPieceKind};

use crate::{JsRuleAction, services::typed::Typed};

declare_lint_rule! {
    /// Require Promise-like statements to be handled appropriately.
    ///
    /// A "floating" `Promise` is one that is created without any code set up to handle any errors it might throw.
    /// Floating Promises can lead to several issues, including improperly sequenced operations, unhandled Promise rejections, and other unintended consequences.
    ///
    /// This rule will report Promise-valued statements that are not treated in one of the following ways:
    /// - Calling its `.then()` method with two arguments
    /// - Calling its `.catch()` method with one argument
    /// - `await`ing it
    /// - `return`ing it
    /// - `void`ing it
    ///
    /// :::caution
    /// ## Important notes
    ///
    /// This rule is a work in progress, and is only partially implemented.
    /// Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/3187
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts
    /// async function returnsPromise(): Promise<string> {
    ///   return 'value';
    /// }
    /// returnsPromise().then(() => {});
    /// ```
    ///
    /// ```ts
    /// const returnsPromise = async (): Promise<string> => {
    ///   return 'value';
    /// }
    /// async function returnsPromiseInAsyncFunction() {
    ///   returnsPromise().then(() => {});
    /// }
    /// ```
    ///
    /// ```ts
    /// const promise = new Promise((resolve) => resolve('value'));
    /// promise.then(() => { }).finally(() => { });
    /// ```
    ///
    /// ```ts
    /// Promise.all([p1, p2, p3])
    /// ```
    ///
    /// ```ts
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
    /// ```ts
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
    /// ```ts
    /// class Api {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   }
    /// }
    /// const api = new Api();
    /// api.returnsPromise().then(() => {}).finally(() => {});
    /// ```
    ///
    /// ```ts
    /// const obj = {
    ///   async returnsPromise(): Promise<string> {
    ///     return 'value';
    ///   },
    /// };
    ///
    /// obj.returnsPromise();
    /// ```
    ///
    /// ```ts
    /// type Props = {
    ///   returnsPromise: () => Promise<void>;
    /// };
    ///
    /// async function testCallingReturnsPromise(props: Props) {
    ///   props.returnsPromise();
    /// }
    /// ```
    /// ### Valid
    ///
    /// ```ts
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
    /// ```
    ///
    /// ```ts
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
        version: "next",
        name: "noFloatingPromises",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-floating-promises")],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoFloatingPromises {
    type Query = Typed<JsExpressionStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let expression = node.expression().ok()?;
        let ty = ctx.type_for_expression(&expression);

        // Uncomment the following line for debugging convenience:
        //let printed = format!("type of {expression:?} = {ty:?}");
        if !ty.is_promise_instance() {
            return None;
        }

        if is_handled_promise(expression).unwrap_or_default() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "A \"floating\" Promise was found, meaning it is not properly handled and could lead to ignored errors or unexpected behavior."
                },
            )
            .note(markup! {
                "This happens when a Promise is not awaited, lacks a `.catch` or `.then` rejection handler, or is not explicitly ignored using the `void` operator."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if !is_in_async_function(node) {
            return None;
        }

        let expression = node.expression().ok()?;
        let mut mutation = ctx.root().begin();
        let await_expression = AnyJsExpression::JsAwaitExpression(make::js_await_expression(
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

/// Checks if the given `JsExpressionStatement` is within an async function.
///
/// This function traverses up the syntax tree from the given expression node
/// to find the nearest function and checks if it is an async function. It
/// supports arrow functions, function declarations, class methods, and object
/// methods.
///
/// # Arguments
///
/// * `node` - A reference to a `JsExpressionStatement` to check.
///
/// # Returns
///
/// * `true` if the expression is within an async function.
/// * `false` otherwise.
fn is_in_async_function(node: &JsExpressionStatement) -> bool {
    node.syntax()
        .ancestors()
        .find_map(|ancestor| match ancestor.kind() {
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => ancestor
                .cast::<JsArrowFunctionExpression>()
                .and_then(|func| func.async_token()),
            JsSyntaxKind::JS_FUNCTION_DECLARATION => ancestor
                .cast::<JsFunctionDeclaration>()
                .and_then(|func| func.async_token()),
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER => ancestor
                .cast::<JsMethodClassMember>()
                .and_then(|method| method.async_token()),
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => ancestor
                .cast::<JsMethodObjectMember>()
                .and_then(|method| method.async_token()),
            _ => None,
        })
        .is_some()
}
