use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssPseudoElement, CssPseudoElementSelector};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_rule_options::no_unknown_pseudo_element::NoUnknownPseudoElementOptions;
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
        sources: &[RuleSource::Stylelint("selector-pseudo-element-no-unknown").same()],
    }
}

impl Rule for NoUnknownPseudoElement {
    type Query = Ast<CssPseudoElementSelector>;
    type State = AnyCssPseudoElement;
    type Signals = Option<Self::State>;
    type Options = NoUnknownPseudoElementOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node: &CssPseudoElementSelector = ctx.query();
        let pseudo_element = node.element().ok()?;

        let should_not_trigger = match &pseudo_element {
            AnyCssPseudoElement::CssBogusPseudoElement(element) => {
                should_not_trigger(element.to_trimmed_text().text())
            }
            AnyCssPseudoElement::CssPseudoElementFunctionCustomIdentifier(ident) => {
                should_not_trigger(ident.name().ok()?.to_trimmed_text().text())
            }
            AnyCssPseudoElement::CssPseudoElementFunctionSelector(selector) => {
                should_not_trigger(selector.name().ok()?.to_trimmed_text().text())
            }
            AnyCssPseudoElement::CssPseudoElementIdentifier(ident) => {
                should_not_trigger(ident.name().ok()?.to_trimmed_text().text())
            }
            AnyCssPseudoElement::CssPseudoElementFunction(ident) => {
                should_not_trigger(ident.name().ok()?.to_trimmed_text().text())
            }
        };

        if should_not_trigger {
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
                    "Unexpected unknown pseudo-elements: "<Emphasis>{ element.to_trimmed_text().text() }</Emphasis>
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

/// It doesn't trigger the rule if the pseudo-element name isn't a vendor prefix or is a pseudo-element
fn should_not_trigger(pseudo_element_name: &str) -> bool {
    !vender_prefix(pseudo_element_name).is_empty()
        || is_pseudo_elements(pseudo_element_name.to_ascii_lowercase_cow().as_ref())
}
