use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::AstNode;
use biome_rule_options::no_noninteractive_element_interactions::NoNoninteractiveElementInteractionsOptions;

use crate::{
    Aria,
    a11y::{has_event_handler, is_content_editable, is_hidden_from_screen_reader},
};

declare_lint_rule! {
    /// Disallow use event handlers on non-interactive elements.
    ///
    /// Non-interactive HTML elements indicate _content_ and _containers_ in the user interface.
    /// Non-interactive elements include `<main>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// A Non-interactive element does not support event handlers(mouse and key handlers).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div onclick="() => {}">button</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <button onclick="() => { }">button</button>
    /// ```
    ///
    /// ```html
    /// // Adding a role to element does not add behavior.
    /// // If not used semantic HTML elements like `button`, developers need to implement the expected behavior for role(like focusability and key press support)
    /// // See https://www.w3.org/WAI/ARIA/apg/
    /// <div role="button" onclick="() => { }">button</div>
    /// ```
    ///
    /// ```html
    /// // The role="presentation" attribute removes the semantic meaning of an element, indicating that it should be ignored by assistive technologies.
    /// // Therefore, it's acceptable to add event handlers to elements with role="presentation" for visual effects or other purposes,
    /// // but users relying on assistive technologies may not be able to interact with these elements.
    /// <div role="presentation" onclick="() => { }">button</div>
    /// ```
    ///
    /// ```html
    /// // Hidden from screen reader.
    /// <div onclick="() => {}" aria-hidden="true" />
    /// ```
    ///
    /// ```svelte
    /// // Custom component is not checked.
    /// <SomeComponent onClick={() => {}}>button</SomeComponent>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ### Resources
    ///
    /// - [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro)
    /// - [WAI-ARIA Authoring Practices Guide - Design Patterns and Widgets](https://www.w3.org/TR/wai-aria-practices-1.1/#aria_ex)
    /// - [Fundamental Keyboard Navigation Conventions](https://www.w3.org/TR/wai-aria-practices-1.1/#kbd_generalnav)
    /// - [Mozilla Developer Network - ARIA Techniques](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_button_role#Keyboard_and_focus)
    ///
    pub NoNoninteractiveElementInteractions {
        version: "next",
        name: "noNoninteractiveElementInteractions",
        language: "html",
        recommended: false,
    }
}

impl Rule for NoNoninteractiveElementInteractions {
    type Query = Aria<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNoninteractiveElementInteractionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Custom components are not checked because we do not know what DOM will be used.
        if element.is_custom_component() {
            return None;
        }

        let aria_roles = ctx.aria_roles();
        let role = aria_roles.get_role_by_element_name(element);
        let has_interactive_role = role.is_some_and(|role| role.is_interactive());

        if !has_event_handler(EVENT_HANDLER_TYPES, element)
            || is_content_editable(element)
            || aria_roles.is_presentation_role(element)
            || is_hidden_from_screen_reader(element)
            || has_interactive_role
        {
            return None;
        }

        // Non-interactive elements what contains event handler should be reported.
        if aria_roles.is_not_interactive_element(element) {
            return Some(());
        };

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected event handler on non-interactive element."
                },
            )
            .note(markup! {
                "Non-interactive element should not have event handler. Consider replace semantically interactive element like "<Emphasis>"<button/>"</Emphasis>" or "<Emphasis>"<a href/>"</Emphasis>"."
            })
        )
    }
}

// Only check the focus, image, keyboard and mouse event handler types.
const EVENT_HANDLER_TYPES: &[&str] = &["focus", "image", "keyboard", "mouse"];

#[test]
fn test_order() {
    assert!(EVENT_HANDLER_TYPES.is_sorted());
}
