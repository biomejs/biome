use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlContent, AnyHtmlElement, HtmlElementList, HtmlSyntaxKind, T};
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_control_label::UseControlLabelOptions;

use crate::a11y::{
    get_truthy_aria_hidden_attribute, has_accessible_name, html_element_has_truthy_aria_hidden,
    html_self_closing_element_has_accessible_name,
    html_self_closing_element_has_non_empty_attribute,
    html_self_closing_element_has_truthy_aria_hidden,
};

declare_lint_rule! {
    /// Enforce that interactive control elements have an accessible label.
    ///
    /// A control with no accessible label is announced by assistive technology
    /// as an anonymous control (e.g. just "button"), leaving its purpose
    /// unclear. A label can come from text content, or from an `aria-label`,
    /// `aria-labelledby`, or `title` attribute.
    ///
    /// This rule checks native controls whose accessible name is expected to
    /// come from their own content or attributes (`button`, `menuitem`).
    /// Elements hidden from assistive technology with `aria-hidden` are
    /// skipped, as are elements that already require a text alternative under
    /// a dedicated rule (e.g. `area`, `img`, checked by `useAltText`).
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<BUTTON>`, `<Button>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<Button>` are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <button></button>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <button>   </button>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <button>Submit</button>
    /// ```
    ///
    /// ```html
    /// <button aria-label="Close"></button>
    /// ```
    ///
    /// ```html
    /// <button><span>Delete</span></button>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 3.3.2](https://www.w3.org/WAI/WCAG21/Understanding/labels-or-instructions)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub UseControlLabel {
        version: "next",
        name: "useControlLabel",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("control-has-associated-label").inspired()],
        recommended: false,
        severity: Severity::Error,
    }
}

/// Native interactive elements whose accessible name comes from their own
/// content or labeling attributes (rather than an external `<label>` or a
/// dedicated alt-text rule).
const CONTROL_ELEMENTS: &[HtmlSyntaxKind] = &[T![button], T![menuitem]];

impl Rule for UseControlLabel {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseControlLabelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        let tag_element = node.clone().as_any_html_tag_element()?;
        let tag_kind = tag_element.tag_name_kind()?;
        if !CONTROL_ELEMENTS.contains(&tag_kind) {
            return None;
        }

        // An element hidden from the accessibility tree does not need a label.
        if get_truthy_aria_hidden_attribute(&tag_element).is_some() {
            return None;
        }

        // A labeling attribute supplies the accessible name.
        if has_accessible_name(&tag_element) {
            return None;
        }

        // A self-closing control (e.g. `<button />`) can never have content.
        let Some(html_element) = node.as_html_element() else {
            return Some(());
        };
        // Skip analysis if we can't fully parse the element to avoid false positives.
        if html_element.opening_element().is_err() {
            return None;
        }

        let is_astro = source_type.is_astro();
        if has_accessible_content(&html_element.children(), is_astro) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This control element has no accessible label."
                },
            )
            .note(markup! {
                "Assistive technology announces it as an anonymous control, so its purpose is unclear to screen-reader users."
            })
            .note(markup! {
                "Add text content, or an "<Emphasis>"aria-label"</Emphasis>", "<Emphasis>"aria-labelledby"</Emphasis>", or "<Emphasis>"title"</Emphasis>" attribute."
            }),
        )
    }
}

/// Checks if `HtmlElementList` contains accessible content (non-empty text or visible elements).
fn has_accessible_content(html_child_list: &HtmlElementList, is_astro: bool) -> bool {
    html_child_list.into_iter().any(|child| match &child {
        AnyHtmlElement::AnyHtmlContent(content) => is_accessible_text_content(content),
        AnyHtmlElement::HtmlElement(element) => {
            if html_element_has_truthy_aria_hidden(element) {
                return false;
            }

            let has_own_accessible_name = element
                .opening_element()
                .ok()
                .is_some_and(|opening| has_accessible_name(&AnyHtmlTagElement::from(opening)));

            has_own_accessible_name || has_accessible_content(&element.children(), is_astro)
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            if html_self_closing_element_has_truthy_aria_hidden(element) {
                return false;
            }

            if html_self_closing_element_has_accessible_name(element) {
                return true;
            }

            let tag_text = element.name().ok().and_then(|n| n.token_text_trimmed());

            match tag_text.as_ref().map(|t| t.as_ref()) {
                Some(name) if name.eq_ignore_ascii_case("img") || (is_astro && name == "Image") => {
                    html_self_closing_element_has_non_empty_attribute(element, "alt")
                }
                Some(name)
                    if name.eq_ignore_ascii_case("br")
                        || name.eq_ignore_ascii_case("hr")
                        || name.eq_ignore_ascii_case("wbr")
                        || name.eq_ignore_ascii_case("meta")
                        || name.eq_ignore_ascii_case("link")
                        || name.eq_ignore_ascii_case("base")
                        || name.eq_ignore_ascii_case("col") =>
                {
                    false
                }
                Some(name) if name.eq_ignore_ascii_case("input") => {
                    let is_hidden =
                        element
                            .find_attribute_or_vue_binding("type")
                            .is_some_and(|attr| {
                                attr.as_static_value()
                                    .is_some_and(|s| s.text().eq_ignore_ascii_case("hidden"))
                            });
                    !is_hidden
                }
                // Custom components (PascalCase) may render accessible content.
                Some(name) if name.starts_with(|c: char| c.is_uppercase()) => true,
                _ => false,
            }
        }
        AnyHtmlElement::HtmlBogusElement(_)
        | AnyHtmlElement::HtmlCdataSection(_)
        | AnyHtmlElement::HtmlProcessingInstruction(_) => true,
    })
}

/// Checks if the content node contains non-empty text.
fn is_accessible_text_content(content: &AnyHtmlContent) -> bool {
    match content {
        AnyHtmlContent::HtmlContent(html_content) => html_content
            .value_token()
            .is_ok_and(|token| !token.text_trimmed().is_empty()),
        AnyHtmlContent::AnyHtmlTextExpression(_) => true,
        AnyHtmlContent::HtmlEmbeddedContent(_) => true,
    }
}
