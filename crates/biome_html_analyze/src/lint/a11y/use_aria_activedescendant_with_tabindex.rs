use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::use_aria_activedescendant_with_tabindex::UseAriaActivedescendantWithTabindexOptions;

declare_lint_rule! {
    /// Enforce that `tabindex` is assigned to non-interactive HTML elements with `aria-activedescendant`.
    ///
    /// `aria-activedescendant` is used to manage focus within a [composite widget](https://www.w3.org/TR/wai-aria/#composite).
    /// The element with the attribute `aria-activedescendant` retains the active document focus.
    ///
    /// It indicates which of its child elements has secondary focus by assigning the ID of that
    /// element to the value of `aria-activedescendant`. This pattern is used to build a widget
    /// like a search typeahead select list. The search input box retains document focus
    /// so that the user can type in the input. If the down arrow key is pressed and
    /// a search suggestion is highlighted, the ID of the suggestion element will be applied
    /// as the value of `aria-activedescendant` on the input element.
    ///
    /// Because an element with `aria-activedescendant` must be tabbable,
    /// it must either have an inherent tabIndex of zero or declare a tabindex attribute.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<DIV>`, `<div>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div aria-activedescendant="some-id"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div aria-activedescendant="some-id" tabindex="0"></div>
    /// ```
    ///
    /// ```html
    /// <input aria-activedescendant="some-id" />
    /// ```
    ///
    /// ```html
    /// <button aria-activedescendant="some-id"></button>
    /// ```
    ///
    pub UseAriaActivedescendantWithTabindex {
        version: "next",
        name: "useAriaActivedescendantWithTabindex",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("aria-activedescendant-has-tabindex").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

/// HTML elements that are natively interactive (focusable without tabindex).
const INTERACTIVE_ELEMENTS: &[&str] = &[
    "button", "input", "select", "textarea",
];

impl Rule for UseAriaActivedescendantWithTabindex {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseAriaActivedescendantWithTabindexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Must have aria-activedescendant attribute
        element.find_attribute_by_name("aria-activedescendant")?;

        // Get element name
        let element_name = element.name()?;
        let element_name = element_name.text();

        let source_type = ctx.source_type::<HtmlFileSource>();

        // Skip interactive elements (they are natively tabbable)
        // In HTML files: case-insensitive (BUTTON, Button, button all match)
        // In component frameworks (Vue, Svelte, Astro): case-sensitive (only lowercase matches)
        if is_interactive_element(element_name, source_type.is_html()) {
            return None;
        }

        // Skip anchor elements with href (natively focusable)
        if is_anchor_with_href(element, element_name, source_type.is_html()) {
            return None;
        }

        // Check if tabindex is already present
        if element.find_attribute_by_name("tabindex").is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                element.range(),
                "Enforce elements with aria-activedescendant are tabbable.",
            )
            .note(
                "aria-activedescendant is used to manage focus within a composite widget.\nThe element with the attribute aria-activedescendant retains the active document focus.",
            )
            .note(
                markup! {
                    "Add the "<Emphasis>"tabindex"</Emphasis>" attribute to the element with a value greater than or equal to -1."
                },
            ),
        )
    }
}

/// Check if the HTML element is natively interactive.
fn is_interactive_element(element_name: &str, is_html: bool) -> bool {
    if is_html {
        INTERACTIVE_ELEMENTS
            .iter()
            .any(|&name| element_name.eq_ignore_ascii_case(name))
    } else {
        INTERACTIVE_ELEMENTS.contains(&element_name)
    }
}

/// Check if the element is an anchor with an href (which is natively focusable).
fn is_anchor_with_href(element: &AnyHtmlElement, element_name: &str, is_html: bool) -> bool {
    let is_anchor = if is_html {
        element_name.eq_ignore_ascii_case("a")
    } else {
        element_name == "a"
    };
    is_anchor && element.find_attribute_by_name("href").is_some()
}
