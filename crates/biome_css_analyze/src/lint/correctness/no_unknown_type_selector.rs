use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{CssPseudoElementFunctionSelector, CssTypeSelector};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_unknown_type_selector::NoUnknownTypeSelectorOptions;

use crate::utils::is_known_type_selector;

const VIEW_TRANSITION_PSEUDO_ELEMENTS: [&str; 5] = [
    "view-transition",
    "view-transition-group",
    "view-transition-image-pair",
    "view-transition-old",
    "view-transition-new",
];

fn is_root_in_view_transition_pseudo_element(type_selector: &CssTypeSelector) -> bool {
    // Only check if the type selector is "root"
    let Some(type_selector_text) = type_selector
        .ident()
        .ok()
        .and_then(|ident| ident.value_token().ok())
        .map(|token| token.token_text_trimmed().text().to_string())
    else {
        return false;
    };

    if type_selector_text != "root" {
        return false;
    }

    // According to the AST structure: CssTypeSelector -> CssCompoundSelector -> CssPseudoElementFunctionSelector
    type_selector
        .syntax()
        .grand_parent()
        .and_then(CssPseudoElementFunctionSelector::cast)
        .and_then(|func| func.name().ok())
        .and_then(|name| name.value_token().ok())
        .map(|token| token.token_text_trimmed().text().to_string())
        .is_some_and(|name_text| VIEW_TRANSITION_PSEUDO_ELEMENTS.contains(&name_text.as_str()))
}

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
        sources: &[RuleSource::Stylelint("selector-type-no-unknown").same()],
    }
}

impl Rule for NoUnknownTypeSelector {
    type Query = Ast<CssTypeSelector>;
    type State = CssTypeSelector;
    type Signals = Option<Self::State>;
    type Options = NoUnknownTypeSelectorOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let css_type_selector = ctx.query();

        // Skip "root" type selectors that are inside View Transition pseudo-element function parameters
        // (e.g., `root` in `::view-transition-old(root)`)
        if is_root_in_view_transition_pseudo_element(css_type_selector) {
            return None;
        }

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
