use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::use_key_with_click_events::UseKeyWithClickEventsOptions;

use crate::a11y::attribute_value_equals_ignore_case;

declare_lint_rule! {
    /// Enforce that elements with `onclick` handlers also have at least one keyboard event handler.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse,
    /// AT compatibility, and screen reader users. This rule checks that interactive elements with an
    /// `onclick` attribute also include at least one of `onkeydown`, `onkeyup`, or `onkeypress`.
    ///
    /// This does not apply to elements that are inherently keyboard-accessible (such as `<a>`, `<button>`,
    /// `<input>`, `<select>`, `<textarea>`) or elements that are hidden from assistive technologies.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<DIV>`, `<Div>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<Div>` are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div onclick="handleClick()" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeydown="handleKeyDown()" />
    /// ```
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeyup="handleKeyUp()" />
    /// ```
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeypress="handleKeyPress()" />
    /// ```
    ///
    /// ```html
    /// <button onclick="handleClick()">Submit</button>
    /// ```
    ///
    /// ```html
    /// <input onclick="handleClick()" />
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub UseKeyWithClickEvents {
        version: "next",
        name: "useKeyWithClickEvents",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("click-events-have-key-events").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

/// Elements that are inherently keyboard-accessible and should not trigger this rule.
const INTERACTIVE_ELEMENTS: &[&str] = &[
    "a", "button", "input", "option", "select", "textarea",
];

impl Rule for UseKeyWithClickEvents {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseKeyWithClickEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let file_source = ctx.source_type::<HtmlFileSource>();

        // Only flag elements that have an onclick attribute
        element.find_attribute_by_name("onclick")?;

        let element_name = element.name()?;
        let is_html_file = file_source.is_html();

        // In framework files, skip PascalCase elements (assumed custom components)
        if !is_html_file && element_name.text().chars().any(|c| c.is_ascii_uppercase()) {
            return None;
        }

        let name_matches = |name: &str| -> bool {
            if is_html_file {
                element_name.eq_ignore_ascii_case(name)
            } else {
                element_name.text() == name
            }
        };

        // Skip inherently keyboard-accessible elements
        if INTERACTIVE_ELEMENTS.iter().any(|&n| name_matches(n)) {
            return None;
        }

        // Skip elements hidden from screen readers (aria-hidden with truthy value)
        if element
            .find_attribute_by_name("aria-hidden")
            .is_some_and(|attr| {
                attr.value()
                    .map_or(true, |v| !v.trim().eq_ignore_ascii_case("false"))
            })
        {
            return None;
        }

        // Skip elements with presentation/none role
        if element
            .find_attribute_by_name("role")
            .is_some_and(|attr| {
                attribute_value_equals_ignore_case(&attr, "presentation")
                    || attribute_value_equals_ignore_case(&attr, "none")
            })
        {
            return None;
        }

        // Check for keyboard event handlers
        if element.find_attribute_by_name("onkeydown").is_some()
            || element.find_attribute_by_name("onkeyup").is_some()
            || element.find_attribute_by_name("onkeypress").is_some()
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                element.syntax().text_trimmed_range(),
                markup! {
                    "Elements with "<Emphasis>"onclick"</Emphasis>" must also include at least one keyboard handler: "<Emphasis>"onkeydown"</Emphasis>", "<Emphasis>"onkeyup"</Emphasis>", or "<Emphasis>"onkeypress"</Emphasis>"."
                },
            )
            .note(markup! {
                "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation."
            }),
        )
    }
}
