use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArgumentList, JsCallExpression,
    JsConditionalExpression, JsNewExpression, JsSyntaxKind,
};
use biome_js_type_info::{Type, TypeMemberKind};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};

use crate::{JsRuleAction, ast_utils::is_in_async_function, services::typed::Typed};

declare_lint_rule! {
    /// Disallow Promises to be used in places where they are almost certainly a
    /// mistake.
    ///
    /// In most cases, if you assign a `Promise` somewhere a `Promise` is not
    /// allowed, the TypeScript compiler will be able to catch such a mistake.
    /// But there are a few places where TypeScript allows them -- they're not
    /// _necessarily_ a mistake -- even though they could be considered almost
    /// certainly to be one.
    ///
    /// This rule disallows using Promises in such places.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=promise-in-condition.js
    /// const promise = Promise.resolve('value');
    /// if (promise) { /* This branch will always execute */ }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=promise-in-ternary-condition.js
    /// const promise = Promise.resolve('value');
    /// const val = promise ? 123 : 456; // Always evaluates to `123`.
    /// ```
    ///
    /// ```js,expect_diagnostic,file=promise-in-filter.js
    /// // The following filter has no effect:
    /// const promise = Promise.resolve('value');
    /// [1, 2, 3].filter(() => promise);
    /// ```
    ///
    /// ```js,expect_diagnostic,file=promise-while-condition.js
    /// const promise = Promise.resolve('value');
    /// while (promise) { /* This is an endless loop */ }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=spread-promise.js
    /// // Using a `Promise` as an iterable expands to nothing:
    /// const getData = () => fetch('/');
    /// console.log({ foo: 42, ...getData() });
    /// ```
    ///
    /// ```js,expect_diagnostic,file=promise-in-forEach.js
    /// // These `fetch`-es are not `await`-ed in order:
    /// [1, 2, 3].forEach(async value => {
    ///     await fetch(`/${value}`);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=valid-promises.js
    /// const promise = Promise.resolve('value');
    /// if (await promise) { /* Do something */ }
    ///
    /// const val = (await promise) ? 123 : 456;
    ///
    /// while (await promise) { /* Do something */ }
    ///
    /// const getData = () => fetch('/');
    /// console.log({ foo: 42, ...(await getData()) });
    ///
    /// // for-of puts `await` in outer context:
    /// for (const value of [1, 2, 3]) {
    ///     await doSomething(value);
    /// }
    /// ```
    ///
    pub NoMisusedPromises {
        version: "2.1.0",
        name: "noMisusedPromises",
        language: "ts",
        recommended: true,
        sources: &[RuleSource::EslintTypeScript("no-misused-promises").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Project],
    }
}

pub enum NoMisusedPromisesState {
    Conditional,
    ConditionalReturn,
    Spread,
    VoidReturn,
}

impl Rule for NoMisusedPromises {
    type Query = Typed<AnyJsExpression>;
    type State = NoMisusedPromisesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        let ty = ctx.type_of_expression(expression);
        if ty.is_function() {
            find_misused_promise_returning_callback(ctx, expression, &ty)
        } else {
            find_misused_promise_expression(expression, &ty)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            NoMisusedPromisesState::Conditional => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "A Promise was found where a conditional was expected."
                    },
                )
                .note(markup! {
                    "A Promise is always truthy, so this is most likely a mistake."
                })
                .note(markup! {
                    "You may have intended to `await` the Promise instead."
                }),
            ),
            NoMisusedPromisesState::ConditionalReturn => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "This function returns a Promise where a conditional "
                        "was expected."
                    },
                )
                .note(markup! {
                    "A Promise is always truthy, so this is most likely a mistake."
                })
                .note(markup! {
                    "You may have intended to `await` the Promise, but this "
                    "does not work inside a synchronous callback."
                }),
            ),
            NoMisusedPromisesState::VoidReturn => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "This function returns a Promise, but no return value "
                        "was expected."
                    },
                )
                .note(markup! {
                    "This may not have the desired result if you expect the "
                    "Promise to be `await`-ed."
                }),
            ),
            NoMisusedPromisesState::Spread => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "A Promise was found where an iterable was expected."
                    },
                )
                .note(markup! {
                    "The spread syntax is used to expand an iterable, but a "
                    "Promise needs to be `await`-ed to take its value."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        use NoMisusedPromisesState::*;
        if matches!(state, ConditionalReturn | VoidReturn) {
            return None; // These cannot be automatically fixed.
        }

        let expression = ctx.query();

        if !is_in_async_function(expression.syntax()) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let await_expression = AnyJsExpression::JsAwaitExpression(make::js_await_expression(
            make::token(JsSyntaxKind::AWAIT_KW)
                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            expression.clone().trim_leading_trivia()?,
        ));

        mutation.replace_node(expression.clone(), await_expression);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add await operator." }.to_owned(),
            mutation,
        ))
    }
}

