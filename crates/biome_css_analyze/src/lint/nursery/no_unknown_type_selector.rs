use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::CssTypeSelector;
use biome_diagnostics::Severity;
use biome_rowan::AstNode;

use crate::utils::is_known_type_selector;

declare_lint_rule! {
    /// Disallow unknown type selectors.
    ///
    /// This rule considers tags defined in the HTML, SVG, and MathML specifications to be known.
    /// For details on known CSS type selectors, see the following links
    /// - https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors
    /// - https://developer.mozilla.org/ja/docs/Web/HTML/Element
    /// - https://developer.mozilla.org/ja/docs/Web/SVG/Element
    /// - https://developer.mozilla.org/ja/docs/Web/MathML/Element
    ///
    /// This rule allows custom elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// unknown {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// unknown > ul {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// x-Foo {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// input {}
    /// ```
    ///
    /// ```css
    /// ul > li {}
    /// ```
    ///
    /// ```css
    /// x-foo {}
    /// ```
    ///
    pub NoUnknownTypeSelector {
        version: "1.9.4",
        name: "noUnknownTypeSelector",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("selector-type-no-unknown")],
    }
}

impl Rule for NoUnknownTypeSelector {
    type Query = Ast<CssTypeSelector>;
    type State = CssTypeSelector;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let css_type_selector = ctx.query();
        let type_selector = css_type_selector
            .ident()
            .ok()?
            .value_token()
            .ok()?
            .token_text_trimmed();
        if !is_known_type_selector(&type_selector) {
            return Some(css_type_selector.clone());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unknown type selector is not allowed."
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Type_selectors">"MDN web docs"</Hyperlink>" for more details."
            }).note(markup! {
                "Consider replacing the unknown type selector with valid one."})
            )
    }
}
