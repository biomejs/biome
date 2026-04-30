use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;
use biome_rule_options::no_react_leaked_intersection_observer::NoReactLeakedIntersectionObserverOptions;

declare_lint_rule! {
    /// Disallow forgetting to disconnect `IntersectionObserver` within `useEffect`.
    ///
    /// This rule detects `IntersectionObserver` instances created within `useEffect` hooks that don't have a corresponding
    /// `disconnect()` call in the cleanup function. Forgetting to disconnect an IntersectionObserver can lead to memory
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
    ///     const observer = new IntersectionObserver((entries) => {
    ///       console.log("intersection");
    ///     });
    ///     observer.observe(document.body);
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
    ///     const observer = new IntersectionObserver((entries) => {
    ///       console.log("intersection");
    ///     });
    ///     observer.observe(document.body);
    ///     return () => observer.disconnect();
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedIntersectionObserver {
        version: "next",
        name: "noReactLeakedIntersectionObserver",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
    }
}

impl Rule for NoReactLeakedIntersectionObserver {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedIntersectionObserverOptions;

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
