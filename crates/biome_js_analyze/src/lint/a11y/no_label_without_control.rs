use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, AnyJsxElementName, AnyJsxTag,
    JsSyntaxKind, JsxAttribute,
};
use biome_rowan::{AstNode, WalkEvent};
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Enforce that a label element or component has a text label and an associated input.
    ///
    /// An "input" is considered one of the following elements: `input`, `meter`, `output`, `progress`, `select`, `textarea` or `button`.
    ///
    /// There are two supported ways to associate a label with an input:
    /// - Wrapping an input in a label element.
    /// - Adding a `for` attribute (or `htmlFor` in React) to a label and assigning it a DOM ID string associated with an input on the page.
    ///
    ///
    /// This rule checks that any `label` element (or an indicated custom component that will output a `label` element) meets one of these conditions:
    /// - Wraps an `input` element (or an indicated custom component that will output an `input` element)
    /// - Has a `for` or `htmlFor` attribute and that the `label` element/component has accessible text content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <label for="js_id" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label for="js_id"><input /></label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label htmlFor="js_id" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label htmlFor="js_id"><input /></label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label>A label</label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div><label /><input /></div>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <label for="js_id" aria-label="A label" />;
    /// <label for="js_id" aria-labelledby="A label" />;
    /// <label htmlFor="js_id" aria-label="A label" />;
    /// <label htmlFor="js_id" aria-labelledby="A label" />;
    /// <label>A label<input /></label>;
    /// <label>A label<textarea /></label>;
    /// <label><img alt="A label" /><input /></label>;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    /// - `inputComponents` - An array of component names that should be considered the same as an `input` element.
    /// - `labelAttributes` - An array of attributes that should be treated as the `label` accessible text content.
    /// - `labelComponents` - An array of component names that should be considered the same as a `label` element.
    ///
    /// Both options `inputComponents` and `labelComponents` don't have support for namespace components (e.g. `<Control.Input>`).
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "inputComponents": ["CustomInput"],
    ///         "labelAttributes": ["label"],
    ///         "labelComponents": ["CustomLabel"]
    ///     }
    /// }
    /// ```
    ///
    pub NoLabelWithoutControl {
        version: "1.8.0",
        name: "noLabelWithoutControl",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("label-has-associated-control")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoLabelWithoutControl {
    type Query = Ast<AnyJsxTag>;
    type State = NoLabelWithoutControlState;
    type Signals = Option<Self::State>;
    type Options = NoLabelWithoutControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let element_name = node.name()?.name_value_token().ok()?;
        let element_name = element_name.text_trimmed();
        let is_allowed_element = options.has_element_name(element_name)
            || DEFAULT_LABEL_COMPONENTS.contains(&element_name);

        if !is_allowed_element {
            return None;
        }

        let has_text_content = options.has_accessible_label(node);
        let has_control_association = has_for_attribute(node) || options.has_nested_control(node);

        if has_text_content && has_control_association {
            return None;
        }

        Some(NoLabelWithoutControlState {
            has_text_content,
            has_control_association,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "A form label must be associated with an input."
            },
        );

        if !state.has_text_content {
            diagnostic = diagnostic.note(
                markup! { "Consider adding an accessible text content to the label element." },
            );
        }

        if !state.has_control_association {
            diagnostic = diagnostic.note(
                markup! { "Consider adding a `for` or `htmlFor` attribute to the label element or moving the input element to inside the label element." },
            );
        }

        Some(diagnostic)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoLabelWithoutControlOptions {
    /// Array of component names that should be considered the same as an `input` element.
    pub input_components: Box<[Box<str>]>,
    /// Array of attributes that should be treated as the `label` accessible text content.
    pub label_attributes: Box<[Box<str>]>,
    /// Array of component names that should be considered the same as a `label` element.
    pub label_components: Box<[Box<str>]>,
}

impl NoLabelWithoutControlOptions {
    /// Returns `true` whether the passed `attribute` meets one of the following conditions:
    /// - Has a label attribute that corresponds to the `label_attributes` parameter
    /// - Has a label among `DEFAULT_LABEL_ATTRIBUTES`
    fn has_label_attribute(&self, attribute: &JsxAttribute) -> bool {
        let Ok(attribute_name) = attribute.name().and_then(|name| name.name_token()) else {
            return false;
        };
        let attribute_name = attribute_name.text_trimmed();
        if !DEFAULT_LABEL_ATTRIBUTES.contains(&attribute_name)
            && !self
                .label_attributes
                .iter()
                .any(|name| name.as_ref() == attribute_name)
        {
            return false;
        }
        attribute
            .initializer()
            .and_then(|init| init.value().ok())
            .is_some_and(|v| has_label_attribute_value(&v))
    }

    /// Returns `true` whether the passed `jsx_tag` meets one of the following conditions:
    /// - Has a label attribute that corresponds to the `label_attributes` parameter
    /// - Has a label among `DEFAULT_LABEL_ATTRIBUTES`
    /// - Has a child that acts as a label
    fn has_accessible_label(&self, jsx_tag: &AnyJsxTag) -> bool {
        let mut child_iter = jsx_tag.syntax().preorder();
        while let Some(event) = child_iter.next() {
            match event {
                WalkEvent::Enter(child) => match child.kind() {
                    JsSyntaxKind::JSX_EXPRESSION_CHILD
                    | JsSyntaxKind::JSX_SPREAD_CHILD
                    | JsSyntaxKind::JSX_TEXT => {
                        return true;
                    }
                    JsSyntaxKind::JSX_ELEMENT
                    | JsSyntaxKind::JSX_OPENING_ELEMENT
                    | JsSyntaxKind::JSX_CHILD_LIST
                    | JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
                    | JsSyntaxKind::JSX_ATTRIBUTE_LIST => {}
                    JsSyntaxKind::JSX_ATTRIBUTE => {
                        let attribute = JsxAttribute::unwrap_cast(child);
                        if self.has_label_attribute(&attribute) {
                            return true;
                        }
                        child_iter.skip_subtree();
                    }
                    _ => {
                        child_iter.skip_subtree();
                    }
                },
                WalkEvent::Leave(_) => {}
            }
        }
        false
    }

    /// Returns whether the passed `AnyJsxTag` have a child that is considered an input component
    /// according to the passed `input_components` parameter
    fn has_nested_control(&self, jsx_tag: &AnyJsxTag) -> bool {
        let mut child_iter = jsx_tag.syntax().preorder();
        while let Some(event) = child_iter.next() {
            match event {
                WalkEvent::Enter(child) => match child.kind() {
                    JsSyntaxKind::JSX_ELEMENT
                    | JsSyntaxKind::JSX_OPENING_ELEMENT
                    | JsSyntaxKind::JSX_CHILD_LIST
                    | JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {}
                    _ => {
                        let Some(element_name) = AnyJsxElementName::cast(child) else {
                            child_iter.skip_subtree();
                            continue;
                        };
                        let Ok(element_name) = element_name.name_value_token() else {
                            continue;
                        };
                        let element_name = element_name.text_trimmed();
                        if DEFAULT_INPUT_COMPONENTS.contains(&element_name)
                            || self
                                .input_components
                                .iter()
                                .any(|name| name.as_ref() == element_name)
                        {
                            return true;
                        }
                    }
                },
                WalkEvent::Leave(_) => {}
            }
        }
        false
    }

    fn has_element_name(&self, element_name: &str) -> bool {
        self.label_components
            .iter()
            .any(|label_component_name| label_component_name.as_ref() == element_name)
    }
}

pub struct NoLabelWithoutControlState {
    pub has_text_content: bool,
    pub has_control_association: bool,
}

const DEFAULT_LABEL_ATTRIBUTES: [&str; 3] = ["aria-label", "aria-labelledby", "alt"];
const DEFAULT_LABEL_COMPONENTS: [&str; 1] = ["label"];
const DEFAULT_INPUT_COMPONENTS: [&str; 7] = [
    "input", "meter", "output", "progress", "select", "textarea", "button",
];

/// Returns whether the passed `AnyJsxTag` have a `for` or `htmlFor` attribute
fn has_for_attribute(jsx_tag: &AnyJsxTag) -> bool {
    let for_attributes = ["for", "htmlFor"];
    let Some(attributes) = jsx_tag.attributes() else {
        return false;
    };
    attributes.into_iter().any(|attribute| match attribute {
        AnyJsxAttribute::JsxAttribute(jsx_attribute) => jsx_attribute
            .name()
            .ok()
            .and_then(|jsx_name| {
                if let AnyJsxAttributeName::JsxName(jsx_name) = jsx_name {
                    jsx_name.value_token().ok()
                } else {
                    None
                }
            })
            .is_some_and(|jsx_name| for_attributes.contains(&jsx_name.text_trimmed())),
        AnyJsxAttribute::JsxSpreadAttribute(_) => false,
    })
}

/// Returns whether the passed `jsx_attribute_value` has a valid value inside it
fn has_label_attribute_value(jsx_attribute_value: &AnyJsxAttributeValue) -> bool {
    match jsx_attribute_value {
        AnyJsxAttributeValue::AnyJsxTag(_) => false,
        AnyJsxAttributeValue::JsxExpressionAttributeValue(_) => true,
        AnyJsxAttributeValue::JsxString(jsx_string) => !jsx_string
            .inner_string_text()
            .is_ok_and(|text| text.text().trim().is_empty()),
    }
}
