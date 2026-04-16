use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsSyntaxKind, JsxNamespaceName};
use biome_rowan::AstNode;
use biome_rule_options::no_jsx_namespace::NoJsxNamespaceOptions;

declare_lint_rule! {
    /// Disallow JSX namespace syntax.
    ///
    /// React does not support XML namespaced tags such as `<ns:Component />`.
    /// Although the JSX specification permits namespaces, React does not implement
    /// them and using a namespaced element may cause a runtime error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <ns:testcomponent />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <svg:circle cx="50" cy="50" r="40" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <testcomponent />
    /// ```
    ///
    /// ```jsx
    /// <object.TestComponent />
    /// ```
    ///
    pub NoJsxNamespace {
        version: "next",
        name: "noJsxNamespace",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReactJsx("no-namespace").same(),
            RuleSource::EslintReactXyz("jsx-no-namespace").same(),
        ],
    }
}

impl Rule for NoJsxNamespace {
    type Query = Ast<JsxNamespaceName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoJsxNamespaceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let parent = node.syntax().parent()?;

        match parent.kind() {
            JsSyntaxKind::JSX_OPENING_ELEMENT | JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected JSX namespace syntax."
                },
            )
            .note(markup! {
                "React does not support namespaced JSX tags. Use a plain element name or a member expression instead."
            }),
        )
    }
}
