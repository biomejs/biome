use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlContent, AnyHtmlElement, HtmlElementList, HtmlSyntaxKind, T};
use biome_parser::{TokenSet, token_set};
use biome_rowan::AstNode;
use biome_rule_options::use_control_label::UseControlLabelOptions;

use crate::a11y::{
    get_truthy_aria_hidden_attribute, has_non_empty_attribute, html_element_has_truthy_aria_hidden,
    html_self_closing_element_has_accessible_name,
    html_self_closing_element_has_non_empty_attribute,
    html_self_closing_element_has_truthy_aria_hidden,
};

declare_lint_rule! {
    /// Enforce that interactive control elements have an accessible label.
    ///
    /// A control with no accessible label is announced by assistive technology
    /// as an anonymous control (e.g. just "button"), leaving its purpose
    /// unclear. A label can come from text content, `aria-label`,
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
        version: "2.5.9",
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
const CONTROL_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(T![button], T![menuitem]);
/// Attributes that supply an accessible name for these controls.
const LABEL_ATTRIBUTES: &[&str] = &["aria-label", "aria-labelledby", "title"];
const EMPTY_CONTENT_ELEMENTS: TokenSet<HtmlSyntaxKind> =
    token_set!(T![br], T![hr], T![wbr], T![meta], T![link], T![base], T![col]);

impl Rule for UseControlLabel {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseControlLabelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let tag_element = node.clone().as_any_html_tag_element()?;
        if !tag_element
            .tag_name_kind()
            .is_some_and(|kind| CONTROL_ELEMENTS.contains(kind))
        {
            return None;
        }

        // An element hidden from the accessibility tree does not need a label.
        if get_truthy_aria_hidden_attribute(&tag_element).is_some() {
            return None;
        }

        // A labeling attribute supplies the accessible name.
        if has_labeling_attribute(&tag_element) {
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

        if has_accessible_content(&html_element.children()) {
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

/// Whether the element carries a labeling attribute with a usable value,
/// including Vue `:` / `v-bind:` bindings.
fn has_labeling_attribute(element: &AnyHtmlTagElement) -> bool {
    LABEL_ATTRIBUTES
        .iter()
        .any(|name| has_non_empty_attribute(element, name))
}

/// Checks if `HtmlElementList` contains accessible content (non-empty text or visible elements).
fn has_accessible_content(html_child_list: &HtmlElementList) -> bool {
    html_child_list.into_iter().any(|child| match &child {
        AnyHtmlElement::AnyHtmlContent(content) => is_accessible_text_content(content),
        AnyHtmlElement::HtmlElement(element) => {
            if html_element_has_truthy_aria_hidden(element) {
                return false;
            }

            let Some(opening) = element.opening_element().ok() else {
                return has_accessible_content(&element.children());
            };
            let tag_element = AnyHtmlTagElement::from(opening);
            // Custom components may render accessible content at runtime.
            if tag_element.tag_name_kind().is_none() {
                return true;
            }

            has_labeling_attribute(&tag_element) || has_accessible_content(&element.children())
        }
        AnyHtmlElement::HtmlSelfClosingElement(element) => {
            if html_self_closing_element_has_truthy_aria_hidden(element) {
                return false;
            }

            if html_self_closing_element_has_accessible_name(element) {
                return true;
            }

            match element.name().ok().and_then(|name| name.tag_name_kind()) {
                Some(T![img]) => html_self_closing_element_has_non_empty_attribute(element, "alt"),
                Some(kind) if EMPTY_CONTENT_ELEMENTS.contains(kind) => false,
                Some(T![input]) => {
                    let is_hidden =
                        element
                            .find_attribute_or_vue_binding("type")
                            .is_some_and(|attr| {
                                attr.as_static_value()
                                    .is_some_and(|s| s.text().eq_ignore_ascii_case("hidden"))
                            });
                    !is_hidden
                }
                // Custom components may render accessible content at runtime.
                None => true,
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
