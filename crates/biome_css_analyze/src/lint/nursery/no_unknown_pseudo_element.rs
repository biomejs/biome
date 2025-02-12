use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssPseudoElement, CssPseudoElementSelector};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

use crate::utils::{is_pseudo_elements, vender_prefix};

declare_lint_rule! {
    /// Disallow unknown pseudo-element selectors.
    ///
    /// For details on known CSS pseudo-elements, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#list_of_pseudo-elements).
    ///
    /// This rule ignores vendor-prefixed pseudo-element selectors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a::pseudo {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a::PSEUDO {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a::element {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a:before {}
    /// ```
    ///
    /// ```css
    /// a::before {}
    /// ```
    ///
    /// ```css
    /// ::selection {}
    /// ```
    ///
    /// ```css
    /// input::-moz-placeholder {}
    /// ```
    ///
    pub NoUnknownPseudoElement {
        version: "1.8.0",
        name: "noUnknownPseudoElement",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("selector-pseudo-element-no-unknown")],
    }
}

impl Rule for NoUnknownPseudoElement {
    type Query = Ast<CssPseudoElementSelector>;
    type State = AnyCssPseudoElement;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node: &CssPseudoElementSelector = ctx.query();
        let pseudo_element = node.element().ok()?;

        let pseudo_element_name = match &pseudo_element {
            AnyCssPseudoElement::CssBogusPseudoElement(element) => element.to_trimmed_string(),
            AnyCssPseudoElement::CssPseudoElementFunctionIdentifier(ident) => {
                ident.name().ok()?.text().to_string()
            }
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(selector) => {
                selector.to_trimmed_string()
            }
            AnyCssPseudoElement::CssPseudoElementIdentifier(ident) => {
                ident.name().ok()?.to_trimmed_string().to_string()
            }
        };

        if !vender_prefix(pseudo_element_name.as_str()).is_empty()
            || is_pseudo_elements(pseudo_element_name.to_ascii_lowercase_cow().as_ref())
        {
            return None;
        }

        Some(pseudo_element)
    }

    fn diagnostic(_: &RuleContext<Self>, element: &Self::State) -> Option<RuleDiagnostic> {
        let span = element.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown pseudo-elements: "<Emphasis>{ element.to_trimmed_string() }</Emphasis>
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#list_of_pseudo-elements">"MDN web docs"</Hyperlink>" for more details."
            })
            .footer_list(
                markup! {
                    "Use a known pseudo-elements instead, such as:"
                },
                ["after", "backdrop", "before", "etc."],
            ),
        )
    }
}
