use crate::frameworks::imported_member_name;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsCallArgument, AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_svelte_store_initial_value::UseSvelteStoreInitialValueOptions;

declare_lint_rule! {
    /// Require Svelte stores to be created with an initial value.
    ///
    /// A store created without an initial value holds `undefined` until it is set for the
    /// first time. Code that reads the store, such as a component that displays its value,
    /// then has to handle `undefined` in addition to the values the store is meant to hold.
    /// Passing an initial value makes the first value of the store explicit.
    ///
    /// This rule checks calls to the following functions imported from `svelte/store`:
    ///
    /// - `writable()` and `readable()`, which take the initial value as their first argument.
    /// - `derived()`, which takes the initial value as its third argument. The store holds
    ///   this value until its callback sets one. This matters most for a callback that sets
    ///   the value later, for example after a timer or a network request.
    ///
    /// Calls that spread an array into their arguments, such as `derived(...args)`, are not
    /// reported, because the rule can't tell how many arguments they pass.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { writable } from "svelte/store";
    ///
    /// export const count = writable();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { readable } from "svelte/store";
    ///
    /// export const time = readable();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { derived } from "svelte/store";
    /// import { count } from "./count.js";
    ///
    /// export const delayed = derived(count, ($count, set) => {
    ///     setTimeout(() => set($count), 1000);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { derived, readable, writable } from "svelte/store";
    ///
    /// export const count = writable(0);
    /// export const time = readable(new Date());
    /// export const delayed = derived(
    ///     count,
    ///     ($count, set) => {
    ///         setTimeout(() => set($count), 1000);
    ///     },
    ///     0,
    /// );
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte stores](https://svelte.dev/docs/svelte/stores)
    /// - [`svelte/store`](https://svelte.dev/docs/svelte/svelte-store)
    pub UseSvelteStoreInitialValue {
        version: "next",
        name: "useSvelteStoreInitialValue",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("require-stores-init").same()],
        recommended: false,
    }
}

impl Rule for UseSvelteStoreInitialValue {
    type Query = Semantic<JsCallExpression>;
    type State = StoreFunction;
    type Signals = Option<Self::State>;
    type Options = UseSvelteStoreInitialValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        let callee = call.callee().ok()?.omit_parentheses();
        let function = StoreFunction::from_callee(&callee, ctx.model())?;

        let args = call.arguments().ok()?.args();
        if args.len() >= function.min_arguments() {
            return None;
        }

        // A spread argument can expand to any number of arguments, including the initial value.
        if args
            .iter()
            .any(|arg| matches!(arg, Ok(AnyJsCallArgument::JsSpread(_))))
        {
            return None;
        }

        Some(function)
    }

    fn diagnostic(ctx: &RuleContext<Self>, function: &Self::State) -> Option<RuleDiagnostic> {
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            callee.range(),
            markup! {
                "This "<Emphasis>{function}</Emphasis>" store is created without an initial value."
            },
        );

        Some(match function {
            StoreFunction::Writable | StoreFunction::Readable => diagnostic
                .note(markup! {
                    "Without an initial value, the store holds "<Emphasis>"undefined"</Emphasis>" until it is set for the first time, which code that reads the store may not expect."
                })
                .note(markup! {
                    "Pass the initial value as the first argument."
                }),
            StoreFunction::Derived => diagnostic
                .note(markup! {
                    "Without an initial value, the store holds "<Emphasis>"undefined"</Emphasis>" until its callback sets a value, which code that reads the store may not expect."
                })
                .note(markup! {
                    "Pass the initial value as the third argument."
                }),
        })
    }
}

const SVELTE_STORE: &str = "svelte/store";

/// A function exported by `svelte/store` that creates a store.
#[derive(Debug, Clone, Copy)]
pub enum StoreFunction {
    Writable,
    Readable,
    Derived,
}

impl StoreFunction {
    /// Returns the store function called by `callee`, if `callee` refers to an import
    /// from `svelte/store`.
    fn from_callee(callee: &AnyJsExpression, model: &SemanticModel) -> Option<Self> {
        let name = imported_member_name(callee, model, SVELTE_STORE)?;
        match name.text_trimmed() {
            "writable" => Some(Self::Writable),
            "readable" => Some(Self::Readable),
            "derived" => Some(Self::Derived),
            _ => None,
        }
    }

    /// Returns how many arguments a call needs for the initial value to be among them.
    const fn min_arguments(self) -> usize {
        match self {
            Self::Writable | Self::Readable => 1,
            Self::Derived => 3,
        }
    }
}

impl biome_console::fmt::Display for StoreFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Writable => "writable()",
            Self::Readable => "readable()",
            Self::Derived => "derived()",
        })
    }
}
