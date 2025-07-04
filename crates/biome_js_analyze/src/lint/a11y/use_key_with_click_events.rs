use crate::{a11y::is_hidden_from_screen_reader, services::aria::Aria};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_key_with_click_events::UseKeyWithClickEventsOptions;

declare_lint_rule! {
    /// Enforce onClick is accompanied by at least one of the following: `onKeyUp`, `onKeyDown`, `onKeyPress`.
    ///
    /// Coding for the keyboard is important for users with physical disabilities who cannot use a mouse, AT compatibility, and screenreader users.
    /// This does not apply for interactive or hidden elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyDown={handleKeyDown} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyUp={handleKeyUp} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyPress={handleKeyPress} />
    ///```
    ///
    /// ```jsx
    /// // this rule doesn't apply to user created component
    /// <MyComponent onClick={() => {}} />
    ///```
    ///
    /// ```jsx
    /// <button onClick={() => console.log("test")}>Submit</button>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub UseKeyWithClickEvents {
        version: "1.0.0",
        name: "useKeyWithClickEvents",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("click-events-have-key-events").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseKeyWithClickEvents {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseKeyWithClickEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let aria_roles = ctx.aria_roles();

        element.find_attribute_by_name("onClick")?;

        // skip custom components for now
        if element.is_custom_component() {
            return None;
        }

        if is_hidden_from_screen_reader(element) || aria_roles.is_presentation_role(element) {
            return None;
        }

        if !aria_roles.is_not_interactive_element(element) {
            return None;
        }

        if element.find_attribute_by_name("onKeyDown").is_some()
            || element.find_attribute_by_name("onKeyUp").is_some()
            || element.find_attribute_by_name("onKeyPress").is_some()
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Enforce to have the "<Emphasis>"onClick"</Emphasis>" mouse event with the "<Emphasis>"onKeyUp"</Emphasis>", the "<Emphasis>"onKeyDown"</Emphasis>", or the "<Emphasis>"onKeyPress"</Emphasis>" keyboard event."
            },
        ).note(markup! {
            "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation."
        }))
    }
}
