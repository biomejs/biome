use std::{collections::HashMap, hash::BuildHasherDefault};

use crate::aria_services::Aria;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_aria::AriaRoles;
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use rustc_hash::FxHasher;

declare_rule! {
    /// Static elements should not be interactive.
    ///
    /// Static HTML elements do not have semantic meaning. This is clear in the case of `<div>` and `<span>`. It is less so clear in the case of elements that seem semantic, but that do not have a semantic mapping in the accessibility layer. For example `<a>`, `<big>`, `<blockquote>`, `<footer>`, `<picture>`, `<strike>` and `<time>` -- to name a few -- have no semantic layer mapping. They are as void of meaning as `<div>`.
    ///
    /// The [WAI-ARIA role attribute](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) confers a semantic mapping to an element. The semantic value can then be expressed to a user via assistive technology.
    /// In order to add interactivity such as a mouse or key event listener to a static element, that element must be given a role value as well.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-static-element-interactions.md#jsx-a11yno-static-element-interactions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}} />;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <div role="button" onClick={() => {}}  />;
    /// ```
    ///
    /// ```jsx
    /// <button onClick={() => {}} className="foo" />;
    /// ```
    ///
    /// ```jsx
    /// <input type="text" onClick={() => {}} />;
    /// ```
    ///
    pub(crate) NoStaticElementInteractions {
        version: "next",
        name: "noStaticElementInteractions",
        recommended: false,
    }
}

const INTERACTIVE_HANDLER_LIST: [&str; 24] = [
    "onBlur",
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
    "onFocus",
    "onKeyDown",
    "onKeyPress",
    "onKeyUp",
    "onMouseDown",
    "onMouseEnter",
    "onMouseLeave",
    "onMouseMove",
    "onMouseOut",
    "onMouseOver",
    "onMouseUp",
];

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();

        let attributes = ctx.extract_attributes(&node.attributes())?;
        let element_name = node.name().ok()?.text();

        let is_element = node.is_element();
        let no_aria_hidden = !node.has_truthy_attribute("aria-hidden");
        let no_explicit_role = node.has_truthy_attribute("role");
        let no_implicit_role = aria_roles
            .get_implicit_role(&element_name, &attributes)
            .is_none();
        let has_interaction_handler = INTERACTIVE_HANDLER_LIST
            .iter()
            .any(|handler| node.has_truthy_attribute(handler));

        (is_element
            && no_aria_hidden
            && no_explicit_role
            && no_implicit_role
            && has_interaction_handler)
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Static elements should not be interactive."
                },
            )
            .note(markup! {
                "To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value"
            }),
        )
    }
}

fn is_not_interactive_element(
    node: &AnyJsxElement,
    aria_roles: &AriaRoles,
    attributes: Option<HashMap<String, Vec<String>, BuildHasherDefault<FxHasher>>>,
) -> Option<bool> {
    let maybe_element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
    let element_name: &str = maybe_element_name.text_trimmed();
    let is_not_interactive = aria_roles.is_not_interactive_element(element_name, attributes);

    is_not_interactive.then_some(true)
}

fn has_interactive_handler(node: &AnyJsxElement) -> Option<bool> {
    let has_interactive_handler = INTERACTIVE_HANDLER_LIST
        .iter()
        .any(|handler| node.has_truthy_attribute(handler));

    has_interactive_handler.then_some(true)
}

fn is_presentation_role(node: &AnyJsxElement) -> Option<bool> {
    let role_attribute = node.find_attribute_by_name("role")?;
    let role_attribute_value = role_attribute.as_static_value()?;

    (role_attribute_value.text() == "presentation").then_some(true)
}

fn is_abstract_role(node: &AnyJsxElement) -> Option<bool> {
    let role_attribute = node.find_attribute_by_name("role")?;
    let role_attribute_value = role_attribute.as_static_value()?;
    [
        "command",
        "composite",
        "input",
        "landmark",
        "range",
        "roletype",
        "section",
        "sectionhead",
        "select",
        "structure",
        "widget",
        "window",
    ]
    .contains(&role_attribute_value.text())
    .then_some(true)
}

fn is_interactive_role(node: &AnyJsxElement, aria_roles: &AriaRoles) -> Option<bool> {
    let role_attribute = node.find_attribute_by_name("role")?;
    let role_attribute_value = role_attribute.as_static_value()?;
    let is_role_interactive = aria_roles.is_role_interactive(role_attribute_value.text());

    (is_role_interactive).then_some(true)
}

fn is_hidden_from_screen_reader(node: &AnyJsxElement) -> Option<bool> {
    let maybe_element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
    let element_name: &str = maybe_element_name.text_trimmed();

    if element_name == "input" {
        let type_attribute = node.find_attribute_by_name("type")?;
        let type_attr_value = type_attribute.as_static_value()?;
        let is_hidden_input = element_name == "input" && type_attr_value.text() == "hidden";

        is_hidden_input.then_some(true)
    } else {
        node.has_truthy_attribute("aria-hidden").then_some(true)
    }
}
