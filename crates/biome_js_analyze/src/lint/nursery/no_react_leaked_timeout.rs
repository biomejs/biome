use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;
use biome_rule_options::no_react_leaked_timeout::NoReactLeakedTimeoutOptions;

declare_lint_rule! {
    /// Disallow forgetting to clear `setTimeout` within `useEffect`.
    ///
    /// This rule detects `setTimeout` calls within `useEffect` hooks that don't have a corresponding
    /// `clearTimeout` call in the cleanup function. Forgetting to clear a timeout can lead to memory
    /// leaks and unexpected behavior when components unmount or dependencies change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const timeoutId = setTimeout(() => {
    ///       console.log("Hello");
    ///     }, 1000);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const timeoutId = setTimeout(() => {
    ///       console.log("Hello");
    ///     }, 1000);
    ///     return () => clearTimeout(timeoutId);
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedTimeout {
        version: "next",
        name: "noReactLeakedTimeout",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactXyz("web-api-no-leaked-timeout").same(), RuleSource::EslintReactWebApi("no-leaked-timeout").same()],
    }
}

impl Rule for NoReactLeakedTimeout {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedTimeoutOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
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
