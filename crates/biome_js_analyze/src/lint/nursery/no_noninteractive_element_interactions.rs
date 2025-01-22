use crate::services::aria::Aria;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow use event handlers on non-interactive elements.
    ///
    /// Non-interactive HTML elements indicate _content_ and _containers_ in the user interface.
    /// Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// A Non-interactive element does not support event handlers(mouse and key handlers).
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}}>button</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <button onClick={() => { }}>button</button>
    /// ```
    ///
    /// ```jsx
    /// // Adding a role to element does not add behavior.
    /// // If not used semantic HTML elements like `button`, developers need to implement the expected behavior for role(like focusability and key press support)
    /// // See https://www.w3.org/WAI/ARIA/apg/
    /// <div role="button" onClick={() => { }}>button</div>
    /// ```
    ///
    /// ```jsx
    /// // The role="presentation" attribute removes the semantic meaning of an element, indicating that it should be ignored by assistive technologies.
    /// // Therefore, it's acceptable to add event handlers to elements with role="presentation" for visual effects or other purposes,
    /// // but users relying on assistive technologies may not be able to interact with these elements.
    /// <div role="presentation" onClick={() => { }}>button</div>
    /// ```
    ///
    /// ```jsx
    /// // Hidden from screen reader.
    /// <div onClick={() => {}} aria-hidden />
    /// ```
    ///
    /// ```jsx
    /// // Custom component is not checked.
    /// <SomeComponent onClick={() => {}}>button</SomeComponent>
    /// ```
    ///
    /// ```jsx
    /// // Spread attributes is not supported.
    /// <div {...{"onClick":() => {}}}>button</div>
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
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-noninteractive-element-interactions")],
        recommended: false,
    }
}

impl Rule for NoNoninteractiveElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Custom components are not checked because we do not know what DOM will be used.
        if element.is_custom_component() {
            return None;
        }

        let aria_roles = ctx.aria_roles();
        let role = aria_roles.get_role_by_element_name(element);
        let has_interactive_role = role.is_some_and(|role| role.is_interactive());

        if !has_handler_props(element)
            || is_content_editable(element)
            || has_presentation_role(element)
            || is_hidden_from_screen_reader(element)?
            || has_interactive_role
        {
            return None;
        }

        // Non-interactive elements what contains event handler should be reported.
        if has_handler_props(element) && aria_roles.is_not_interactive_element(element) {
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
                    "Non-interactive element should not have event handler."
                },
            )
            .note(markup! {
                "Consider replace semantically interactive element like "<Emphasis>"<button/>"</Emphasis>" or "<Emphasis>"<a href/>"</Emphasis>"."
            })
        )
    }
}

/// Ref: https://github.com/jsx-eslint/jsx-ast-utils/blob/v3.3.5/src/eventHandlers.js
const INTERACTIVE_HANDLERS: &[&str] = &[
    "onClick",
    "onContextMenu",
    "onDblClick",
    "onDoubleClick",
    "onDrag",
    "onDragEnd",
    "onDragEnter",
    "onDragExit",
    "onDragLeave",
    "onDragOver",
    "onDragStart",
    "onDrop",
    "onMouseDown",
    "onMouseEnter",
    "onMouseLeave",
    "onMouseMove",
    "onMouseOut",
    "onMouseOver",
    "onKeyDown",
    "onKeyPress",
    "onKeyUp",
    "onFocus",
    "onBlur",
    "onLoad",
    "onError",
];

/// Check if the element contains event handler
fn has_handler_props(element: &AnyJsxElement) -> bool {
    INTERACTIVE_HANDLERS
        .iter()
        .any(|handler| element.find_attribute_by_name(handler).is_some())
}

/// Check if the element's implicit ARIA semantics have been removed.
/// See https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/presentation_role
///
/// Ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isPresentationRole.js
fn has_presentation_role(element: &AnyJsxElement) -> bool {
    if let Some(attribute) = element.find_attribute_by_name("role") {
        let value = attribute.as_static_value();
        if let Some(value) = value {
            return matches!(value.as_string_constant(), Some("presentation" | "none"));
        }
    }
    false
}

/// Check the element is hidden from screen reader.
/// See
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
///
/// Ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
fn is_hidden_from_screen_reader(element_name: &AnyJsxElement) -> Option<bool> {
    let is_aria_hidden = element_name.has_truthy_attribute("aria-hidden");

    let name = element_name.name_value_token().ok()?;

    let is_input_hidden = if name.text_trimmed() == "input" {
        element_name
            .find_attribute_by_name("type")
            .and_then(|attribute| attribute.as_static_value())
            .and_then(|value| value.as_string_constant().map(|value| value == "hidden"))
            .unwrap_or_default()
    } else {
        false
    };

    Some(is_aria_hidden || is_input_hidden)
}

/// Check if the element is `contentEditable`
/// See https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable
///
/// Ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isContentEditable.js
fn is_content_editable(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("contentEditable")
        .and_then(|attribute| attribute.as_static_value())
        .and_then(|value| value.as_string_constant().map(|value| value == "true"))
        .unwrap_or_default()
}
