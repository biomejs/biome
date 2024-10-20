use crate::services::aria::Aria;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

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
    /// <button onClick={() => {}}>button</button>
    /// <div role="button" onClick={() => {}}>button</div>
    /// <div role="presentation" onClick={() => {}}>button</div>
    /// ```
    /// 
    /// ```jsx
    /// // Hidden from screen reader.
    /// <div onClick={() => void 0} role="button" aria-hidden />
    /// ```
    /// 
    /// ```jsx
    /// // Custom component is not checked.
    /// <SomeComponent onClick={()=>{}}>button</SomeComponent>
    /// ```
    /// 
    /// ```jsx
    /// // Spread attributes is not supported.
    /// <div {...{"onClick":()=>{}}}>button</div>
    /// ```
    ///
    pub NoNoninteractiveElementInteractions {
        version: "next",
        name: "noNoninteractiveElementInteractions",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-noninteractive-element-interactions")],
        recommended: false,
    }
}

/// Copy from https://github.com/jsx-eslint/jsx-ast-utils/blob/v3.3.5/src/eventHandlers.js
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
    "onMouseUp",
];

type AttributesRef<'a> = Option<&'a FxHashMap<String, Vec<String>>>;

/// Check if the element contains event handler
fn has_handler_props(attributes: AttributesRef) -> bool {
    attributes.is_some_and(|m| {
        INTERACTIVE_HANDLERS
            .iter()
            .any(|handler| m.contains_key(*handler))
    })
}

/// Check if the element's implicit ARIA semantics have been removed.
/// See https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/presentation_role
///
/// Copy from https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isPresentationRole.js
fn has_presentation_role(attributes: AttributesRef) -> bool {
    const PRESENTATION_ROLE: &[&str] = &["presentation", "none"];

    if let Some(attributes) = attributes {
        if let Some(values) = attributes.get("role") {
            let is_presentation = values
                .iter()
                .any(|v| PRESENTATION_ROLE.contains(&v.as_str()));
            return is_presentation;
        }
    }
    false
}

/// Check the element is hidden from screen reader.
/// See
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
///
/// Copy from https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
fn is_hidden_from_screen_reader(element_name: &str, attributes: AttributesRef) -> bool {
    if let Some(attributes) = attributes {
        let aria_hidden = attributes
            .get("aria-hidden")
            .map_or(false, |attr| attr.contains(&"true".to_string()));

        let input_hidden = element_name == "input"
            && attributes
                .get("type")
                .map_or(false, |attr| attr.contains(&"hidden".to_string()));

        return aria_hidden || input_hidden;
    }

    false
}

/// Check if the element is `contentEditable`
/// See https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable
///
/// Copy from https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isContentEditable.js
fn is_content_editable(attributes: AttributesRef) -> bool {
    if let Some(attributes) = attributes {
        if let Some(values) = attributes.get("contentEditable") {
            return values.contains(&"true".to_string());
        }
    }
    false
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

        let attributes = ctx.extract_attributes(&element.attributes());
        let attributes = ctx.convert_all_attribute_values(attributes);
        let attributes_ref = attributes.as_ref();
        let element_name = element.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();

        if !has_handler_props(attributes_ref)
            || is_content_editable(attributes_ref)
            || has_presentation_role(attributes_ref)
            || is_hidden_from_screen_reader(element_name, attributes_ref)
        {
            return None;
        }

        let aria_roles = ctx.aria_roles();

        // Non-interactive elements what contains event handler should be reported.
        if aria_roles.is_not_interactive_element(element_name, attributes) {
            return Some(());
        };

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
