use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_diagnostics::Severity;
use biome_html_syntax::AnyHtmlElement;

declare_lint_rule! {
    /// The scope prop should be used only on `<th>` elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,ignore
    /// <div scope="col" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html,ignore
    /// <th scope="col"></th>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 4.1.1](https://www.w3.org/WAI/WCAG21/Understanding/parsing)
    ///
    pub NoHeaderScope {
        version: "next",
        name: "noHeaderScope",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("scope")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoHeaderScope {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _element = ctx.query();

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        None
    }
}
