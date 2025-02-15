use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::CssDeclarationImportant;
use biome_diagnostics::Severity;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow using `!important` in declarations
    ///
    /// Using `!important` is considered bad practice as it makes styles harder to maintain
    /// by breaking the natural cascading in CSS.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   color: red !important;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///
    ///  color: red;
    /// }
    /// ```
    pub NoImportant {
        version: "next",
        name: "noImportant",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::Stylelint("declaration-no-important")],
    }
}

impl Rule for NoImportant {
    type Query = Ast<CssDeclarationImportant>;
    type State = CssDeclarationImportant;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        Some(node.clone())
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = state;
        let span = node.range();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected "<Emphasis>"!important"</Emphasis>" declaration."
                },
            )
            .note(markup! {
                "Using "<Emphasis>"!important"</Emphasis>" breaks the natural cascading of CSS rules and makes styles harder to maintain."
            }),
        )
    }
}