fn find_misused_promise_expression(
    expression: &AnyJsExpression,
    ty: &Type,
) -> Option<NoMisusedPromisesState> {
    let parent = expression.syntax().parent()?;
    let state = match parent.kind() {
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
            if JsConditionalExpression::cast(parent)
                .is_some_and(|conditional| conditional.test().is_ok_and(|test| test == *expression))
            {
                NoMisusedPromisesState::Conditional
            } else {
                return None; // Promise occurred in the branches.
            }
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => NoMisusedPromisesState::Conditional,
        JsSyntaxKind::JS_IF_STATEMENT => NoMisusedPromisesState::Conditional,
        JsSyntaxKind::JS_SPREAD => NoMisusedPromisesState::Spread,
        JsSyntaxKind::JS_WHILE_STATEMENT => NoMisusedPromisesState::Conditional,
        _ => return None,
    };

    // Uncomment the following line for debugging convenience:
    //let printed = format!("type of {expression:?} = {ty:?}");
    let is_promise = ty.is_promise_instance();
    let is_maybe_promise = ty.has_variant(|ty| ty.is_promise_instance());
    let should_signal =
        // "maybe" promises could still be considered conditionals, so we
        // don't signal those for the conditional variants
        is_promise || (!matches!(state, NoMisusedPromisesState::Conditional) && is_maybe_promise);
    should_signal.then_some(state)
}

fn find_misused_promise_returning_callback(
    ctx: &RuleContext<NoMisusedPromises>,
    expression: &AnyJsExpression,
    ty: &Type,
) -> Option<NoMisusedPromisesState> {
    let callback = ty.as_function()?;

    let return_ty = callback.return_type.as_type()?;
    let return_ty = ty.resolve(return_ty)?;

    let should_signal =
        return_ty.is_promise_instance() || return_ty.has_variant(|ty| ty.is_promise_instance());
    if !should_signal {
        return None;
    }

    let argument = expression
        .syntax()
        .ancestors()
        .find_map(AnyJsCallArgument::cast)?;
    let argument_list = argument.parent::<JsCallArgumentList>()?;
    let argument_index = argument_list
        .iter()
        .position(|arg| arg.is_ok_and(|arg| arg == argument))?;

    let argument_ty = if let Some(call_expression) = argument_list
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsCallExpression::cast)
    {
        let callee_ty = ctx.type_of_expression(&call_expression.callee().ok()?);
        let function = callee_ty.as_function()?;
        callee_ty.resolve(function.parameters.get(argument_index)?.ty())?
    } else if let Some(new_expression) = argument_list
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsNewExpression::cast)
    {
        let callee_ty = ctx.type_of_expression(&new_expression.callee().ok()?);
        let class = callee_ty.as_class()?;
        let constructor = class
            .members
            .iter()
            .find(|member| member.kind == TypeMemberKind::Constructor)?;
        let constructor_ty = callee_ty.resolve(&constructor.ty)?;
        let constructor = constructor_ty.as_function()?;
        constructor_ty.resolve(constructor.parameters.get(argument_index)?.ty())?
    } else {
        return None;
    };

    if argument_ty.is_function_with_return_type(|ty| ty.is_conditional()) {
        Some(NoMisusedPromisesState::ConditionalReturn)
    } else if argument_ty.is_function_with_return_type(|ty| ty.is_void_keyword()) {
        Some(NoMisusedPromisesState::VoidReturn)
    } else {
        None
    }
}
