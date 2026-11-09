use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxToken};
use biome_rowan::AstSeparatedList;
use biome_rule_options::no_svelte_async_store::NoSvelteAsyncStoreOptions;

use crate::frameworks::framework_api_name;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow passing async functions to Svelte stores.
    ///
    /// The `writable()`, `readable()`, and `derived()` functions from `svelte/store` accept a
    /// function as their second argument, and Svelte uses the value that this function returns.
    /// An async function always returns a promise instead:
    ///
    /// - For `writable()` and `readable()`, Svelte calls the returned value when the store is no
    ///   longer used, for example when a component that reads the store is removed from the page.
    ///   Calling a promise throws an error.
    /// - For `derived()`, Svelte ignores the promise, so the cleanup never runs. If the function
    ///   takes a single parameter, the store holds the promise instead of the result.
    ///
    /// To do async work, keep the function synchronous and start the work inside it, then call
    /// `set()` once the result is ready.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { writable } from "svelte/store";
    /// const user = writable(null, async (set) => {
    ///     set(await fetchUser());
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { readable } from "svelte/store";
    /// const time = readable(new Date(), async function (set) {
    ///     set(await fetchTime());
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { derived } from "svelte/store";
    /// const profile = derived(user, async ($user) => await fetchProfile($user));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { derived, writable } from "svelte/store";
    /// const user = writable(null, (set) => {
    ///     fetchUser().then(set);
    /// });
    /// const profile = derived(user, ($user, set) => {
    ///     fetchProfile($user).then(set);
    /// });
    /// ```
    ///
    /// ## References
    ///
    /// - [`svelte/store`](https://svelte.dev/docs/svelte/svelte-store)
    pub NoSvelteAsyncStore {
        version: "next",
        name: "noSvelteAsyncStore",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-store-async").same()],
        recommended: true,
    }
}

/// An async function passed to a store function.
pub struct RuleState {
    store: StoreFunction,
    /// The `async` keyword of the function.
    async_token: JsSyntaxToken,
}

impl Rule for NoSvelteAsyncStore {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoSvelteAsyncStoreOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let name = framework_api_name(&call.callee().ok()?, ctx.model(), SVELTE_STORE_PACKAGE, None)?;
        let store = StoreFunction::from_name(name.text())?;

        let function = call.arguments().ok()?.args().iter().nth(1)?.ok()?;
        let async_token = match function.as_any_js_expression()?.clone().omit_parentheses() {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.async_token(),
            AnyJsExpression::JsFunctionExpression(function) => function.async_token(),
            _ => None,
        }?;

        Some(RuleState { store, async_token })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.async_token.text_trimmed_range(),
                markup! {
                    "This async function is passed to "<Emphasis>{state.store}</Emphasis>"."
                },
            )
            .note(match state.store {
                StoreFunction::Readable | StoreFunction::Writable => markup! {
                    "Svelte calls the value this function returns when the store is no longer used. An async function returns a promise, and calling a promise throws an error."
                },
                StoreFunction::Derived => markup! {
                    "Svelte uses the value this function returns, either to clean up or as the value of the store. An async function returns a promise instead, so the cleanup never runs or the store holds the promise."
                },
            })
            .note(markup! {
                "Make the function synchronous, start the async work inside it, and call "<Emphasis>"set()"</Emphasis>" when the result is ready."
            }),
        )
    }
}

const SVELTE_STORE_PACKAGE: &[&str] = &["svelte/store"];

/// A function from `svelte/store` that creates a store.
#[derive(Clone, Copy)]
enum StoreFunction {
    Derived,
    Readable,
    Writable,
}

impl StoreFunction {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "derived" => Some(Self::Derived),
            "readable" => Some(Self::Readable),
            "writable" => Some(Self::Writable),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for StoreFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Derived => "derived()",
            Self::Readable => "readable()",
            Self::Writable => "writable()",
        })
    }
}
