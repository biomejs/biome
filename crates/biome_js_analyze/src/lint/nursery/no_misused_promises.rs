use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsSyntaxKind};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

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
    /// ```js
    /// const promise = Promise.resolve('value');
    /// if (promise) { /* Do something */ }
    /// ```
    ///
    /// ```js
    /// const promise = Promise.resolve('value');
    /// const val = promise ? 123 : 456;
    /// ```
    ///
    /// ```js
    /// const promise = Promise.resolve('value');
    /// [1, 2, 3].filter(() => promise);
    /// ```
    ///
    /// ```js
    /// const promise = Promise.resolve('value');
    /// while (promise) { /* Do something */ }
    /// ```
    ///
    /// ```js
    /// const getData = () => fetch('/');
    /// console.log({ foo: 42, ...getData() });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const promise = Promise.resolve('value');
    /// if (await promise) { /* Do something */ }
    ///
    /// const val = (await promise) ? 123 : 456;
    ///
    /// while (await promise) { /* Do something */ }
    ///
    /// const getData = () => fetch('/');
    /// console.log({ foo: 42, ...(await getData()) });
    /// ```
    ///
    pub NoMisusedPromises {
        version: "next",
        name: "noMisusedPromises",
        language: "ts",
        recommended: true,
        sources: &[RuleSource::EslintTypeScript("no-misused-promises")],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Project],
    }
}

pub enum NoMisusedPromisesState {
    Conditional,
    Spread,
}

impl Rule for NoMisusedPromises {
    type Query = Typed<AnyJsExpression>;
    type State = NoMisusedPromisesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();

        let parent = expression.syntax().parent()?;
        let state = match parent.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => NoMisusedPromisesState::Conditional,
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => NoMisusedPromisesState::Conditional,
            JsSyntaxKind::JS_IF_STATEMENT => NoMisusedPromisesState::Conditional,
            JsSyntaxKind::JS_SPREAD => NoMisusedPromisesState::Spread,
            JsSyntaxKind::JS_WHILE_STATEMENT => NoMisusedPromisesState::Conditional,
            _ => return None,
        };

        let ty = ctx.type_of_expression(expression);

        // Uncomment the following line for debugging convenience:
        //let printed = format!("type of {expression:?} = {ty:?}");
        let should_signal = ty.is_promise_instance()
            || matches!(state, NoMisusedPromisesState::Spread)
                && ty.has_variant(|ty| ty.is_promise_instance());

        should_signal.then_some(state)
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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
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
