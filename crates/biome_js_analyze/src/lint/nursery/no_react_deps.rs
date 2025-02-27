use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsIdentifierBinding};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow usage of dependency arrays in `createEffect` and `createMemo`.
    /// 
    /// In Solid, `createEffect` and `createMemo` track dependencies automatically, it's no need to add dependency arrays. 
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal()]);
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal]); 
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// const deps = [signal];
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, deps)
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a(), b()]);
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b]);
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b()]);
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const deps = [a, b];
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), deps);
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const deps = [a, b];
    /// const memoFn = () => computeExpensiveValue(a(), b());
    /// const value = createMemo(memoFn, deps);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///    console.log(signal());
    /// });
    /// ```
    /// 
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return prev + 1;
    /// }, 0);
    /// ```
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return (prev || 0) + 1;
    /// });
    /// ```
    /// 
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return prev ? prev + 1 : 1;
    /// }, undefined);
    /// ```
    /// 
    /// ```js
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()));
    /// ```
    /// 
    /// ```js
    /// import { createMemo } from "solid-js";
    /// const sum = createMemo((prev) => input() + prev, 0);
    /// ```
    /// 
    /// ```js
    /// import { createEffect } from "solid-js";
    /// const args = [
    ///   () => {
    ///     console.log(signal());
    ///   },
    ///   [signal()],
    /// ];
    /// createEffect(...args);
    /// ```
    pub NoReactDeps {
        version: "next",
        name: "noReactDeps",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("no-react-deps")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoReactDeps {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
