use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;
use biome_rule_options::no_react_leaked_event_listener::NoReactLeakedEventListenerOptions;

declare_lint_rule! {
    /// Disallow forgetting to remove event listeners within `useEffect`.
    ///
    /// This rule detects `addEventListener` calls within `useEffect` hooks that don't have a corresponding
    /// `removeEventListener` call in the cleanup function. Forgetting to remove an event listener can lead to
    /// memory leaks and unexpected behavior when components unmount or dependencies change.
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
    ///     const handleClick = () => console.log("clicked");
    ///     window.addEventListener("click", handleClick);
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
    ///     const handleClick = () => console.log("clicked");
    ///     window.addEventListener("click", handleClick);
    ///     return () => window.removeEventListener("click", handleClick);
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedEventListener {
        version: "next",
        name: "noReactLeakedEventListener",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactXyz("web-api-no-leaked-event-listener").same(), RuleSource::EslintReactWebApi("no-leaked-event-listener").same()],
    }
}

impl Rule for NoReactLeakedEventListener {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedEventListenerOptions;

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
