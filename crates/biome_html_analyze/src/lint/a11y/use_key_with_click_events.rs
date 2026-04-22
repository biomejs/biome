use crate::a11y::is_hidden_from_screen_reader;
use crate::{Aria, utils::is_html_tag};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlFileSource;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::AstNode;
use biome_rule_options::use_key_with_click_events::UseKeyWithClickEventsOptions;

declare_lint_rule! {
    /// Enforce elements with a click event handler to also have at least one keyboard event handler.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse,
    /// AT compatibility, and screen reader users. This rule checks that interactive elements with an
    /// `onclick` attribute also include at least one of `onkeydown`, `onkeyup`, or `onkeypress`.
    ///
    /// This does not apply to elements that are inherently keyboard-accessible (such as `<button>`,
    /// `<input>`, `<select>`, `<textarea>`, or `<a>` with an `href` attribute) or elements that are
    /// hidden from assistive technologies.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div onclick="handleClick()"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeydown="handleKeyDown()"></div>
    /// ```
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeyup="handleKeyUp()"></div>
    /// ```
    ///
    /// ```html
    /// <div onclick="handleClick()" onkeypress="handleKeyPress()"></div>
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

impl Rule for UseKeyWithClickEvents {
    type Query = Aria<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseKeyWithClickEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let aria_roles = ctx.aria_roles();

        // Only flag elements that have an onclick attribute
        if element.find_attribute_by_name("onclick").is_none()
            && element.find_vue_event_handling_directive("click").is_none()
        {
            return None;
        }

        if element.is_custom_component() {
            return None;
        }

        if is_hidden_from_screen_reader(element) || aria_roles.is_presentation_role(element) {
            return None;
        }

        if !aria_roles.is_not_interactive_element(element) {
            let source_type = ctx.source_type::<HtmlFileSource>();
            if !is_html_tag(element, source_type, "a")
                || element.find_attribute_by_name("href").is_some()
            {
                return None;
            }
        }

        // Check for keyboard event handlers
        if element.find_attribute_by_name("onkeydown").is_some()
            || element
                .find_vue_event_handling_directive("keydown")
                .is_some()
            || element.find_attribute_by_name("onkeyup").is_some()
            || element.find_vue_event_handling_directive("keyup").is_some()
            || element.find_attribute_by_name("onkeypress").is_some()
            || element
                .find_vue_event_handling_directive("keypress")
                .is_some()
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
