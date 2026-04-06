use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::use_key_with_mouse_events::UseKeyWithMouseEventsOptions;

declare_lint_rule! {
    /// Enforce that `onmouseover` is accompanied by `onfocus` and `onmouseout` by `onblur`.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse,
    /// AT compatibility, and screen reader users.
    ///
    /// :::note
    /// In `.html` files, attribute names are matched case-insensitively.
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), PascalCase elements like `<MyComponent>`
    /// are assumed to be custom components and are ignored.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div onmouseover="handleMouseOver()"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div onmouseout="handleMouseOut()"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div onmouseover="handleMouseOver()" onfocus="handleFocus()"></div>
    /// ```
    ///
    /// ```html
    /// <div onmouseout="handleMouseOut()" onblur="handleBlur()"></div>
    /// ```
    ///
    /// ```html
    /// <div onmouseover="handleMouseOver()" onfocus="handleFocus()" onmouseout="handleMouseOut()" onblur="handleBlur()"></div>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub UseKeyWithMouseEvents {
        version: "next",
        name: "useKeyWithMouseEvents",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("mouse-events-have-key-events").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum UseKeyWithMouseEventsState {
    MissingOnFocus,
    MissingOnBlur,
}

impl Rule for UseKeyWithMouseEvents {
    type Query = Ast<AnyHtmlElement>;
    type State = UseKeyWithMouseEventsState;
    type Signals = Option<Self::State>;
    type Options = UseKeyWithMouseEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        // In Vue/Svelte/Astro, PascalCase tag names indicate custom components
        // and should be skipped. In HTML, all tag names are native elements
        // regardless of casing (e.g. <DIV> is a valid div).
        if !source_type.is_html() {
            let element_name = node.name()?;
            if element_name.text().starts_with(|c: char| c.is_uppercase()) {
                return None;
            }
        }

        if node.find_attribute_by_name("onmouseover").is_some()
            && node.find_attribute_by_name("onfocus").is_none()
        {
            return Some(UseKeyWithMouseEventsState::MissingOnFocus);
        }

        if node.find_attribute_by_name("onmouseout").is_some()
            && node.find_attribute_by_name("onblur").is_none()
        {
            return Some(UseKeyWithMouseEventsState::MissingOnBlur);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (message, note) = match state {
            UseKeyWithMouseEventsState::MissingOnFocus => (
                markup! {
                    <Emphasis>"onmouseover"</Emphasis>" must be accompanied by "<Emphasis>"onfocus"</Emphasis>"."
                },
                markup! {
                    "Users who navigate via keyboard cannot interact with elements that only have mouse event handlers. Add an "<Emphasis>"onfocus"</Emphasis>" attribute to ensure keyboard accessibility."
                },
            ),
            UseKeyWithMouseEventsState::MissingOnBlur => (
                markup! {
                    <Emphasis>"onmouseout"</Emphasis>" must be accompanied by "<Emphasis>"onblur"</Emphasis>"."
                },
                markup! {
                    "Users who navigate via keyboard cannot interact with elements that only have mouse event handlers. Add an "<Emphasis>"onblur"</Emphasis>" attribute to ensure keyboard accessibility."
                },
            ),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                message,
            )
            .note(note),
        )
    }
}
